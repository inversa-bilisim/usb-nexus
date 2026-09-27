# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — German.
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = Sicheres USB over IP: USB-Geräte im Netzwerk freigeben, mit Verschlüsselung, Kopplung und automatischer Wiederverbindung.
arg-lang = Sprache der Benutzeroberfläche (z. B. en, tr). Standardmäßig die Systemsprache.
arg-state-dir = Verzeichnis für Schlüssel und gekoppelte Computer.
arg-name = Name dieses Computers, wie er anderen angezeigt wird.
arg-verbose = Detaillierte Protokollmeldungen anzeigen.

cmd-daemon-about = Den USB-Nexus-Dienst ausführen (wird von der Desktop-App verwendet).
arg-allow-all-users = Jedem lokalen Benutzer erlauben, den Dienst zu steuern (Standard: Mitglieder der Gruppe usbnexus).
arg-socket = Pfad des lokalen Steuerungssockets des Dienstes.
cmd-service-about = Den USB-Nexus-Windows-Dienst installieren oder entfernen.
cmd-install-about = Den Dienst installieren, jetzt und bei jedem Start ausführen (als Administrator ausführen).
cmd-uninstall-about = Den Dienst beenden und entfernen.
cmd-serve-about = USB-Geräte dieses Computers freigeben.
arg-export = Bus-ID eines freizugebenden Geräts (wiederholbar). Siehe: usbnexus local
arg-listen = Adresse und Port, auf denen gelauscht wird.
arg-pair = Beim Start ein Kopplungsfenster öffnen und die PIN anzeigen.
arg-no-mdns = Diesen Server nicht im lokalen Netzwerk ankündigen.

cmd-pin-about = Eine PIN anzeigen, um einen neuen Computer mit dem laufenden Server zu koppeln.
arg-seconds = Wie lange die PIN gültig bleibt, in Sekunden.

cmd-local-about = USB-Geräte anzeigen, die mit diesem Computer verbunden sind.
cmd-discover-about = USB-Nexus-Server im lokalen Netzwerk suchen.
arg-timeout = Wie lange gesucht wird, in Sekunden.
cmd-pair-about = Mit einem Server koppeln, mithilfe der von ihm angezeigten PIN.
arg-server = Server: gekoppelter Name, Fingerabdruck oder Host[:Port].
arg-pin = Vom Server angezeigte PIN (wird abgefragt, wenn nicht angegeben).
cmd-list-about = Von einem Server freigegebene Geräte anzeigen.
cmd-allow-user-about = Einem Benutzer erlauben, den Dienst über die Desktop-App zu steuern (fügt ihn der Gruppe usbnexus hinzu).
arg-user = Benutzername.
allow-user-invalid = „{ $user }“ ist kein gültiger Benutzername.
allow-user-failed = „{ $user }“ konnte nicht zur Gruppe usbnexus hinzugefügt werden.
allow-user-done = „{ $user }“ kann den USB-Nexus-Dienst jetzt steuern.
allow-user-relogin = Dies gilt nach der nächsten Anmeldung.
cmd-attach-about = Ein entferntes Gerät auf diesem Computer verwenden; verbindet sich automatisch neu.
arg-busid = Bus-ID des entfernten Geräts.
cmd-peers-about = Gekoppelte Computer anzeigen.
cmd-forget-about = Einen gekoppelten Computer entfernen.
arg-peer = Name oder Fingerabdruck des gekoppelten Computers.

## General

error-prefix = Fehler: { $detail }
hint-root = Für diesen Vorgang sind Administratorrechte erforderlich. Versuchen Sie es erneut mit sudo.
unsupported-os = Dieser Befehl wird auf diesem Betriebssystem noch nicht unterstützt.
yes = ja
no = nein

service-installed = Der USB-Nexus-Dienst wurde installiert und gestartet.
service-removed = Der USB-Nexus-Dienst wurde entfernt.

## Server

