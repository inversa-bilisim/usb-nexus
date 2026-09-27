// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

// USB Nexus desktop UI. Talks to the service through two Tauri commands:
//   api({request})   -> forwards a request to the local service API
//   ui_strings({lang}) -> translated message templates
//
// All text coming from the service or remote computers is inserted with
// textContent, never as HTML.

"use strict";

const invoke = (cmd, args) => window.__TAURI__.core.invoke(cmd, args);

const state = {
  os: "linux",
  lang: null,
  languages: [],
  messages: {},
  view: "this",
  remote: null, // { fingerprint, name } when browsing a server's devices
  status: null,
  attachmentsCount: 0,
  serviceDown: false,
  pollTimer: null,
  policyAsked: false,
};

// ---------------------------------------------------------------- i18n

function t(id, args = {}) {
  const tpl = state.messages[id];
  if (tpl === undefined) return id;
  return tpl.replace(/\{(\w+)\}/g, (_, k) => (k in args ? String(args[k]) : ""));
}

async function loadStrings(lang) {
  const s = await invoke("ui_strings", { lang: lang || null });
  state.os = s.os;
  state.lang = s.lang;
  state.languages = s.languages;
  state.messages = s.messages;
  document.documentElement.lang = s.lang;
}

function errorText(err) {
  const code = (err && err.code) || "other";
  if (code === "pairing_required") return t("err-not-trusted");
  const key = "err-" + code.replace(/_/g, "-");
  if (code !== "other" && key in state.messages) return t(key);
  return t("err-other", { detail: (err && err.message) || String(err) });
}

// ---------------------------------------------------------------- DOM helpers

function h(tag, attrs = {}, ...children) {
  const el = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) {
    if (v === undefined || v === null || v === false) continue;
    if (k === "class") el.className = v;
    else if (k.startsWith("on")) el.addEventListener(k.slice(2), v);
    else if (v === true) el.setAttribute(k, "");
    else el.setAttribute(k, v);
  }
  for (const c of children.flat()) {
    if (c === null || c === undefined || c === false) continue;
    el.append(c instanceof Node ? c : document.createTextNode(String(c)));
  }
  return el;
}

const ICONS = {
  computer: "M4 5h16v10H4zM2 19h20M9 15v4M15 15v4",
  network: "M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18zM3 12h18M12 3c2.5 2.7 3.8 5.7 3.8 9s-1.3 6.3-3.8 9c-2.5-2.7-3.8-5.7-3.8-9S9.5 5.7 12 3z",
  plug: "M9 3v5M15 3v5M6 8h12v3a6 6 0 0 1-12 0zM12 17v4",
  link: "M10 14a4 4 0 0 0 5.7 0l3-3a4 4 0 0 0-5.7-5.7l-1 1M14 10a4 4 0 0 0-5.7 0l-3 3a4 4 0 0 0 5.7 5.7l1-1",
  usb: "M12 2v14M12 2l-2.5 3h5zM8 9v3l4 3M16 7v4l-4 3M6.5 7h3v2h-3zM16 5.5a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zM12 16a2 2 0 1 0 0 4 2 2 0 0 0 0-4z",
  refresh: "M20 11a8 8 0 0 0-14.9-3M4 4v4h4M4 13a8 8 0 0 0 14.9 3M20 20v-4h-4",
  back: "M15 18l-6-6 6-6",
  plus: "M12 5v14M5 12h14",
  chevron: "M9 6l6 6-6 6",
  clock: "M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18zM12 7v5l3 2",
  settings: "M4 6h9M17 6h3M4 12h3M11 12h9M4 18h11M19 18h1M15 4v4M9 10v4M17 16v4",
  shield: "M12 3l7 3v5c0 4.5-3 8.3-7 10-4-1.7-7-5.5-7-10V6z",
  download: "M12 4v11M7 10l5 5 5-5M5 20h14",
};

