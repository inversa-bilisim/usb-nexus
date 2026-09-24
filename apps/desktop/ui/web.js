// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

// Browser shim for the web interface. It stands in for the Tauri bridge the
// desktop app uses (window.__TAURI__.core.invoke), talks to the service over
// HTTPS, and handles signing in and out. Loaded before app.js.

"use strict";

(() => {
  window.USBNEXUS_WEB = true;
  const HEADERS = { "Content-Type": "application/json", "X-USBNexus": "1" };
  let messages = {};

  const t = (id, args = {}) =>
    (messages[id] ?? id).replace(/\{(\w+)\}/g, (_, k) => (k in args ? String(args[k]) : ""));

  function savedLang() {
    try {
      return localStorage.getItem("lang") || "";
    } catch {
      return "";
    }
  }

  async function strings(lang) {
    const r = await fetch("/api/strings?lang=" + encodeURIComponent(lang || ""));
    const s = await r.json();
    messages = s.messages;
    return s;
  }

  window.__TAURI__ = {
    core: {
      async invoke(cmd, args) {
        if (cmd === "ui_strings") return strings(args && args.lang);
        if (cmd === "api") {
          const r = await fetch("/api/call", { method: "POST", headers: HEADERS, body: JSON.stringify(args.request) });
          if (r.status === 401) {
            // Session expired: back to the sign-in page.
            location.reload();
            throw { code: "not_logged_in", message: "session expired" };
          }
          const body = await r.json();
          if (body.status === "ok") return body.data;
          throw body.error || { code: "other", message: "HTTP " + r.status };
        }
        throw { code: "invalid", message: "unknown command " + cmd };
      },
    },
  };

  function el(tag, attrs = {}, ...children) {
    const e = document.createElement(tag);
    for (const [k, v] of Object.entries(attrs)) {
      if (k.startsWith("on")) e.addEventListener(k.slice(2), v);
      else e.setAttribute(k, v);
    }
    for (const c of children) e.append(c instanceof Node ? c : document.createTextNode(String(c)));
    return e;
  }

  function addSignOut() {
    const box = document.getElementById("web-extra");
    if (!box) return;
    box.replaceChildren(
      el(
        "button",
        {
          class: "btn",
          onclick: async () => {
            await fetch("/api/logout", { method: "POST", headers: HEADERS });
            location.reload();
          },
        },
        t("gui-web-sign-out"),
      ),
    );
  }

  function showLogin(name) {
    document.title = "USB Nexus – " + name;
    document.querySelector(".sidebar").hidden = true;
    document.getElementById("app").classList.add("login-mode");
    const password = el("input", { type: "password", autocomplete: "current-password", required: "" });
    const errorEl = el("div", { class: "form-error", role: "alert" });
    const submit = el("button", { class: "btn primary", type: "submit" }, t("gui-web-sign-in"));
    const form = el(
      "form",
      {
        class: "modal login",
        onsubmit: async (e) => {
          e.preventDefault();
          errorEl.textContent = "";
          submit.disabled = true;
          const r = await fetch("/api/login", {
            method: "POST",
            headers: HEADERS,
            body: JSON.stringify({ password: password.value }),
          });
          if (r.ok) {
            location.reload();
            return;
          }
          const body = await r.json().catch(() => ({}));
          errorEl.textContent =
            r.status === 429
              ? t("gui-web-locked", { seconds: body.retry_after || 60 })
              : t("gui-web-wrong-password");
          submit.disabled = false;
          password.select();
        },
      },
      el("img", { src: "logo.svg", alt: "", class: "login-logo" }),
      el("h2", {}, t("gui-web-login-title", { name })),
      el("label", { class: "field" }, t("gui-web-password"), password),
      errorEl,
      el("div", { class: "modal-actions" }, submit),
    );
    document.getElementById("main").replaceChildren(el("div", { class: "login-wrap" }, form));
    password.focus();
  }

  window.addEventListener("DOMContentLoaded", async () => {
    await strings(savedLang());
    const s = await (await fetch("/api/session")).json();
    if (s.logged_in) {
      addSignOut();
      window.usbnexusStart();
    } else {
      showLogin(s.name);
    }
  });
})();