serve-started = Server „{ $name }“ lauscht auf { $addr }.
serve-fingerprint = Fingerabdruck: { $fp }
serve-exporting = Freigegebene Geräte:
serve-no-exports = Es sind keine Geräte freigegeben. Fügen Sie --export BUSID hinzu (Geräte auflisten mit: usbnexus local).
serve-pairing-pin = Kopplungs-PIN: { $pin } (gültig für { $seconds } Sekunden)
serve-paired = Mit „{ $name }“ gekoppelt.
serve-pairing-failed = Fehlgeschlagener Kopplungsversuch von { $addr }.
serve-exported = { $busid } wird jetzt von „{ $client }“ verwendet.
serve-released = { $busid } wurde von „{ $client }“ freigegeben.
serve-stopping = Wird beendet; Geräte werden ihren normalen Treibern zurückgegeben …
serve-mdns-failed = Die Suche im lokalen Netzwerk ist nicht verfügbar: { $detail }
serve-bind-failed = { $busid } konnte nicht für die Freigabe vorbereitet werden: { $detail }

## Pairing

pin-show = Kopplungs-PIN: { $pin }
pin-hint = Führen Sie auf dem anderen Computer innerhalb von { $seconds } Sekunden „usbnexus pair { $name }“ aus.
pin-no-server = Es wurde kein laufender USB-Nexus-Server gefunden. Starten Sie einen mit: usbnexus serve
pair-enter-pin = Geben Sie die auf dem Server angezeigte PIN ein:
pair-ok = Mit „{ $name }“ ({ $fp }) gekoppelt.
pair-already = Bereits mit „{ $name }“ gekoppelt.

## Devices

local-header = USB-Geräte auf diesem Computer:
local-empty = Keine USB-Geräte gefunden.
list-header = Von „{ $name }“ freigegebene Geräte:
list-empty = Dieser Server gibt keine Geräte frei.
list-in-use = in Verwendung
col-busid = BUS-ID
col-id = VID:PID
col-speed = GESCHWINDIGKEIT
col-product = PRODUKT
col-driver = TREIBER
col-state = STATUS
col-name = NAME
col-fingerprint = FINGERABDRUCK
col-address = ADRESSE

## Discovery and peers

discover-searching = Lokales Netzwerk wird durchsucht …
discover-none = Es wurden keine USB-Nexus-Server gefunden.
discover-paired = gekoppelt
peers-empty = Noch keine gekoppelten Computer.
forget-ok = „{ $name }“ wurde entfernt.
forget-unknown = Kein gekoppelter Computer entspricht „{ $peer }“.

## Attaching

attach-connecting = Verbindung zu { $addr } wird aufgebaut …
attach-attached = { $busid } ist verbunden (virtueller Port { $port }).
attach-disconnected = Verbindung unterbrochen: { $reason }
attach-retrying = Erneute Verbindung in { $seconds } Sekunden …
attach-detached = Das Gerät wurde getrennt.
attach-stop-hint = Drücken Sie Strg+C, um die Verbindung zu trennen.

## Errors

err-pairing-required = „{ $name }“ ist noch nicht mit diesem Computer gekoppelt. Führen Sie aus: usbnexus pair { $target }
err-not-found = Der Server konnte im Netzwerk nicht gefunden werden.
err-version = Der Server läuft mit einer inkompatiblen Version von USB Nexus.
err-not-trusted = Dieser Computer ist nicht mit dem Server gekoppelt.
err-pairing-closed = Die Kopplung ist auf dem Server nicht geöffnet. Führen Sie auf dem Server aus: usbnexus pin
err-pairing-failed = Kopplung fehlgeschlagen. Überprüfen Sie die PIN und versuchen Sie es erneut.
err-no-such-device = Der Server gibt dieses Gerät nicht frei.
err-device-busy = Das Gerät wird von einem anderen Computer verwendet.
err-internal = Der Server hat einen internen Fehler gemeldet.
err-protocol = Unerwartete Antwort vom Server.
err-connection-lost = Die Verbindung zum Computer wurde unterbrochen.
err-unsupported = Dies ist auf diesem Betriebssystem noch nicht verfügbar.
err-driver-missing = Der usbip-win2-Treiber ist nicht installiert. Führen Sie das USB-Nexus-Setup erneut aus und akzeptieren Sie die Installation von usbip-win2.
err-driver-outdated = Der installierte usbip-win2-Treiber ist zu alt. Führen Sie das USB-Nexus-Setup erneut aus und akzeptieren Sie die Aktualisierung von usbip-win2.
err-vboxusb-missing = Die für die Freigabe benötigten VirtualBox-USB-Treiber sind nicht installiert. Installieren Sie USB Nexus neu.
err-device-in-use-by-os = Das Betriebssystem verwendet dieses Gerät mit seinem eigenen Treiber, daher kann es von diesem Computer nicht freigegeben werden.
err-unreachable = Der Computer konnte nicht erreicht werden. Prüfen Sie, ob er eingeschaltet und mit dem Netzwerk verbunden ist.
err-cancelled = Die Aktion wurde abgebrochen.
err-permission-denied = USB Nexus verfügt nicht über die erforderlichen Berechtigungen.
err-invalid = Die Anfrage wurde nicht verstanden.
err-other = Etwas ist schiefgelaufen: { $detail }