function icon(name, cls) {
  const ns = "http://www.w3.org/2000/svg";
  const svg = document.createElementNS(ns, "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("fill", "none");
  svg.setAttribute("stroke", "currentColor");
  svg.setAttribute("stroke-width", "1.8");
  svg.setAttribute("stroke-linecap", "round");
  svg.setAttribute("stroke-linejoin", "round");
  svg.setAttribute("aria-hidden", "true");
  if (cls) svg.setAttribute("class", cls);
  const p = document.createElementNS(ns, "path");
  p.setAttribute("d", ICONS[name]);
  svg.append(p);
  return svg;
}

const hex4 = (v) => (v === null || v === undefined ? "????" : v.toString(16).padStart(4, "0"));
const shortFp = (fp) => (fp || "").slice(0, 16).match(/.{1,4}/g)?.join("-") || "";

function deviceName(d) {
  const name = [d.manufacturer, d.product].filter(Boolean).join(" ");
  return name || t("gui-unnamed-device");
}

// Remembers remote device names so "Connected devices" can show them.
const names = {
  get(server, device) {
    try {
      return JSON.parse(localStorage.getItem("names") || "{}")[server + "/" + device];
    } catch {
      return undefined;
    }
  },
  put(server, device, name) {
    try {
      const all = JSON.parse(localStorage.getItem("names") || "{}");
      all[server + "/" + device] = name;
      localStorage.setItem("names", JSON.stringify(all));
    } catch {
      /* storage unavailable: names are a convenience only */
    }
  },
};

function toast(text, kind) {
  const el = h("div", { class: "toast" + (kind === "error" ? " error" : "") }, text);
  document.getElementById("toasts").append(el);
  setTimeout(() => el.remove(), kind === "error" ? 7000 : 4000);
}

// ---------------------------------------------------------------- API

class ServiceDown extends Error {}

async function api(cmd, args = {}) {
  try {
    const data = await invoke("api", { request: { cmd, ...args } });
    if (state.serviceDown) {
      state.serviceDown = false;
      render();
    }
    return data;
  } catch (err) {
    if (err && (err.code === "service_unavailable" || err.code === "permission_denied")) {
      if (!state.serviceDown || state.serviceDownReason !== err.code) {
        state.serviceDown = true;
        state.serviceDownReason = err.code;
        render();
      }
      throw new ServiceDown(err.message);
    }
    throw err;
  }
}

/** Runs an action, reporting failures as a toast. */
async function act(fn) {
  try {
    await fn();
  } catch (err) {
    if (!(err instanceof ServiceDown)) toast(errorText(err), "error");
  }
}

// ---------------------------------------------------------------- modal

function openModal(content, { onClose, wide = false } = {}) {
  const backdrop = document.getElementById("modal");
  const box = h("div", { class: "modal" + (wide ? " wide" : ""), role: "dialog", "aria-modal": "true" }, content);
  backdrop.replaceChildren(box);
  backdrop.hidden = false;
  const close = () => {
    backdrop.hidden = true;
    backdrop.replaceChildren();
    document.removeEventListener("keydown", onKey);
    onClose && onClose();
  };
  const onKey = (e) => e.key === "Escape" && close();
  document.addEventListener("keydown", onKey);
  // Returning false from an onclick property would cancel clicks inside the
  // dialog (checkboxes, submit buttons), so nothing is returned.
  backdrop.onclick = (e) => {
    if (e.target === backdrop) close();
  };
  setTimeout(() => (box.querySelector("input, .btn.primary, .btn") || box).focus(), 0);
  return close;
}

// ---------------------------------------------------------------- sidebar

const VIEWS = [
  ["this", "computer", "gui-nav-this-computer"],
  ["network", "network", "gui-nav-network"],
  ["connected", "plug", "gui-nav-connected"],
  ["paired", "link", "gui-nav-paired"],
  ["history", "clock", "gui-nav-history"],
  ["settings", "settings", "gui-nav-settings"],
];

/** What this computer is set up for (both until the service says). */
function roles() {
  return (state.status && state.status.roles) || { server: true, client: true };
}

/** Views of the roles this computer has; the others are not shown at all. */
function visibleViews() {
  const r = roles();
  return VIEWS.filter(([id]) => (id !== "this" || r.server) && ((id !== "network" && id !== "connected") || r.client));
}

function renderSidebar() {
  const nav = document.getElementById("nav");
  nav.setAttribute("aria-label", t("gui-nav-this-computer"));
  nav.replaceChildren(
    ...visibleViews().map(([id, ic, key]) =>
      h(
        "button",
        {
          class: "nav-item",
          "aria-current": state.view === id ? "page" : null,
          onclick: () => go(id),
        },
        icon(ic),
        h("span", {}, t(key)),
        id === "connected" && state.attachmentsCount > 0 ? h("span", { class: "nav-count" }, state.attachmentsCount) : null,
      ),
    ),
  );

  const me = document.getElementById("me");
  me.replaceChildren(
    ...(state.status
      ? [
          h("div", { class: "me-name", title: t("gui-nav-this-computer") }, state.status.name),
          h("div", { class: "me-fp", title: t("gui-fingerprint") }, shortFp(state.status.fingerprint)),
        ]
      : []),
  );

  document.getElementById("lang-label").textContent = t("gui-language");
  const sel = document.getElementById("lang");
  sel.replaceChildren(...state.languages.map(([code, name]) => h("option", { value: code, selected: code === state.lang }, name)));
  sel.onchange = async () => {
    try {
      localStorage.setItem("lang", sel.value);
    } catch {
      /* not persisted */
    }
    await loadStrings(sel.value);
    render();
  };
}

function go(view) {
  state.view = view;
  state.remote = null;
  render();
  document.getElementById("main").focus();
}

// ---------------------------------------------------------------- pages

function pageHead(title, subtitle, ...actions) {
  return h(
    "div",
    { class: "page-head" },
    h("div", {}, h("h1", {}, title), subtitle ? h("p", {}, subtitle) : null),
    actions.length ? h("div", { class: "actions" }, actions) : null,
  );
}

function list(rows, emptyText) {
  return h("div", { class: "list" }, rows.length ? rows : h("div", { class: "empty" }, emptyText));
}

function deviceMeta(d, extra = []) {
  return h(
    "div",
    { class: "row-meta" },
    d.busid ? h("span", { class: "mono" }, d.busid) : null,
    h("span", { class: "mono" }, `${hex4(d.vendor_id)}:${hex4(d.product_id)}`),
    d.speed ? h("span", {}, d.speed) : null,
    d.by_port ? h("span", { title: t("gui-tracked-by-port-hint") }, t("gui-tracked-by-port")) : null,
    ...extra,
  );
}

// Short description of who may use a shared device.
function accessLabel(d) {
  if (d.open_to_all) return t("gui-access-everyone");
  const n = (d.access && d.access.allowed && d.access.allowed.length) || 0;
  return n ? t("gui-access-some", { count: n }) : t("gui-access-nobody");
}

// This computer: local devices and sharing.
async function pageThis() {
  const devices = await api("local_devices");
  const rows = devices.map((d) => {
    names.put("local", d.id, deviceName(d));
    const toggle = h("button", {
      class: "switch",
      role: "switch",
      "aria-checked": String(d.shared),
      "aria-label": t("gui-share") + " " + deviceName(d),
      title: t("gui-share"),
      onclick: () =>
        act(async () => {
          toggle.disabled = true;
          await api("set_shared", { device: d.id, shared: !d.shared });
          await refresh();
          if (!d.shared) await askWhoMayUse(d.id);
        }),
    });
    toggle.addEventListener("click", (e) => e.stopPropagation());
    let badge;
    if (d.used_by) {
      const waiting = (d.queue || []).length;
      const text = waiting
        ? t("gui-used-by-waiting", { name: d.used_by, count: waiting })
        : t("gui-used-by", { name: d.used_by });
      badge = h("span", { class: "badge accent" }, text);
    } else if (!d.present) badge = h("span", { class: "badge warn" }, t("gui-not-plugged-in"));
    else if (d.shared) badge = h("span", { class: "badge ok" }, t("gui-shared"));
    else badge = h("span", { class: "badge" }, t("gui-not-shared"));
    // Shared devices open their details (use, queue, handover, access).
    const access = d.shared ? h("span", { class: "muted small-text" }, icon("shield"), " ", accessLabel(d)) : null;
    const open = () => act(() => deviceDialog(d.id));
    return h(
      "div",
      {
        class: "row" + (d.present ? "" : " absent") + (d.shared ? " clickable" : ""),
        role: d.shared ? "button" : null,
        tabindex: d.shared ? "0" : null,
        onclick: d.shared ? open : null,
        onkeydown: d.shared ? (e) => e.key === "Enter" && open() : null,
      },
      h("div", { class: "tile" + (d.shared && d.present ? " on" : "") }, icon("usb")),
      h("div", { class: "row-body" }, h("div", { class: "row-title" }, deviceName(d)), deviceMeta(d)),
      h("div", { class: "row-end" }, access, badge, toggle, d.shared ? icon("chevron") : null),
    );
  });
  return [
    pageHead(
      t("gui-this-title"),
      t("gui-this-subtitle"),
      h("button", { class: "btn primary", onclick: () => act(showPin) }, icon("plus"), t("gui-pair-new")),
    ),
    list(rows, t("gui-no-local-devices")),
  ];
}

// Pairing PIN shown on this computer.
async function showPin() {
  let view = await api("open_pairing", { seconds: 300 });
  const total = view.remaining_secs;
  const clientsBefore = new Set((await api("peers")).clients.map((p) => p.fingerprint));
  const pinEl = h("div", { class: "pin", "aria-live": "polite" }, view.pin.replace(/(\d{3})(\d{3})/, "$1 $2"));
  const bar = h("div");
  bar.style.width = "100%";
  const label = h("div", { class: "meter-label" });
  let timer;
  const stop = async () => {
    clearInterval(timer);
    await api("close_pairing").catch(() => {});
  };
  const close = openModal(
    [
      h("h2", {}, t("gui-pin-title")),
      h("p", {}, t("gui-pin-body")),
      pinEl,
      h("div", { class: "meter" }, bar),
      label,
      h("div", { class: "modal-actions" }, h("button", { class: "btn", onclick: () => close() }, t("gui-pin-stop"))),
    ],
    { onClose: stop },
  );
  const tick = async () => {
    const s = await api("status").catch(() => null);
    if (!s) return;
    if (s.pairing) {
      bar.style.width = `${(100 * s.pairing.remaining_secs) / total}%`;
      label.textContent = t("gui-pin-remaining", { seconds: s.pairing.remaining_secs });
      return;
    }
    clearInterval(timer);
    // The window closes after a successful pairing; find who joined.
    const peers = await api("peers").catch(() => ({ clients: [] }));
    const added = peers.clients.find((p) => !clientsBefore.has(p.fingerprint));
    if (added) {
      close();
      toast(t("gui-pair-done", { name: added.name }));
      if (s.policy === "restricted") await act(() => clientDevicesDialog(added, { afterPairing: true }));
      refresh();
    } else {
      pinEl.textContent = "— — —";
      label.textContent = t("gui-pin-expired");
    }
  };
  label.textContent = t("gui-pin-remaining", { seconds: total });
  timer = setInterval(tick, 1000);
}

// Network: discovered and paired servers.
let lastDiscovery = null;

async function pageNetwork() {
  if (state.remote) return pageRemote();
  const peers = await api("peers");
  const busy = h("span", { class: "badge busy" }, t("gui-searching"));
  const listEl = h("div");
  const draw = (found) => {
    const byFp = new Map();
    for (const d of found || []) byFp.set(d.fingerprint, d);
    for (const p of peers.servers) {
      if (!byFp.has(p.fingerprint)) {
        byFp.set(p.fingerprint, { name: p.name, fingerprint: p.fingerprint, address: p.last_addr, paired: true });
      }
    }
    const rows = [...byFp.values()]
      .sort((a, b) => Number(b.paired) - Number(a.paired) || a.name.localeCompare(b.name))
      .map((d) => {
        const open = () => {
          state.remote = { fingerprint: d.fingerprint, name: d.name };
          render();
        };
        return h(
          "div",
          {
            class: "row" + (d.paired ? " clickable" : ""),
            onclick: d.paired ? open : null,
            tabindex: d.paired ? "0" : null,
            onkeydown: d.paired ? (e) => e.key === "Enter" && open() : null,
          },
          h("div", { class: "tile" + (d.paired ? " on" : "") }, icon("computer")),
          h(
            "div",
            { class: "row-body" },
            h("div", { class: "row-title" }, d.name),
            h(
              "div",
              { class: "row-meta" },
              d.address ? h("span", { class: "mono" }, d.address) : null,
              h("span", { class: "mono", title: t("gui-fingerprint") }, shortFp(d.fingerprint)),
            ),
          ),
          h(
            "div",
            { class: "row-end" },
            d.paired
              ? [h("span", { class: "badge ok" }, t("gui-paired")), icon("chevron")]
              : h(
                  "button",
                  {
                    class: "btn primary",
                    onclick: (e) => {
                      e.stopPropagation();
                      pairDialog(d.name, d.address);
                    },
                  },
                  t("gui-pair"),
                ),
          ),
        );
      });
    listEl.replaceChildren(list(rows, found === null ? t("gui-searching") : t("gui-none-found")));
  };
  const discover = async () => {
    busy.hidden = false;
    try {
      lastDiscovery = await api("discover", { seconds: 3 });
    } catch (err) {
      if (!(err instanceof ServiceDown)) toast(errorText(err), "error");
      lastDiscovery = lastDiscovery || [];
    }
    busy.hidden = true;
    if (state.view === "network" && !state.remote) draw(lastDiscovery);
  };
  draw(lastDiscovery);
  discover();
  return [
    pageHead(
      t("gui-network-title"),
      t("gui-network-subtitle"),
      busy,
      h("button", { class: "btn", onclick: () => pairDialog(null, "") }, icon("plus"), t("gui-add-by-address")),
      h("button", { class: "btn", onclick: discover, title: t("gui-refresh") }, icon("refresh"), t("gui-refresh")),
    ),
    listEl,
  ];
}

function pairDialog(name, address) {
  const addr = h("input", { type: "text", value: address || "", placeholder: t("gui-address-hint"), autocomplete: "off" });
  const pin = h("input", {
    class: "pin-input",
    type: "text",
    inputmode: "numeric",
    maxlength: "7",
    autocomplete: "one-time-code",
    placeholder: "000000",
  });
  const errorEl = h("div", { class: "form-error", role: "alert" });
  const submit = h("button", { class: "btn primary", type: "submit" }, t("gui-pair"));
  const form = h(
    "form",
    {
      onsubmit: async (e) => {
        e.preventDefault();
        errorEl.textContent = "";
        submit.disabled = true;
        try {
          const peer = await api("pair", { address: addr.value.trim(), pin: pin.value.replace(/\D/g, "") });
          close();
          toast(t("gui-pair-done", { name: (peer && peer.name) || name || addr.value }));
          refresh();
        } catch (err) {
          if (!(err instanceof ServiceDown)) errorEl.textContent = errorText(err);
          submit.disabled = false;
          pin.select();
        }
      },
    },
    h("h2", {}, name ? t("gui-pair-title", { name }) : t("gui-add-by-address")),
    h("p", {}, t("gui-pair-body", { name: name || t("gui-nav-network") })),
    name ? null : h("label", { class: "field" }, t("gui-address"), addr),
    h("label", { class: "field" }, t("gui-pin"), pin),
    errorEl,
    h(
      "div",
      { class: "modal-actions" },
      h("button", { class: "btn", type: "button", onclick: () => close() }, t("gui-cancel")),
      submit,
    ),
  );
  const close = openModal(form);
  if (name) setTimeout(() => pin.focus(), 0);
}

// Devices shared by one paired server.
async function pageRemote() {
  const { fingerprint, name } = state.remote;
  const back = h("button", { class: "btn ghost", onclick: () => ((state.remote = null), render()) }, icon("back"), t("gui-back"));
  const devices = await api("remote_devices", { server: fingerprint });
  const rows = devices.map((d) => {
    names.put(fingerprint, d.id, deviceName(d));
    let end;
    if (d.attached_here) {
      end = [
        !d.allowed
          ? h("span", { class: "badge danger", title: t("gui-no-permission-hint") }, t("gui-no-permission"))
          : d.present
            ? h("span", { class: "badge ok" }, t("gui-connected-here"))
            : h("span", { class: "badge busy warn" }, t("gui-state-waiting-device")),
        h(
          "button",
          { class: "btn", onclick: () => act(async () => (await api("detach", { server: fingerprint, device: d.id }), refresh())) },
          t("gui-disconnect"),
        ),
      ];
    } else if (!d.allowed) {
      end = h("span", { class: "badge danger", title: t("gui-no-permission-hint") }, t("gui-no-permission"));
    } else if (d.in_use) {
      end = h("span", { class: "badge warn" }, t("gui-in-use-elsewhere"));
    } else {
      end = [
        d.present ? null : h("span", { class: "badge warn" }, t("gui-not-plugged-in")),
        h(
          "button",
          {
            class: "btn primary",
            title: d.present ? null : t("gui-connect-when-plugged-in"),
            onclick: () =>
              act(async () => {
                await api("attach", { server: fingerprint, device: d.id });
                toast(t("gui-state-connecting") + " " + deviceName(d));
                setTimeout(refresh, 800);
              }),
          },
          t("gui-connect"),
        ),
      ];
    }
    return h(
      "div",
      { class: "row" + (d.present ? "" : " absent") },
      h("div", { class: "tile" + (d.attached_here ? " on" : "") }, icon("usb")),
      h("div", { class: "row-body" }, h("div", { class: "row-title" }, deviceName(d)), deviceMeta(d)),
      h("div", { class: "row-end" }, end),
    );
  });
  return [back, pageHead(t("gui-devices-of", { name }), null), list(rows, t("gui-no-remote-devices"))];
}

// Remote devices attached to this computer.
function attachBadge(a) {
  switch (a.state) {
    case "attached":
      return h("span", { class: "badge ok" }, t("gui-state-attached"));
    case "connecting":
      return h("span", { class: "badge busy accent" }, t("gui-state-connecting"));
    case "retrying":
      return h("span", { class: "badge busy warn" }, t("gui-state-retrying", { seconds: a.seconds }));
    case "waiting":
      return a.error && a.error.code === "access_denied"
        ? h("span", { class: "badge danger" }, t("gui-no-permission"))
        : h("span", { class: "badge busy warn" }, t("gui-state-waiting-device"));
    case "queued":
      return h("span", { class: "badge busy warn" }, t("gui-state-queued", { position: a.position }));
    case "stopped":
      return h("span", { class: "badge" }, t("gui-state-stopped"));
    default:
      return h("span", { class: "badge danger" }, t("gui-state-failed"));
  }
}

async function pageConnected() {
  const items = await api("attachments");
  state.attachmentsCount = items.filter((a) => a.state === "attached").length;
  const rows = items.map((a) => {
    const title = a.name || names.get(a.server, a.device) || t("gui-unnamed-device");
    const again = () => act(async () => (await api("attach", { server: a.server, device: a.device }), refresh()));
    // Technical detail stays available as a tooltip.
    const detail =
      a.state === "failed"
        ? h("div", { class: "error-line", title: a.error.message }, errorText(a.error))
        : a.state === "retrying" && a.error
          ? h("div", { class: "row-meta", title: a.error.message }, errorText(a.error))
          : null;
    return h(
      "div",
      { class: "row" },
      h("div", { class: "tile" + (a.state === "attached" ? " on" : "") }, icon("usb")),
      h(
        "div",
        { class: "row-body" },
        h("div", { class: "row-title" }, title),
        h(
          "div",
          { class: "row-meta" },
          h("span", {}, t("gui-on-computer", { name: a.server_name || shortFp(a.server) })),
          a.busid ? h("span", { class: "mono" }, a.busid) : null,
          a.vendor_id !== null && a.vendor_id !== undefined ? h("span", { class: "mono" }, `${hex4(a.vendor_id)}:${hex4(a.product_id)}`) : null,
        ),
        detail,
      ),
      h(
        "div",
        { class: "row-end" },
        attachBadge(a),
        a.state === "failed" || a.state === "stopped" ? h("button", { class: "btn", onclick: again }, t("gui-reconnect")) : null,
        h(
          "button",
          { class: "btn", onclick: () => act(async () => (await api("detach", { server: a.server, device: a.device }), refresh())) },
          t("gui-disconnect"),
        ),
      ),
    );
  });
  return [pageHead(t("gui-connected-title"), t("gui-connected-subtitle")), list(rows, t("gui-connected-empty"))];
}

// Paired computers in both directions.
async function pagePaired() {
  const peers = await api("peers");
  const section = (title, items, isClient) => [
    h("h2", { class: "section-title" }, title),
    list(
      items.map((p) =>
        h(
          "div",
          { class: "row" },
          h("div", { class: "tile on" }, icon("computer")),
          h(
            "div",
            { class: "row-body" },
            h("div", { class: "row-title" }, p.name),
            h(
              "div",
              { class: "row-meta" },
              h("span", { class: "mono", title: t("gui-fingerprint") }, shortFp(p.fingerprint)),
              p.last_addr ? h("span", { class: "mono" }, p.last_addr) : null,
            ),
          ),
          h(
            "div",
            { class: "row-end" },
            isClient
              ? h("button", { class: "btn", onclick: () => act(() => clientDevicesDialog(p)) }, icon("shield"), t("gui-client-devices"))
              : null,
            h("button", { class: "btn danger", onclick: () => confirmForget(p) }, t("gui-remove")),
          ),
        ),
      ),
      t("gui-paired-empty"),
    ),
  ];
  const r = roles();
  return [
    pageHead(
      t("gui-paired-title"),
      t("gui-paired-subtitle"),
      r.server ? h("button", { class: "btn primary", onclick: () => act(showPin) }, icon("plus"), t("gui-pair-new")) : null,
    ),
    ...(r.client ? section(t("gui-paired-servers"), peers.servers) : []),
    ...(r.server ? section(t("gui-paired-clients"), peers.clients, true) : []),
  ];
}

function confirmForget(p) {
  const close = openModal([
    h("h2", {}, t("gui-remove")),
    h("p", {}, t("gui-remove-confirm", { name: p.name })),
    h(
      "div",
      { class: "modal-actions" },
      h("button", { class: "btn", onclick: () => close() }, t("gui-cancel")),
      h(
        "button",
        {
          class: "btn primary",
          onclick: () =>
            act(async () => {
              await api("forget", { fingerprint: p.fingerprint });
              close();
              refresh();
            }),
        },
        t("gui-remove"),
      ),
    ),
  ]);
}

// ---------------------------------------------------------------- access control

function choice(name, value, checked, title, body) {
  return h(
    "label",
    { class: "choice" },
    h("input", { type: "radio", name, value, checked }),
    h("span", {}, h("strong", {}, title), body ? h("small", {}, body) : null),
  );
}

function checkedValue(form, name) {
  const el = form.querySelector(`input[name="${name}"]:checked`);
  return el ? el.value : null;
}

// Right after sharing: if no computer may use the device yet (restricted
// policy, empty list), ask which ones may, as long as there is a choice.
async function askWhoMayUse(id) {
  const shared = (await api("local_devices")).find((x) => x.id === id);
  if (!shared || !shared.shared || shared.open_to_all) return;
  if (shared.access && shared.access.allowed && shared.access.allowed.length) return;
  if (!(await api("peers")).clients.length) return;
  await deviceDialog(id);
}

// Details of one shared device: who uses it and who waits (with
// "disconnect"), automatic handover, and who may use it. `id` is looked up
// again so the dialog shows the current state.
async function deviceDialog(id) {
  const [devices, peers, status] = [await api("local_devices"), await api("peers"), await api("status")];
  const d = devices.find((x) => x.id === id);
  if (!d || !d.shared) return;
  const access = d.access || { mode: "default", allowed: [] };
  const allowed = new Set(access.allowed || []);
  const queuePos = new Map((d.queue || []).map((q, i) => [q.fingerprint, i + 1]));

  // --- permissions column
  const defaultText = status.policy === "open" ? t("gui-access-default-open") : t("gui-access-default-restricted");
  const boxes = peers.clients.map((p) => {
    let mark = null;
    if (d.used_by_fingerprint === p.fingerprint) mark = h("span", { class: "badge accent" }, t("gui-badge-using"));
    else if (queuePos.has(p.fingerprint))
      mark = h("span", { class: "badge" }, t("gui-badge-queued", { position: queuePos.get(p.fingerprint) }));
    return h(
      "label",
      { class: "check" },
      h("input", { type: "checkbox", value: p.fingerprint, checked: allowed.has(p.fingerprint) }),
      h("span", {}, p.name),
      mark,
    );
  });
  const listBox = h(
    "div",
    { class: "checks" },
    h("div", { class: "checks-title" }, t("gui-access-computers")),
    boxes.length ? boxes : h("div", { class: "muted" }, t("gui-paired-empty")),
  );
  const permissions = h(
    "div",
    {},
    h("div", { class: "col-title" }, t("gui-col-permissions")),
    h("div", { class: "detail-section" }, t("gui-access-title")),
    h(
      "div",
      { class: "choices" },
      choice("mode", "default", access.mode === "default", t("gui-access-mode-default"), defaultText),
      choice("mode", "open", access.mode === "open", t("gui-access-mode-open"), t("gui-access-mode-open-body")),
      choice("mode", "selected", access.mode === "selected", t("gui-access-mode-selected"), t("gui-access-mode-selected-body")),
    ),
    listBox,
    h("p", { class: "note muted" }, t("gui-access-revoke-note")),
  );

  // --- status column
  const disconnect = h("button", { class: "btn small danger", type: "button" }, t("gui-disconnect-user"));
  disconnect.onclick = () =>
    act(async () => {
      disconnect.disabled = true;
      await api("disconnect", { device: d.id });
      toast(t("gui-disconnected-user", { name: d.used_by }));
      close();
      await refresh();
    });
  const user = d.used_by
    ? h(
        "div",
        { class: "user-line" },
        h("span", {}, h("strong", {}, d.used_by), h("br"), h("span", { class: "muted" }, d.used_since ? t("gui-since", { time: formatTime(d.used_since) }) : "")),
        disconnect,
      )
    : h("div", { class: "user-line" }, h("span", { class: "muted" }, t("gui-nobody-using")));
  const queue = (d.queue || []).length
    ? h("ol", { class: "queue" }, (d.queue || []).map((q) => h("li", {}, q.name)))
    : h("p", { class: "muted note" }, t("gui-queue-empty"));

  const defaultOn = d.kind === "dongle" || d.kind === "printer";
  const handover = d.handover || { mode: "default", seconds: null };
  const defaultLabel = defaultOn
    ? t("gui-handover-default-on", { seconds: 30 })
    : t("gui-handover-default-off");
  const modeSel = h(
    "select",
    {},
    h("option", { value: "default", selected: handover.mode === "default" }, defaultLabel),
    h("option", { value: "on", selected: handover.mode === "on" }, t("gui-handover-on")),
    h("option", { value: "off", selected: handover.mode === "off" }, t("gui-handover-off")),
  );
  const seconds = h("input", { type: "number", min: "1", max: "3600", value: String(handover.seconds || 30) });
  const updateHandover = () => {
    seconds.disabled = modeSel.value !== "on";
  };
  modeSel.onchange = updateHandover;
  updateHandover();
  const statusCol = h(
    "div",
    {},
    h("div", { class: "col-title" }, t("gui-col-status")),
    h("div", { class: "detail-section" }, t("gui-in-use-title")),
    user,
    h("div", { class: "detail-section" }, t("gui-queue-title", { count: (d.queue || []).length })),
    queue,
    h("div", { class: "detail-section" }, t("gui-handover-title")),
    h(
      "div",
      { class: "handover" },
      modeSel,
      h("div", { class: "row2" }, h("span", { class: "muted" }, t("gui-handover-before")), seconds, h("span", { class: "muted" }, t("gui-handover-after"))),
    ),
  );

  const errorEl = h("div", { class: "form-error", role: "alert" });
  const form = h(
    "form",
    {
      onchange: () => update(),
      onsubmit: async (e) => {
        e.preventDefault();
        errorEl.textContent = "";
        try {
          await api("set_device_access", {
            device: d.id,
            mode: checkedValue(form, "mode"),
            allowed: [...form.querySelectorAll(".checks input:checked")].map((i) => i.value),
          });
          const mode = modeSel.value;
          if (mode !== handover.mode || (mode === "on" && Number(seconds.value) !== (handover.seconds || 30))) {
            await api("set_device_handover", { device: d.id, mode, seconds: mode === "on" ? Number(seconds.value) : null });
          }
          close();
          refresh();
        } catch (err) {
          if (!(err instanceof ServiceDown)) errorEl.textContent = errorText(err);
        }
      },
    },
    h("h2", {}, deviceName(d)),
    h("div", { class: "meta-line" }, deviceMeta(d, d.kind ? [h("span", {}, t("gui-kind-" + d.kind))] : [])),
    h("div", { class: "cols" }, permissions, statusCol),
    errorEl,
    h(
      "div",
      { class: "modal-actions" },
      h("button", { class: "btn", type: "button", onclick: () => close() }, t("gui-cancel")),
      h("button", { class: "btn primary", type: "submit" }, t("gui-save")),
    ),
  );
  const update = () => {
    const mode = checkedValue(form, "mode");
    listBox.hidden = mode === "open" || (mode === "default" && status.policy === "open");
  };
  update();
  const close = openModal(form, { wide: true });
}

// Which shared devices one computer may use (also shown right after pairing
// when access is restricted).
async function clientDevicesDialog(peer, { afterPairing = false } = {}) {
  const devices = (await api("local_devices")).filter((d) => d.shared);
  const rows = devices.map((d) => {
    const mine = !!(d.access && (d.access.allowed || []).includes(peer.fingerprint));
    return h(
      "label",
      { class: "check" },
      h("input", {
        type: "checkbox",
        value: d.id,
        checked: d.open_to_all || mine,
        disabled: d.open_to_all,
        "data-mine": mine ? "1" : null,
      }),
      h("span", {}, deviceName(d)),
      d.open_to_all ? h("span", { class: "muted" }, t("gui-access-everyone")) : null,
    );
  });
  const errorEl = h("div", { class: "form-error", role: "alert" });
  const form = h(
    "form",
    {
      onsubmit: async (e) => {
        e.preventDefault();
        errorEl.textContent = "";
        // Devices open to everyone keep whatever they had for this computer.
        const chosen = [...form.querySelectorAll("input[type=checkbox]")]
          .filter((i) => (i.disabled ? i.dataset.mine === "1" : i.checked))
          .map((i) => i.value);
        try {
          await api("set_client_access", { fingerprint: peer.fingerprint, devices: chosen });
          close();
          refresh();
        } catch (err) {
          if (!(err instanceof ServiceDown)) errorEl.textContent = errorText(err);
        }
      },
    },
    h("h2", {}, t("gui-client-devices-title", { name: peer.name })),
    h("p", {}, afterPairing ? t("gui-client-devices-after-pairing") : t("gui-client-devices-body")),
    h("div", { class: "checks" }, rows.length ? rows : h("div", { class: "muted" }, t("gui-no-shared-devices"))),
    errorEl,
    h(
      "div",
      { class: "modal-actions" },
      h("button", { class: "btn", type: "button", onclick: () => close() }, afterPairing ? t("gui-skip") : t("gui-cancel")),
      h("button", { class: "btn primary", type: "submit" }, t("gui-save")),
    ),
  );
  const close = openModal(form);
}

function policyChoices(current) {
  return h(
    "div",
    { class: "choices" },
    choice("policy", "open", current === "open", t("gui-policy-open"), t("gui-policy-open-body")),
    choice("policy", "restricted", current === "restricted", t("gui-policy-restricted"), t("gui-policy-restricted-body")),
  );
}

// First-run question: the default access policy.
function policyDialog() {
  const form = h(
    "form",
    {
      onsubmit: (e) => {
        e.preventDefault();
        act(async () => {
          await api("set_policy", { policy: checkedValue(form, "policy") });
          close();
          refresh();
        });
      },
    },
    h("h2", {}, t("gui-policy-first-title")),
    h("p", {}, t("gui-policy-first-body")),
    policyChoices("open"),
    h("p", { class: "note" }, t("gui-policy-later")),
    h("div", { class: "modal-actions" }, h("button", { class: "btn primary", type: "submit" }, t("gui-save"))),
  );
  const close = openModal(form);
}

// ---------------------------------------------------------------- history

const HISTORY_KINDS = {
  paired: "gui-history-paired",
  pairing_failed: "gui-history-pairing-failed",
  attached: "gui-history-attached",
  detached: "gui-history-detached",
  denied: "gui-history-denied",
};

function formatTime(unix) {
  return new Date(unix * 1000).toLocaleString(state.lang || undefined);
}

function formatDuration(secs) {
  if (secs === null || secs === undefined) return "";
  const hh = Math.floor(secs / 3600);
  const mm = Math.floor((secs % 3600) / 60);
  const ss = String(secs % 60).padStart(2, "0");
  return hh ? `${hh}:${String(mm).padStart(2, "0")}:${ss}` : `${mm}:${ss}`;
}

function historyDevice(e) {
  return e.device_name || e.device || "";
}

function csvField(v) {
  let s = v === null || v === undefined ? "" : String(v);
  // Keep spreadsheets from running cell contents as formulas.
  if (/^[=+\-@\t\r]/.test(s)) s = "'" + s;
  return /[",;\r\n]/.test(s) ? '"' + s.replace(/"/g, '""') + '"' : s;
}

async function exportHistory() {
  const view = await api("usage", {});
  const header = [
    "gui-history-time",
    "gui-history-event",
    "gui-history-computer",
    "gui-address",
    "gui-history-device",
    "gui-history-device-id",
    "gui-history-duration",
    "gui-fingerprint",
  ].map((k) => t(k));
  const lines = [header];
  for (const e of view.entries.slice().reverse()) {
    lines.push([
      new Date(e.time * 1000).toISOString(),
      t(HISTORY_KINDS[e.kind] || e.kind),
      e.computer,
      e.address,
      e.device_name,
      e.device,
      e.duration_secs,
      e.fingerprint,
    ]);
  }
  // A byte order mark lets spreadsheet programs detect UTF-8.
  const csv = "\ufeff" + lines.map((l) => l.map(csvField).join(",")).join("\r\n") + "\r\n";
  const url = URL.createObjectURL(new Blob([csv], { type: "text/csv;charset=utf-8" }));
  const a = h("a", { href: url, download: `usbnexus-history-${new Date().toISOString().slice(0, 10)}.csv` });
  document.body.append(a);
  a.click();
  a.remove();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

async function pageHistory() {
  const view = await api("usage", { limit: 500 });
  const rows = view.entries.map((e) => {
    const bad = e.kind === "denied" || e.kind === "pairing_failed";
    return h(
      "tr",
      { class: bad ? "bad" : null },
      h("td", { class: "nowrap" }, formatTime(e.time)),
      h("td", {}, t(HISTORY_KINDS[e.kind] || e.kind)),
      h("td", { title: e.fingerprint ? shortFp(e.fingerprint) : null }, e.computer || "", e.address ? h("div", { class: "muted mono" }, e.address) : null),
      h("td", { title: e.device || null }, historyDevice(e)),
      h("td", { class: "nowrap" }, formatDuration(e.duration_secs)),
    );
  });
  const table = rows.length
    ? h(
        "div",
        { class: "list table-wrap" },
        h(
          "table",
          { class: "table" },
          h(
            "thead",
            {},
            h(
              "tr",
              {},
              ...["gui-history-time", "gui-history-event", "gui-history-computer", "gui-history-device", "gui-history-duration"].map((k) =>
                h("th", {}, t(k)),
              ),
            ),
          ),
          h("tbody", {}, rows),
        ),
      )
    : list([], t("gui-history-empty"));
  return [
    pageHead(
      t("gui-history-title"),
      t("gui-history-subtitle", { days: view.retention_days }),
      h("button", { class: "btn", onclick: () => act(exportHistory), disabled: !rows.length }, icon("download"), t("gui-history-export")),
    ),
    table,
  ];
}

// ---------------------------------------------------------------- settings

async function pageSettings() {
  const [status, usage] = [await api("status"), await api("usage", { limit: 0 })];
  const policyForm = h(
    "form",
    {
      class: "card",
      onsubmit: (e) => {
        e.preventDefault();
        act(async () => {
          await api("set_policy", { policy: checkedValue(policyForm, "policy") });
          toast(t("gui-saved"));
          state.status = await api("status");
        });
      },
    },
    h("h2", {}, t("gui-policy-title")),
    h("p", {}, t("gui-policy-body")),
    policyChoices(status.policy),
    h("p", { class: "note" }, t("gui-access-revoke-note")),
    h("div", { class: "modal-actions" }, h("button", { class: "btn primary", type: "submit" }, t("gui-save"))),
  );
  const days = h("input", { type: "number", min: "1", max: "3650", value: String(usage.retention_days), required: true });
  const retentionForm = h(
    "form",
    {
      class: "card",
      onsubmit: (e) => {
        e.preventDefault();
        act(async () => {
          await api("set_usage_retention", { days: Number(days.value) });
          toast(t("gui-saved"));
        });
      },
    },
    h("h2", {}, t("gui-retention-title")),
    h("p", {}, t("gui-retention-body")),
    h("label", { class: "field" }, t("gui-retention-days"), days),
    h("div", { class: "modal-actions" }, h("button", { class: "btn primary", type: "submit" }, t("gui-save"))),
  );
  const r = status.roles || { server: true, client: true };
  const web = await webCard();
  return [pageHead(t("gui-settings-title"), null), rolesForm(r), web, r.server ? policyForm : null, retentionForm];
}

// The web interface: on/off, reachable from this computer only or the whole
// network, port and password. Changeable in the app only (the web interface
// cannot reconfigure itself).
async function webCard() {
  const w = await api("web_status");
  const info = h(
    "div",
    {},
    w.enabled && w.urls.length
      ? h("p", {}, t("gui-web-open-at"), " ", ...w.urls.flatMap((u, i) => [i ? ", " : null, h("code", {}, u)]))
      : null,
    w.enabled && w.fingerprint ? h("p", { class: "note muted" }, t("gui-web-fingerprint", { fp: w.fingerprint })) : null,
    w.enabled && w.error ? h("p", { class: "form-error" }, w.error) : null,
  );
  if (state.os === "web") {
    return h("div", { class: "card" }, h("h2", {}, t("gui-web-title")), h("p", {}, t("gui-web-local-only-note")), info);
  }
  const enabled = h("input", { type: "checkbox", checked: w.enabled });
  const local = h("input", { type: "radio", name: "web-access", value: "local", checked: !w.lan });
  const network = h("input", { type: "radio", name: "web-access", value: "network", checked: w.lan });
  const port = h("input", { type: "number", min: "1", max: "65535", value: String(w.port), required: true });
  const pw1 = h("input", { type: "password", autocomplete: "new-password", placeholder: w.password_set ? "" : "" });
  const pw2 = h("input", { type: "password", autocomplete: "new-password" });
  const errorEl = h("div", { class: "form-error", role: "alert" });
  const fields = h(
    "div",
    { class: "web-fields" },
    h("div", { class: "checks" }, h("label", { class: "check" }, local, h("span", {}, t("gui-web-access-local"))), h("label", { class: "check" }, network, h("span", {}, t("gui-web-access-network")))),
    h("label", { class: "field" }, t("gui-web-port"), port),
    h("label", { class: "field" }, t("gui-web-new-password"), pw1),
    h("label", { class: "field" }, t("gui-web-repeat-password"), pw2),
    h("p", { class: "note muted" }, w.password_set ? t("gui-web-password-keep") : t("gui-web-password-required")),
  );
  const updateFields = () => {
    fields.hidden = !enabled.checked;
  };
  enabled.addEventListener("change", updateFields);
  updateFields();
  const form = h(
    "form",
    {
      class: "card",
      onsubmit: (e) => {
        e.preventDefault();
        errorEl.textContent = "";
        act(async () => {
          if (pw1.value !== pw2.value) {
            errorEl.textContent = t("gui-web-mismatch");
            return;
          }
          const args = { enabled: enabled.checked, lan: network.checked, port: Number(port.value) };
          if (pw1.value) args.password = pw1.value;
          try {
            await api("web_configure", args);
          } catch (err) {
            if (!(err instanceof ServiceDown)) errorEl.textContent = errorText(err);
            return;
          }
          toast(t("gui-saved"));
          await render();
        });
      },
    },
    h("h2", {}, t("gui-web-title")),
    h("p", {}, t("gui-web-body")),
    h("div", { class: "checks" }, h("label", { class: "check" }, enabled, h("span", {}, t("gui-web-enabled")))),
    fields,
    info,
    errorEl,
    h("div", { class: "modal-actions" }, h("button", { class: "btn primary", type: "submit" }, t("gui-save"))),
  );
  return form;
}

// What this computer is used for; adding a role installs its drivers.
function rolesForm(current) {
  const server = h("input", { type: "checkbox", checked: current.server });
  const client = h("input", { type: "checkbox", checked: current.client });
  const note = h("p", { class: "note" }, t("gui-roles-client-note"));
  const save = h("button", { class: "btn primary", type: "submit" }, t("gui-save"));
  const update = () => {
    save.disabled = !server.checked && !client.checked;
    // Only Windows installs a driver (usbip-win2) for the client role.
    note.hidden = !(state.os === "windows" && client.checked && !current.client);
  };
  server.addEventListener("change", update);
  client.addEventListener("change", update);
  update();
  const form = h(
    "form",
    {
      class: "card",
      onsubmit: (e) => {
        e.preventDefault();
        act(async () => {
          save.disabled = true;
          save.textContent = t("gui-roles-applying");
          try {
            await api("set_roles", { server: server.checked, client: client.checked });
            toast(t("gui-saved"));
            state.status = await api("status");
          } finally {
            save.textContent = t("gui-save");
          }
          await render();
        });
      },
    },
    h("h2", {}, t("gui-roles-title")),
    h("p", {}, t("gui-roles-body")),
    h(
      "div",
      { class: "checks" },
      h("label", { class: "check" }, server, h("span", {}, t("gui-role-server"))),
      h("label", { class: "check" }, client, h("span", {}, t("gui-role-client"))),
    ),
    note,
    h("div", { class: "modal-actions" }, save),
  );
  return form;
}

// This user may not use the service's socket (Linux: not in the usbnexus
// group). On Linux a button fixes it through polkit.
function pageServicePermission() {
  const grant = h("button", { class: "btn primary" }, t("gui-grant-access"));
  grant.onclick = () =>
    act(async () => {
      grant.disabled = true;
      try {
        await invoke("grant_access");
        toast(t("gui-grant-access-done"));
        state.serviceDown = false;
        await refresh();
      } finally {
        grant.disabled = false;
      }
    });
  return h(
    "div",
    { class: "down" },
    h("img", { src: "logo.svg", alt: "" }),
    h("h1", {}, t("gui-service-denied-title")),
    h("p", {}, t("gui-service-denied-body")),
    h("p", {}, t("gui-service-denied-command")),
    h("code", {}, "sudo usbnexus allow-user " + t("gui-service-denied-user")),
    h("div", {}, state.os === "linux" ? grant : null, " ", h("button", { class: "btn", onclick: () => refresh() }, t("gui-retry"))),
  );
}

function pageServiceDown() {
  if (state.serviceDownReason === "permission_denied") return pageServicePermission();
  return h(
    "div",
    { class: "down" },
    h("img", { src: "logo.svg", alt: "" }),
    h("h1", {}, t("gui-service-down-title")),
    h("p", {}, t("gui-service-down-body")),
    ...(state.os === "windows"
      ? [h("p", {}, t("gui-service-down-windows")), h("code", {}, "usbnexus service install")]
      : state.os === "macos"
        ? [
            h("p", {}, t("gui-service-down-macos")),
            h("code", {}, "sudo launchctl bootstrap system /Library/LaunchDaemons/org.usbnexus.daemon.plist"),
          ]
        : [h("p", {}, t("gui-service-down-linux")), h("code", {}, "sudo systemctl start usbnexus")]),
    h("div", {}, h("button", { class: "btn primary", onclick: () => refresh() }, t("gui-retry"))),
  );
}

// Something a role still needs on this computer (e.g. kernel modules).
function setupBanner(issue) {
  if (issue.code !== "kernel_modules_missing") return null;
  return h(
    "div",
    { class: "banner" },
    t("gui-setup-kernel-modules", { modules: issue.detail }),
    " ",
    issue.command ? h("code", {}, issue.command) : t("gui-setup-kernel-modules-nocmd"),
  );
}

// ---------------------------------------------------------------- render loop

const PAGES = {
  this: pageThis,
  network: pageNetwork,
  connected: pageConnected,
  paired: pagePaired,
  history: pageHistory,
  settings: pageSettings,
};

let renderSeq = 0;

async function render() {
  renderSidebar();
  const main = document.getElementById("main");
  if (state.serviceDown) {
    main.replaceChildren(pageServiceDown());
    return;
  }
  // The current view may belong to a role this computer no longer has.
  if (!visibleViews().some(([id]) => id === state.view)) state.view = visibleViews()[0][0];
  const seq = ++renderSeq;
  try {
    const content = await PAGES[state.view]();
    if (seq !== renderSeq) return; // a newer render started meanwhile
    const reboot = state.status && state.status.reboot_required ? h("div", { class: "banner" }, t("gui-reboot-required")) : null;
    const issues = ((state.status && state.status.setup_issues) || []).map(setupBanner);
    main.replaceChildren(h("div", { class: "page" }, reboot, issues, content));
  } catch (err) {
    if (err instanceof ServiceDown || seq !== renderSeq) return;
    main.replaceChildren(h("div", { class: "page" }, h("div", { class: "list" }, h("div", { class: "empty" }, errorText(err)))));
  }
  renderSidebar();
}

/** Refreshes status and the current page without losing modal state. */
async function refresh() {
  try {
    state.status = await api("status");
    const items = await api("attachments");
    state.attachmentsCount = items.filter((a) => a.state === "attached").length;
  } catch {
    /* handled by api() */
  }
  // First run: ask who may use shared devices (once per window).
  if (state.status && roles().server && !state.status.policy_chosen && !state.policyAsked && document.getElementById("modal").hidden) {
    state.policyAsked = true;
    policyDialog();
  }
  // Network discovery is refreshed on demand only; forms are not redrawn.
  if (((state.view === "network" && !state.remote) || state.view === "settings") && !state.serviceDown) {
    renderSidebar();
    return;
  }
  await render();
}

function startPolling() {
  clearInterval(state.pollTimer);
  state.pollTimer = setInterval(() => {
    const modalOpen = !document.getElementById("modal").hidden;
    if (!modalOpen && document.visibilityState === "visible") refresh();
  }, 2500);
}

async function main() {
  let saved = null;
  try {
    saved = localStorage.getItem("lang");
  } catch {
    /* ignore */
  }
  await loadStrings(saved);
  await refresh();
  startPolling();
}

// In the web interface, web.js starts the app after signing in.
window.usbnexusStart = main;
if (!window.USBNEXUS_WEB) main();
