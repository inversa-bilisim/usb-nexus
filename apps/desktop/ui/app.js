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
  get(server, busid) {
    try {
      return JSON.parse(localStorage.getItem("names") || "{}")[server + "/" + busid];
    } catch {
      return undefined;
    }
  },
  put(server, busid, name) {
    try {
      const all = JSON.parse(localStorage.getItem("names") || "{}");
      all[server + "/" + busid] = name;
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
    if (err && err.code === "service_unavailable") {
      if (!state.serviceDown) {
        state.serviceDown = true;
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

function openModal(content, { onClose } = {}) {
  const backdrop = document.getElementById("modal");
  const box = h("div", { class: "modal", role: "dialog", "aria-modal": "true" }, content);
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
  backdrop.onclick = (e) => e.target === backdrop && close();
  setTimeout(() => (box.querySelector("input, .btn.primary, .btn") || box).focus(), 0);
  return close;
}

// ---------------------------------------------------------------- sidebar

const VIEWS = [
  ["this", "computer", "gui-nav-this-computer"],
  ["network", "network", "gui-nav-network"],
  ["connected", "plug", "gui-nav-connected"],
  ["paired", "link", "gui-nav-paired"],
];

function renderSidebar() {
  const nav = document.getElementById("nav");
  nav.setAttribute("aria-label", t("gui-nav-this-computer"));
  nav.replaceChildren(
    ...VIEWS.map(([id, ic, key]) =>
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
    h("span", { class: "mono" }, d.busid),
    h("span", { class: "mono" }, `${hex4(d.vendor_id)}:${hex4(d.product_id)}`),
    d.speed ? h("span", {}, d.speed) : null,
    ...extra,
  );
}

// This computer: local devices and sharing.
async function pageThis() {
  const devices = await api("local_devices");
  const rows = devices.map((d) => {
    const toggle = h("button", {
      class: "switch",
      role: "switch",
      "aria-checked": String(d.shared),
      "aria-label": t("gui-share") + " " + deviceName(d),
      title: t("gui-share"),
      onclick: () =>
        act(async () => {
          toggle.disabled = true;
          await api("set_shared", { busid: d.busid, shared: !d.shared });
          await refresh();
        }),
    });
    let badge;
    if (d.used_by) badge = h("span", { class: "badge accent" }, t("gui-used-by", { name: d.used_by }));
    else if (d.shared) badge = h("span", { class: "badge ok" }, t("gui-shared"));
    else badge = h("span", { class: "badge" }, t("gui-not-shared"));
    return h(
      "div",
      { class: "row" },
      h("div", { class: "tile" + (d.shared ? " on" : "") }, icon("usb")),
      h("div", { class: "row-body" }, h("div", { class: "row-title" }, deviceName(d)), deviceMeta(d)),
      h("div", { class: "row-end" }, badge, toggle),
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
    names.put(fingerprint, d.busid, deviceName(d));
    let end;
    if (d.attached_here) {
      end = [
        h("span", { class: "badge ok" }, t("gui-connected-here")),
        h(
          "button",
          { class: "btn", onclick: () => act(async () => (await api("detach", { server: fingerprint, busid: d.busid }), refresh())) },
          t("gui-disconnect"),
        ),
      ];
    } else if (d.in_use) {
      end = h("span", { class: "badge warn" }, t("gui-in-use-elsewhere"));
    } else {
      end = h(
        "button",
        {
          class: "btn primary",
          onclick: () =>
            act(async () => {
              await api("attach", { server: fingerprint, busid: d.busid });
              toast(t("gui-state-connecting") + " " + deviceName(d));
              setTimeout(refresh, 800);
            }),
        },
        t("gui-connect"),
      );
    }
    return h(
      "div",
      { class: "row" },
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
    const title = names.get(a.server, a.busid) || t("gui-unnamed-device");
    const again = () => act(async () => (await api("attach", { server: a.server, busid: a.busid }), refresh()));
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
          h("span", { class: "mono" }, a.busid),
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
          { class: "btn", onclick: () => act(async () => (await api("detach", { server: a.server, busid: a.busid }), refresh())) },
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
  const section = (title, items) => [
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
          h("div", { class: "row-end" }, h("button", { class: "btn danger", onclick: () => confirmForget(p) }, t("gui-remove"))),
        ),
      ),
      t("gui-paired-empty"),
    ),
  ];
  return [
    pageHead(t("gui-paired-title"), t("gui-paired-subtitle")),
    ...section(t("gui-paired-servers"), peers.servers),
    ...section(t("gui-paired-clients"), peers.clients),
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

function pageServiceDown() {
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

// ---------------------------------------------------------------- render loop

const PAGES = { this: pageThis, network: pageNetwork, connected: pageConnected, paired: pagePaired };

let renderSeq = 0;

async function render() {
  renderSidebar();
  const main = document.getElementById("main");
  if (state.serviceDown) {
    main.replaceChildren(pageServiceDown());
    return;
  }
  const seq = ++renderSeq;
  try {
    const content = await PAGES[state.view]();
    if (seq !== renderSeq) return; // a newer render started meanwhile
    main.replaceChildren(h("div", { class: "page" }, content));
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
  // Network discovery is refreshed on demand only.
  if (state.view === "network" && !state.remote && !state.serviceDown) {
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