## Help layout

help-usage = Verwendung:
help-arguments = Argumente
help-options = Optionen
help-commands = Befehle
arg-help = Hilfe anzeigen.
arg-version = Version anzeigen.

## Peers

peers-servers = Gekoppelte Server (dieser Computer kann deren Geräte verwenden):
peers-clients = Gekoppelte Clients (dürfen die Geräte dieses Computers verwenden):

## Desktop app

gui-nav-this-computer = Dieser Computer
gui-nav-network = Computer im Netzwerk
gui-nav-connected = Verbundene Geräte
gui-nav-paired = Gekoppelte Computer
gui-language = Sprache
gui-this-title = Geräte auf diesem Computer
gui-this-subtitle = Wählen Sie, welche Geräte andere Computer verwenden dürfen.
gui-share = Freigeben
gui-shared = Freigegeben
gui-not-shared = Nicht freigegeben
gui-used-by = In Verwendung von { $name }
gui-no-local-devices = Auf diesem Computer wurden keine USB-Geräte gefunden.
gui-unnamed-device = USB-Gerät
gui-pair-new = Neuen Computer koppeln
gui-pin-title = Kopplungs-PIN
gui-pin-body = Geben Sie diese PIN auf dem anderen Computer ein.
gui-pin-remaining = Noch { $seconds } Sekunden gültig
gui-pin-expired = Die PIN ist abgelaufen.
gui-pin-new = Neue PIN
gui-pin-stop = Kopplung beenden
gui-close = Schließen
gui-cancel = Abbrechen
gui-network-title = Computer im Netzwerk
gui-network-subtitle = Im lokalen Netzwerk gefundene USB-Nexus-Computer.
gui-refresh = Aktualisieren
gui-searching = Netzwerk wird durchsucht …
gui-none-found = Es wurden keine Computer gefunden. Stellen Sie sicher, dass USB Nexus auf dem anderen Computer läuft, oder fügen Sie ihn über die Adresse hinzu.
gui-add-by-address = Über Adresse hinzufügen
gui-address = Adresse
gui-address-hint = z. B. 192.168.1.20
gui-pair = Koppeln
gui-paired = Gekoppelt
gui-pair-title = Mit { $name } koppeln
gui-pair-body = Wählen Sie auf { $name } „Neuen Computer koppeln“ und geben Sie die dort angezeigte PIN ein.
gui-pin = PIN
gui-pair-done = Mit { $name } gekoppelt.
gui-devices-of = Von { $name } freigegebene Geräte
gui-no-remote-devices = Dieser Computer gibt keine Geräte frei.
gui-connect = Verbinden
gui-disconnect = Trennen
gui-in-use-elsewhere = Wird von einem anderen Computer verwendet
gui-connected-here = Mit diesem Computer verbunden
gui-back = Zurück
gui-connected-title = Mit diesem Computer verbundene Geräte
gui-connected-subtitle = Entfernte Geräte bleiben verbunden und stellen die Verbindung bei Netzwerkausfall selbstständig wieder her.
gui-connected-empty = Es sind keine entfernten Geräte verbunden. Öffnen Sie „Netzwerk“, um eines zu verbinden.
gui-on-computer = auf { $name }
gui-state-connecting = Verbindung wird aufgebaut …
gui-state-attached = Verbunden
gui-state-retrying = Erneute Verbindung in { $seconds } s
gui-state-stopped = Getrennt
gui-state-failed = Fehlgeschlagen
gui-reconnect = Erneut verbinden
gui-remove = Entfernen
gui-paired-title = Gekoppelte Computer
gui-paired-subtitle = Computer werden einmal mit einer PIN gekoppelt und danach automatisch erkannt.
gui-paired-servers = Computer, deren Geräte Sie verwenden können
gui-paired-clients = Computer, die Ihre Geräte verwenden können
gui-paired-empty = Noch keine.
gui-remove-confirm = { $name } entfernen? Sie müssen erneut koppeln, um ihn wieder zu verwenden.
gui-fingerprint = Fingerabdruck
gui-service-denied-title = Dieser Benutzer darf den USB-Nexus-Dienst nicht steuern
gui-service-denied-body = Nur Administratoren und Mitglieder der Gruppe usbnexus dürfen das. Erlauben Sie diesem Benutzer den Zugriff (fragt nach dem Administratorkennwort), oder führen Sie als Administrator aus:
gui-service-denied-command = Befehl:
gui-service-denied-user = BENUTZER
gui-grant-access = Diesem Benutzer Zugriff erlauben
gui-grant-access-done = Zugriff gewährt.
gui-setup-kernel-modules = Für USB over IP benötigte Kernelmodule fehlen: { $modules }. Installieren Sie sie; der Dienst lädt sie danach selbstständig:
gui-setup-kernel-modules-nocmd = Sie sind im Kernel der meisten Distributionen enthalten (Ubuntu: linux-modules-extra, Fedora: kernel-modules-extra).
gui-service-down-title = Der USB-Nexus-Dienst läuft nicht
gui-service-down-body = Starten Sie den Dienst; dieses Fenster verbindet sich automatisch mit ihm.
gui-service-down-linux = Führen Sie unter Linux aus:
gui-service-down-windows = Führen Sie unter Windows als Administrator aus:
gui-service-down-macos = Führen Sie unter macOS aus:
gui-retry = Erneut versuchen
gui-details = Details
err-service-unavailable = Der USB-Nexus-Dienst konnte nicht erreicht werden.

## Web interface

gui-web-login-title = Bei { $name } anmelden
gui-web-password = Kennwort
gui-web-sign-in = Anmelden
gui-web-sign-out = Abmelden
gui-web-wrong-password = Falsches Kennwort.
gui-web-locked = Zu viele Versuche. Versuchen Sie es in { $seconds } Sekunden erneut.
cmd-web-about = Die Weboberfläche ein- oder ausschalten.
cmd-enable-about = Die Weboberfläche einschalten (fragt beim ersten Mal nach einem Kennwort).
cmd-disable-about = Die Weboberfläche ausschalten.
cmd-password-about = Das Kennwort der Weboberfläche ändern.
cmd-status-about = Anzeigen, ob die Weboberfläche eingeschaltet ist und wo.
arg-lan = Zugriff von anderen Computern im Netzwerk erlauben.
arg-port = TCP-Port der Weboberfläche.
web-on = Die Weboberfläche ist eingeschaltet:
web-off = Die Weboberfläche ist ausgeschaltet.
web-local-only = Nur dieser Computer kann sie öffnen. Verwenden Sie --lan, um andere Computer zuzulassen.
web-fingerprint = Der Browser warnt vor dem Zertifikat; sein Fingerabdruck sollte lauten: { $fp }
web-trusted = Browser auf diesem Computer vertrauen dem Zertifikat; andere Computer warnen davor. Sein Fingerabdruck lautet: { $fp }
web-not-running = Die Weboberfläche ist aktiviert, konnte aber nicht gestartet werden: { $detail }
web-password-prompt = Neues Kennwort für die Weboberfläche:
web-password-repeat = Kennwort wiederholen:
web-password-mismatch = Die Kennwörter stimmen nicht überein.
web-password-set = Das Kennwort der Weboberfläche wurde geändert.
err-weak-password = Das Kennwort muss mindestens 8 Zeichen lang sein.
err-port-in-use = Dieser Port wird von einem anderen Programm verwendet. Wählen Sie einen anderen Port.
err-password-required = Legen Sie zuerst ein Kennwort für die Weboberfläche fest: usbnexus web password
err-forbidden = Dies kann nur auf dem Computer selbst geändert werden.
err-not-logged-in = Bitte melden Sie sich erneut an.

## Hotplug, access control and usage log

arg-device = Gerät: Kennung oder Bus-ID (siehe: usbnexus list SERVER).
cmd-policy-about = Anzeigen oder festlegen, wer freigegebene Geräte verwenden darf.
arg-policy = open: jeder gekoppelte Computer darf jedes freigegebene Gerät verwenden; restricted: nur die pro Gerät erlaubten Computer.
arg-no-server = Die Freigabe der USB-Geräte dieses Computers nicht einrichten.
arg-no-client = Die Verwendung von USB-Geräten anderer Computer nicht einrichten.
arg-web = Weboberfläche: off, local (nur dieser Computer) oder network (das gesamte Netzwerk).
arg-web-port = Port der Weboberfläche (Standard 3242).
arg-web-password-file = Datei mit dem neuen Kennwort der Weboberfläche.
cmd-history-about = Das Nutzungsprotokoll der freigegebenen Geräte dieses Computers anzeigen.
arg-csv = Alle Einträge als CSV ausgeben (z. B. zum Speichern in einer Datei).
arg-limit = Anzahl der anzuzeigenden Einträge.
col-device-id = GERÄTE-ID
col-time = ZEIT (UTC)
col-event = EREIGNIS
col-computer = COMPUTER
col-duration = DAUER
state-unplugged = nicht angeschlossen
state-no-permission = keine Berechtigung
serve-denied = „{ $client }“ darf { $busid } nicht verwenden.
attach-waiting-device = Das Gerät ist auf dem Server nicht angeschlossen; es wird gewartet …
attach-queued = Ein anderer Computer verwendet das Gerät gerade; dieser Computer ist die Nummer { $position } in der Warteschlange.
policy-open = Jeder gekoppelte Computer darf jedes freigegebene Gerät verwenden (offen).
policy-restricted = Gekoppelte Computer dürfen nur die Geräte verwenden, für die sie berechtigt sind (eingeschränkt).
history-header = Nutzungsprotokoll (Einträge werden { $days } Tage aufbewahrt):
history-empty = Das Nutzungsprotokoll ist leer.
history-paired = gekoppelt
history-pairing-failed = falsche PIN
history-attached = begann Verwendung
history-detached = beendete Verwendung
history-denied = abgelehnt (keine Berechtigung)
err-access-denied = Dieser Computer darf das Gerät nicht verwenden.

gui-nav-history = Verlauf
gui-nav-settings = Einstellungen
gui-not-plugged-in = Nicht angeschlossen
gui-tracked-by-port = wird über Port verfolgt
gui-tracked-by-port-hint = Dieses Gerät hat keine Seriennummer und wird deshalb über den USB-Port erkannt, an dem es angeschlossen ist. Schließen Sie es wieder an denselben Port an.
gui-no-permission = Keine Berechtigung
gui-no-permission-hint = Der Besitzer dieses Computers hat diesem Computer nicht erlaubt, das Gerät zu verwenden.
gui-connect-when-plugged-in = Verbindet sich automatisch, sobald das Gerät angeschlossen wird.
gui-state-waiting-device = Warten auf das Gerät
gui-state-queued = Anderweitig in Verwendung; Nummer { $position } in der Warteschlange
gui-save = Speichern
gui-saved = Gespeichert.
gui-skip = Überspringen
gui-access-title = Wer dieses Gerät verwenden darf
gui-access-everyone = Alle gekoppelten Computer
gui-access-some = Ausgewählte Computer ({ $count })
gui-access-nobody = Noch kein Computer
gui-access-mode-default = Der Standardeinstellung folgen
gui-access-default-open = Derzeit: alle gekoppelten Computer.
gui-access-default-restricted = Derzeit: nur die unten ausgewählten Computer.
gui-access-mode-open = Alle gekoppelten Computer
gui-access-mode-open-body = Jeder mit diesem Computer gekoppelte Computer darf es verwenden.
gui-access-mode-selected = Nur ausgewählte Computer
gui-access-mode-selected-body = Nur die unten angehakten Computer dürfen es verwenden.
gui-access-computers = Computer, die es verwenden dürfen
gui-access-revoke-note = Ein Computer, der die Berechtigung verliert, wird sofort vom Gerät getrennt.
gui-client-devices = Geräte
gui-client-devices-title = Geräte, die { $name } verwenden darf
gui-client-devices-body = Wählen Sie die freigegebenen Geräte aus, die dieser Computer verwenden darf.
gui-client-devices-after-pairing = Nur berechtigte Computer dürfen freigegebene Geräte verwenden. Wählen Sie die Geräte aus, die dieser Computer verwenden darf; wenn Sie dies überspringen, kann er vorerst keines verwenden.
gui-no-shared-devices = Dieser Computer gibt noch keine Geräte frei.
gui-roles-title = Wie dieser Computer verwendet wird
gui-roles-body = Bildschirme für eine nicht gewählte Verwendung sind ausgeblendet. Beim Hinzufügen wird das Nötige installiert.
gui-role-server = Als Server verwenden (USB-Geräte dieses Computers freigeben)
gui-role-client = Als Client verwenden (USB-Geräte anderer Computer verwenden)
gui-roles-client-note = Falls nötig, wird der usbip-win2-Treiber installiert; USB-Geräte pausieren kurz, und Windows muss möglicherweise neu gestartet werden.
gui-roles-applying = Wird angewendet …
gui-reboot-required = Starten Sie den Computer neu, um die Einrichtung abzuschließen.
gui-used-by-waiting = { $name } verwendet es · { $count } warten
gui-col-permissions = Berechtigungen
gui-col-status = Status
gui-in-use-title = In Verwendung von
gui-nobody-using = Niemand verwendet das Gerät gerade.
gui-since = seit { $time }
gui-disconnect-user = Trennen
gui-disconnected-note = { $name } wurde getrennt. Wenn das Gerät weiterhin angefordert wird, verbindet sich der Computer innerhalb von Sekunden erneut; um ihn dauerhaft auszuschließen, deaktivieren Sie ihn in der Liste links und speichern Sie.
gui-queue-title = Wartend ({ $count })
gui-queue-empty = Niemand wartet.
gui-badge-using = in Verwendung
gui-badge-queued = wartet ({ $position })
gui-handover-title = Automatische Übergabe
gui-handover-default-on = Standard (ein, { $seconds } s)
gui-handover-default-off = Standard (aus)
gui-handover-on = Ein
gui-handover-off = Aus
gui-handover-before = Während ein anderer Computer wartet, geht es nach
gui-handover-after = Sekunden ohne Verwendung an den nächsten über.
gui-kind-storage = Speicher
gui-kind-input = Tastatur / Maus
gui-kind-printer = Drucker
gui-kind-dongle = Lizenz-Dongle
gui-kind-other = Anderes Gerät
gui-web-title = Weboberfläche
gui-web-body = Verwalten Sie diesen Computer über einen Browser, mit einem Kennwort.
gui-web-confirm-off = Die Weboberfläche ausschalten? Diese Seite funktioniert dann nicht mehr. Sie kann in der Desktop-App oder mit „usbnexus web enable“ auf dem Computer selbst wieder eingeschaltet werden.
gui-web-confirm-local = Zugriff nur von diesem Computer erlauben? Diese Seite wurde über das Netzwerk geöffnet und funktioniert danach nicht mehr.
gui-web-turned-off = Die Weboberfläche ist ausgeschaltet. Sie kann in der Desktop-App oder mit „usbnexus web enable“ auf dem Computer selbst wieder eingeschaltet werden.
gui-web-enabled = Weboberfläche eingeschaltet
gui-web-access-local = Nur dieser Computer
gui-web-access-network = Das gesamte Netzwerk
gui-web-port = Port
gui-web-new-password = Neues Kennwort
gui-web-repeat-password = Neues Kennwort (Wiederholung)
gui-web-password-keep = Mindestens 8 Zeichen. Leer lassen, um das aktuelle Kennwort zu behalten.
gui-web-password-required = Mindestens 8 Zeichen.
gui-web-mismatch = Die Kennwörter stimmen nicht überein.
gui-web-open-at = Öffnen unter:
gui-web-fingerprint = Der Browser warnt vor dem Zertifikat; sein Fingerabdruck ist { $fp }.
gui-web-trusted = Browser auf diesem Computer vertrauen dem Zertifikat; andere Computer warnen davor. Sein Fingerabdruck ist { $fp }.
gui-policy-title = Wer freigegebene Geräte verwenden darf
gui-policy-body = Computer müssen immer zuerst gekoppelt werden. Dies ist die Standardeinstellung für jedes freigegebene Gerät; jedes Gerät kann sie überschreiben.
gui-policy-first-title = Wer darf Ihre freigegebenen Geräte verwenden?
gui-policy-first-body = Computer müssen immer zuerst mit einer PIN gekoppelt werden. Wählen Sie, was gekoppelte Computer dürfen:
gui-policy-later = Sie können dies später in den Einstellungen ändern.
gui-policy-open = Jeder gekoppelte Computer
gui-policy-open-body = Jeder gekoppelte Computer darf jedes freigegebene Gerät verwenden.
gui-policy-restricted = Nur berechtigte Computer
gui-policy-restricted-body = Sie wählen pro Gerät aus, welche gekoppelten Computer es verwenden dürfen.
gui-retention-title = Nutzungsprotokoll
gui-retention-body = Kopplungen, falsche PINs, Gerätenutzung und abgelehnte Anfragen werden protokolliert. Ältere Einträge werden automatisch gelöscht.
gui-retention-days = Einträge aufbewahren für (Tage)
gui-settings-title = Einstellungen
gui-startup-title = Autostart
gui-startup-body = Beim Schließen des Fensters bleibt USB Nexus im Infobereich; der Dienst setzt Freigaben und Verbindungen trotzdem fort.
gui-startup-enabled = Automatisch starten, wenn ich mich anmelde
gui-tray-open = USB Nexus öffnen
gui-tray-quit = Beenden
gui-history-title = Verlauf
gui-history-subtitle = Wer die Geräte dieses Computers verwendet hat, und wann. Einträge werden { $days } Tage aufbewahrt.
gui-history-empty = Es wurde noch nichts aufgezeichnet.
gui-history-export = CSV exportieren
gui-history-time = Zeit
gui-history-event = Ereignis
gui-history-computer = Computer
gui-history-device = Gerät
gui-history-device-id = Geräte-ID
gui-history-duration = Dauer
gui-history-paired = Gekoppelt
gui-history-pairing-failed = Falsche PIN
gui-history-attached = Verwendung begonnen
gui-history-detached = Verwendung beendet
gui-history-denied = Abgelehnt: keine Berechtigung

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = Wie werden Sie USB Nexus verwenden?
setup-roles-subtitle = Wählen Sie, was dieser Computer tun soll.
setup-role-server = Als Server verwenden
setup-role-client = Als Client verwenden
setup-role-web = Webzugriff
setup-usbip-install-note = Der usbip-win2-Treiber wird ebenfalls installiert. USB-Geräte pausieren während der Installation kurz, und Windows muss danach neu gestartet werden.
setup-usbip-update-note = Der installierte usbip-win2-Treiber ist zu alt und wird aktualisiert. USB-Geräte pausieren während der Installation kurz, und Windows muss danach neu gestartet werden.
setup-usbip-present-note = Der usbip-win2-Treiber ist auf diesem Computer bereits installiert.
setup-usbip-failed = Der usbip-win2-Treiber konnte nicht installiert werden. Sie können das USB-Nexus-Setup später erneut ausführen, um es noch einmal zu versuchen.
setup-service-failed = Der USB-Nexus-Dienst konnte nicht eingerichtet werden. Details finden Sie im Installationsprotokoll.
setup-web-title = Weboberfläche
setup-web-subtitle = Einstellungen für die Verwaltung dieses Computers über einen Browser.
setup-web-access = Zugriff:
setup-web-local = Nur dieser Computer
setup-web-network = Das gesamte Netzwerk
setup-web-port = Port:
setup-web-port-free = ✓ Der Port ist verfügbar
setup-web-port-busy = ✗ Dieser Port wird von einem anderen Programm verwendet
setup-web-port-invalid = ✗ Geben Sie eine Zahl zwischen 1 und 65535 ein
setup-web-password = Kennwort:
setup-web-password-repeat = Kennwort (Wiederholung):
setup-web-password-hint = Mindestens 8 Zeichen.
setup-web-password-keep = Mindestens 8 Zeichen. Leer lassen, um das aktuelle Kennwort zu behalten.
