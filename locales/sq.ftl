# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Albanian.

## Command line help

app-about = USB-over-IP i sigurt: ndani pajisje USB në rrjet me shifrim, çiftim dhe rilidhje automatike.
arg-lang = Gjuha e ndërfaqes (p.sh. en, tr). Parazgjedhja: gjuha e sistemit.
arg-state-dir = Dosja për çelësat dhe kompjuterët e çiftuar.
arg-name = Emri i këtij kompjuteri siç u shfaqet të tjerëve.
arg-verbose = Shfaq mesazhe të hollësishme në regjistër.

cmd-daemon-about = Ekzekuto shërbimin USB Nexus (përdoret nga aplikacioni desktop).
arg-allow-all-users = Lejo çdo përdorues lokal të kontrollojë shërbimin (parazgjedhja: anëtarët e grupit usbnexus).
arg-socket = Shtegu i soketit lokal të kontrollit të shërbimit.
cmd-service-about = Instalo ose hiq shërbimin USB Nexus për Windows.
cmd-install-about = Instalo shërbimin, nise tani dhe në çdo nisje (ekzekutojeni si administrator).
cmd-uninstall-about = Ndalo dhe hiq shërbimin.
cmd-serve-about = Ndaj pajisjet USB të këtij kompjuteri.
arg-export = Bus id-ja e një pajisjeje për t'u ndarë (mund të përsëritet). Shih: usbnexus local
arg-listen = Adresa dhe porti për të dëgjuar.
arg-pair = Hap një dritare çiftimi në nisje dhe shfaq PIN-in.
arg-no-mdns = Mos e njofto këtë server në rrjetin lokal.

cmd-pin-about = Shfaq një PIN për të çiftuar një kompjuter të ri me serverin që po xhiron.
arg-seconds = Sa kohë mbetet i vlefshëm PIN-i, në sekonda.

cmd-local-about = Liston pajisjet USB të lidhura me këtë kompjuter.
cmd-discover-about = Gjej serverat USB Nexus në rrjetin lokal.
arg-timeout = Sa kohë të kërkojë, në sekonda.
cmd-pair-about = Çiftohu me një server duke përdorur PIN-in që ai shfaq.
arg-server = Server: emri i çiftuar, fingerprint-i, ose host[:port].
arg-pin = PIN-i i shfaqur nga serveri (nëse mungon, kërkohet).
cmd-list-about = Liston pajisjet e ndara nga një server.
cmd-allow-user-about = Lejo një përdorues të kontrollojë shërbimin nga aplikacioni desktop (e shton në grupin usbnexus).
arg-user = Emri i përdoruesit.
allow-user-invalid = “{ $user }” nuk është një emër përdoruesi i vlefshëm.
allow-user-failed = “{ $user }” nuk mund të shtohej në grupin usbnexus.
allow-user-done = “{ $user }” tani mund të kontrollojë shërbimin USB Nexus.
allow-user-relogin = Zbatohet pasi të hyni përsëri në sistem.
cmd-attach-about = Përdor një pajisje të largët në këtë kompjuter; rilidhet automatikisht.
arg-busid = Bus id-ja e pajisjes së largët.
cmd-peers-about = Liston kompjuterat e çiftuar.
cmd-forget-about = Hiq një kompjuter të çiftuar.
arg-peer = Emri ose fingerprint-i i kompjuterit të çiftuar.

## General

error-prefix = Gabim: { $detail }
hint-root = Kjo veprim kërkon të drejta administratori. Provoni sërish me sudo.
unsupported-os = Kjo komandë nuk mbështetet ende në këtë sistem operativ.
yes = po
no = jo

service-installed = Shërbimi USB Nexus u instalua dhe u nis.
service-removed = Shërbimi USB Nexus u hoq.

## Server

serve-started = Serveri “{ $name }” po dëgjon në { $addr }.
serve-fingerprint = Fingerprint: { $fp }
serve-exporting = Pajisje të ndara:
serve-no-exports = Nuk ndahet asnjë pajisje. Shtoni --export BUSID (listoni pajisjet me: usbnexus local).
serve-pairing-pin = PIN-i i çiftimit: { $pin } (i vlefshëm për { $seconds } sekonda)
serve-paired = U çiftua me “{ $name }”.
serve-pairing-failed = Përpjekje çiftimi e dështuar nga { $addr }.
serve-exported = { $busid } tani përdoret nga “{ $client }”.
serve-released = { $busid } u lirua nga “{ $client }”.
serve-stopping = Po ndalon; pajisjet po kthehen te drejtuesit e tyre normalë…
serve-mdns-failed = Zbulimi në rrjetin lokal nuk është i disponueshëm: { $detail }
serve-bind-failed = Nuk u përgatit { $busid } për ndarje: { $detail }

## Pairing

pin-show = PIN-i i çiftimit: { $pin }
pin-hint = Në kompjuterin tjetër ekzekutoni “usbnexus pair { $name }” brenda { $seconds } sekondave.
pin-no-server = Nuk u gjet asnjë server USB Nexus që po xhiron. Nisni një me: usbnexus serve
pair-enter-pin = Shkruani PIN-in e shfaqur nga serveri:
pair-ok = U çiftua me “{ $name }” ({ $fp }).
pair-already = Tashmë i çiftuar me “{ $name }”.

## Devices

local-header = Pajisjet USB në këtë kompjuter:
local-empty = Nuk u gjet asnjë pajisje USB.
list-header = Pajisjet e ndara nga “{ $name }”:
list-empty = Ky server nuk ndan asnjë pajisje.
list-in-use = në përdorim
col-busid = BUS ID
col-id = VID:PID
col-speed = SHPEJTËSIA
col-product = PRODUKTI
col-driver = DRIVERI
col-state = GJENDJA
col-name = EMRI
col-fingerprint = FINGERPRINT
col-address = ADRESA

## Discovery and peers

discover-searching = Duke kërkuar në rrjetin lokal…
discover-none = Nuk u gjet asnjë server USB Nexus.
discover-paired = i çiftuar
peers-empty = Ende nuk ka kompjuter të çiftuar.
forget-ok = “{ $name }” u hoq.
forget-unknown = Nuk ka kompjuter të çiftuar që përputhet me “{ $peer }”.

## Attaching

attach-connecting = Duke u lidhur me { $addr }…
attach-attached = { $busid } është i lidhur (porti virtual { $port }).
attach-disconnected = Lidhja u ndërpre: { $reason }
attach-retrying = Rilidhje pas { $seconds } sekondash…
attach-detached = Pajisja u shkëput.
attach-stop-hint = Shtypni Ctrl+C për ta shkëputur.

## Errors

err-pairing-required = “{ $name }” nuk është ende i çiftuar me këtë kompjuter. Ekzekutoni: usbnexus pair { $target }
err-not-found = Serveri nuk u gjet në rrjet.
err-version = Serveri po xhiron një version të papërputhshëm të USB Nexus.
err-not-trusted = Ky kompjuter nuk është i çiftuar me serverin.
err-pairing-closed = Çiftimi nuk është hapur në server. Në server ekzekutoni: usbnexus pin
err-pairing-failed = Çiftimi dështoi. Kontrolloni PIN-in dhe provoni sërish.
err-no-such-device = Serveri nuk e ndan këtë pajisje.
err-device-busy = Pajisja po përdoret nga një kompjuter tjetër.
err-internal = Serveri raportoi një gabim të brendshëm.
err-protocol = Përgjigje e papritur nga serveri.
err-connection-lost = Lidhja me kompjuterin u ndërpre.
err-unsupported = Kjo nuk është ende e disponueshme në këtë sistem operativ.
err-driver-missing = Drejtuesi usbip-win2 nuk është i instaluar. Ekzekutoni sërish instalimin e USB Nexus dhe pranoni instalimin e usbip-win2.
err-driver-outdated = Drejtuesi usbip-win2 i instaluar është shumë i vjetër. Ekzekutoni sërish instalimin e USB Nexus dhe pranoni përditësimin e usbip-win2.
err-vboxusb-missing = Drejtuesit USB të VirtualBox të nevojshëm për ndarje nuk janë të instaluar. Riinstaloni USB Nexus.
err-device-in-use-by-os = Sistemi operativ po e përdor këtë pajisje me drejtuesin e vet, kështu që nuk mund të ndahet nga ky kompjuter.
err-unreachable = Kompjuteri nuk mund të kontaktohej. Kontrolloni që të jetë i ndezur dhe i lidhur me rrjetin.
err-cancelled = Veprimi u anulua.
err-permission-denied = USB Nexus nuk ka lejet e nevojshme.
err-invalid = Kërkesa nuk u kuptua.
err-other = Ndodhi një problem: { $detail }

## Help layout

help-usage = Përdorimi:
help-arguments = Argumente
help-options = Opsione
help-commands = Komanda
arg-help = Shfaq ndihmën.
arg-version = Shfaq versionin.

## Peers

peers-servers = Serverat e çiftuar (ky kompjuter mund të përdorë pajisjet e tyre):
peers-clients = Klientët e çiftuar (të lejuar të përdorin pajisjet e këtij kompjuteri):

## Desktop app

gui-nav-this-computer = Ky kompjuter
gui-nav-network = Kompjuterat në rrjet
gui-nav-connected = Pajisje të lidhura
gui-nav-paired = Kompjuterat e çiftuar
gui-language = Gjuha
gui-this-title = Pajisjet në këtë kompjuter
gui-this-subtitle = Zgjidhni cilat pajisje mund t'i përdorin kompjuterat e tjerë.
gui-share = Ndaj
gui-shared = E ndarë
gui-not-shared = E pandarë
gui-used-by = Në përdorim nga { $name }
gui-no-local-devices = Nuk u gjet asnjë pajisje USB në këtë kompjuter.
gui-unnamed-device = Pajisje USB
gui-pair-new = Çifto një kompjuter të ri
gui-pin-title = PIN-i i çiftimit
gui-pin-body = Shkruani këtë PIN në kompjuterin tjetër.
gui-pin-remaining = I vlefshëm për { $seconds } sekonda të tjera
gui-pin-expired = PIN-i ka skaduar.
gui-pin-new = PIN i ri
gui-pin-stop = Ndalo çiftimin
gui-close = Mbyll
gui-cancel = Anulo
gui-network-title = Kompjuterat në rrjet
gui-network-subtitle = Kompjuterat USB Nexus të gjetur në rrjetin tuaj lokal.
gui-refresh = Rifresko
gui-searching = Duke kërkuar në rrjet…
gui-none-found = Nuk u gjet asnjë kompjuter. Sigurohuni që USB Nexus po xhiron në kompjuterin tjetër, ose shtojeni me adresë.
gui-add-by-address = Shto me adresë
gui-address = Adresa
gui-address-hint = p.sh. 192.168.1.20
gui-pair = Çifto
gui-paired = I çiftuar
gui-pair-title = Çiftohu me { $name }
gui-pair-body = Në { $name }, zgjidhni “Çifto një kompjuter të ri” dhe shkruani PIN-in e shfaqur atje.
gui-pin = PIN
gui-pair-done = U çiftua me { $name }.
gui-devices-of = Pajisjet e ndara nga { $name }
gui-no-remote-devices = Ky kompjuter nuk ndan asnjë pajisje.
gui-connect = Lidhu
gui-disconnect = Shkëputu
gui-in-use-elsewhere = Në përdorim nga një kompjuter tjetër
gui-connected-here = I lidhur me këtë kompjuter
gui-back = Mbrapa
gui-connected-title = Pajisjet e lidhura me këtë kompjuter
gui-connected-subtitle = Pajisjet e largëta mbeten të lidhura dhe rilidhen vetë nëse rrjeti bie.
gui-connected-empty = Nuk ka pajisje të largëta të lidhura. Hapni “Rrjeti” për të lidhur një.
gui-on-computer = në { $name }
gui-state-connecting = Duke u lidhur…
gui-state-attached = I lidhur
gui-state-retrying = Rilidhje pas { $seconds } sek
gui-state-stopped = I shkëputur
gui-state-failed = Dështoi
gui-reconnect = Rilidhu
gui-remove = Hiq
gui-paired-title = Kompjuterat e çiftuar
gui-paired-subtitle = Kompjuterat çiftohen një herë me PIN dhe njihen automatikisht pas kësaj.
gui-paired-servers = Kompjuterat pajisjet e të cilëve mund t'i përdorni
gui-paired-clients = Kompjuterat që mund të përdorin pajisjet tuaja
gui-paired-empty = Ende asnjë.
gui-remove-confirm = Të hiqet { $name }? Do t'ju duhet të çiftoheni sërish për ta përdorur.
gui-fingerprint = Fingerprint
gui-service-denied-title = Ky përdorues nuk mund të kontrollojë shërbimin USB Nexus
gui-service-denied-body = Vetëm administratorët dhe anëtarët e grupit usbnexus mund ta bëjnë këtë. Lejoni këtë përdorues (kërkon fjalëkalimin e administratorit), ose ekzekutoni si administrator:
gui-service-denied-command = Komanda:
gui-service-denied-user = PËRDORUESI
gui-grant-access = Lejo këtë përdorues
gui-grant-access-done = Leja u dha.
gui-setup-kernel-modules = Mungojnë modulet e kernelit të nevojshme për USB over IP: { $modules }. Instalojini; shërbimi i ngarkon vetë:
gui-setup-kernel-modules-nocmd = Ato vijnë me kernelin e shumicës së shpërndarjeve (Ubuntu: linux-modules-extra, Fedora: kernel-modules-extra).
gui-service-down-title = Shërbimi USB Nexus nuk po xhiron
gui-service-down-body = Nisni shërbimin; kjo dritare lidhet me të automatikisht.
gui-service-down-linux = Në Linux ekzekutoni:
gui-service-down-windows = Në Windows, si administrator, ekzekutoni:
gui-service-down-macos = Në macOS ekzekutoni:
gui-retry = Provo sërish
gui-details = Detaje
err-service-unavailable = Shërbimi USB Nexus nuk mund të kontaktohej.

## Web interface

gui-web-login-title = Hyni në { $name }
gui-web-password = Fjalëkalimi
gui-web-sign-in = Hyr
gui-web-sign-out = Dil
gui-web-wrong-password = Fjalëkalim i gabuar.
gui-web-locked = Shumë përpjekje. Provoni sërish pas { $seconds } sekondash.
cmd-web-about = Ndiz ose fik ndërfaqen web.
cmd-enable-about = Ndiz ndërfaqen web (kërkon fjalëkalim herën e parë).
cmd-disable-about = Fik ndërfaqen web.
cmd-password-about = Ndrysho fjalëkalimin e ndërfaqes web.
cmd-status-about = Shfaq nëse ndërfaqja web është ndezur dhe ku.
arg-lan = Lejo qasje nga kompjuterat e tjerë në rrjet.
arg-port = Porti TCP i ndërfaqes web.
web-on = Ndërfaqja web është ndezur:
web-off = Ndërfaqja web është fikur.
web-local-only = Vetëm ky kompjuter mund ta hapë. Përdorni --lan për të lejuar kompjuterat e tjerë.
web-fingerprint = Shfletuesi do të paralajmërojë për certifikatën; fingerprint-i i saj duhet të jetë: { $fp }
web-trusted = Shfletuesit në këtë kompjuter i besojnë certifikatës; kompjuterat e tjerë do të paralajmërojnë. Fingerprint-i i saj është: { $fp }
web-not-running = Ndërfaqja web është e aktivizuar, por nuk mund të nisej: { $detail }
web-password-prompt = Fjalëkalimi i ri i ndërfaqes web:
web-password-repeat = Përsëritni fjalëkalimin:
web-password-mismatch = Fjalëkalimet nuk përputhen.
web-password-set = Fjalëkalimi i ndërfaqes web u ndryshua.
err-weak-password = Fjalëkalimi duhet të ketë të paktën 8 shenja.
err-port-in-use = Ky port përdoret nga një program tjetër. Zgjidhni një port tjetër.
err-password-required = Vendosni së pari një fjalëkalim për ndërfaqen web: usbnexus web password
err-forbidden = Kjo mund të ndryshohet vetëm nga kompjuteri vetë.
err-not-logged-in = Ju lutemi, hyni sërish.

## Hotplug, access control and usage log

arg-device = Pajisja: identiteti ose bus id-ja (shih: usbnexus list SERVER).
cmd-policy-about = Shfaq ose zgjidh kush mund t'i përdorë pajisjet e ndara.
arg-policy = open: çdo kompjuter i çiftuar mund të përdorë çdo pajisje të ndarë; restricted: vetëm kompjuterat e lejuar për secilën pajisje.
arg-no-server = Mos e konfiguro ndarjen e pajisjeve USB të këtij kompjuteri.
arg-no-client = Mos e konfiguro përdorimin e pajisjeve USB të kompjuterave të tjerë.
arg-web = Ndërfaqja web: off (fikur), local (vetëm ky kompjuter) ose network (i gjithë rrjeti).
arg-web-port = Porti i ndërfaqes web (parazgjedhja 3242).
arg-web-password-file = Skedari që përmban fjalëkalimin e ri të ndërfaqes web.
cmd-history-about = Shfaq regjistrin e përdorimit të pajisjeve të ndara të këtij kompjuteri.
arg-csv = Shtyp të gjitha hyrjet si CSV (p.sh. për t'i ruajtur në një skedar).
arg-limit = Numri i hyrjeve për t'u shfaqur.
cmd-log-about = Shfaq ose ndrysho sa hollësisht mban regjistër shërbimi.
arg-level = info (parazgjedhja), debug, ose trace. Hyn në fuqi menjëherë; mbi info përdoret për të gjetur probleme.
col-device-id = ID E PAJISJES
col-time = KOHA (UTC)
col-event = NGJARJA
col-computer = KOMPJUTERI
col-duration = KOHËZGJATJA
state-unplugged = e palidhur
state-no-permission = pa leje
serve-denied = “{ $client }” nuk lejohet ta përdorë { $busid }.
attach-waiting-device = Pajisja nuk është e lidhur në server; duke pritur…
attach-queued = Një kompjuter tjetër po e përdor pajisjen; ky është numri { $position } në radhë.
policy-open = Çdo kompjuter i çiftuar mund të përdorë çdo pajisje të ndarë (hapur).
policy-restricted = Kompjuterat e çiftuar mund të përdorin vetëm pajisjet që u lejohet (i kufizuar).
log-level = Niveli i regjistrit: { $level }
history-header = Regjistri i përdorimit (hyrjet ruhen për { $days } ditë):
history-empty = Regjistri i përdorimit është bosh.
history-paired = u çiftua
history-pairing-failed = PIN i gabuar
history-attached = filloi ta përdorë
history-detached = ndaloi së përdoruri
history-denied = u refuzua (pa leje)
err-access-denied = Ky kompjuter nuk lejohet të përdorë pajisjen.

gui-nav-history = Historiku
gui-nav-settings = Cilësimet
gui-not-plugged-in = E palidhur
gui-tracked-by-port = ndiqet sipas portit
gui-tracked-by-port-hint = Kjo pajisje nuk ka numër serie, kështu njihet nga porti USB në të cilin është lidhur. Lidheni sërish në të njëjtin port.
gui-no-permission = Pa leje
gui-no-permission-hint = Pronari i atij kompjuteri nuk e ka lejuar këtë kompjuter të përdorë pajisjen.
gui-connect-when-plugged-in = Lidhet automatikisht sapo pajisja të lidhet.
gui-state-waiting-device = Duke pritur pajisjen
gui-state-queued = Në përdorim gjetkë; numri { $position } në radhë
gui-save = Ruaj
gui-saved = U ruajt.
gui-skip = Kapërce
gui-access-title = Kush mund ta përdorë këtë pajisje
gui-access-everyone = Të gjithë kompjuterat e çiftuar
gui-access-some = Kompjuterat e zgjedhur ({ $count })
gui-access-nobody = Ende asnjë kompjuter
gui-access-mode-default = Ndiq cilësimin e parazgjedhur
gui-access-default-open = Aktualisht: të gjithë kompjuterat e çiftuar.
gui-access-default-restricted = Aktualisht: vetëm kompjuterat e zgjedhur më poshtë.
gui-access-mode-open = Të gjithë kompjuterat e çiftuar
gui-access-mode-open-body = Çdo kompjuter i çiftuar me këtë mund ta përdorë.
gui-access-mode-selected = Vetëm kompjuterat e zgjedhur
gui-access-mode-selected-body = Vetëm kompjuterat e shënuar më poshtë mund ta përdorin.
gui-access-computers = Kompjuterat e lejuar ta përdorin
gui-access-revoke-note = Një kompjuter që humbet lejen shkëputet menjëherë nga pajisja.
gui-client-devices = Pajisjet
gui-client-devices-title = Pajisjet që { $name } mund t'i përdorë
gui-client-devices-body = Shënoni pajisjet e ndara që ky kompjuter mund t'i përdorë.
gui-client-devices-after-pairing = Pajisjet e ndara mund t'i përdorin vetëm kompjuterat e lejuar. Shënoni pajisjet që ky kompjuter mund t'i përdorë; nëse e kapërceni këtë, ai nuk mund të përdorë asnjë për tani.
gui-no-shared-devices = Ky kompjuter nuk ndan ende asnjë pajisje.
gui-roles-title = Si përdoret ky kompjuter
gui-roles-body = Ekranet për një përdorim që nuk është zgjedhur janë të fshehura. Shtimi i njërit instalon çfarë i nevojitet.
gui-role-server = Përdore si server (ndaj pajisjet USB të këtij kompjuteri)
gui-role-client = Përdore si klient (përdor pajisjet USB të kompjuterave të tjerë)
gui-roles-client-note = Nëse nevojitet, instalohet drejtuesi usbip-win2; pajisjet USB ndalojnë për disa sekonda dhe Windows mund të duhet të rinisë.
gui-roles-applying = Duke zbatuar…
gui-reboot-required = Rinisni kompjuterin për të përfunduar konfigurimin.
gui-used-by-waiting = { $name } po e përdor · { $count } duke pritur
gui-col-permissions = Lejet
gui-col-status = Statusi
gui-in-use-title = Në përdorim nga
gui-nobody-using = Askush nuk e po e përdor pajisjen tani.
gui-since = prej { $time }
gui-disconnect-user = Shkëpute
gui-disconnected-note = { $name } u shkëput. Nëse vazhdon ta kërkojë pajisjen, rilidhet brenda disa sekondash; për ta mbajtur larg përgjithmonë, hiqjani shenjën në listën majtas dhe ruajeni.
gui-queue-title = Në radhë ({ $count })
gui-queue-empty = Askush nuk pret.
gui-badge-using = duke e përdorur
gui-badge-queued = në radhë ({ $position })
gui-handover-title = Kalimi automatik
gui-handover-default-on = Parazgjedhje (aktiv, { $seconds } sek)
gui-handover-default-off = Parazgjedhje (joaktiv)
gui-handover-on = Aktiv
gui-handover-off = Joaktiv
gui-handover-before = Kur një kompjuter tjetër pret, pas
gui-handover-after = sekondave pa përdorim i kalon radhës tjetër.
gui-kind-storage = Ruajtje
gui-kind-input = Tastierë / mi
gui-kind-printer = Printer
gui-kind-dongle = Dongle licence
gui-kind-other = Pajisje tjetër
gui-web-title = Ndërfaqja web
gui-web-body = Menaxhoni këtë kompjuter nga një shfletues, me fjalëkalim.
gui-web-confirm-off = Të fiket ndërfaqja web? Kjo faqe do të ndalojë të punojë. Mund të ndizet sërish nga aplikacioni desktop ose me “usbnexus web enable” në kompjuterin vetë.
gui-web-confirm-local = Të lejohet qasja vetëm nga ky kompjuter? Kjo faqe u hap nga rrjeti dhe do të ndalojë të punojë.
gui-web-turned-off = Ndërfaqja web është fikur. Mund të ndizet sërish nga aplikacioni desktop ose me “usbnexus web enable” në kompjuterin vetë.
gui-web-enabled = Ndërfaqja web e ndezur
gui-web-access-local = Vetëm ky kompjuter
gui-web-access-network = I gjithë rrjeti
gui-web-port = Porti
gui-web-new-password = Fjalëkalimi i ri
gui-web-repeat-password = Fjalëkalimi i ri (përsëri)
gui-web-password-keep = Të paktën 8 shenja. Lëreni bosh për të ruajtur fjalëkalimin aktual.
gui-web-password-required = Të paktën 8 shenja.
gui-web-mismatch = Fjalëkalimet nuk përputhen.
gui-web-open-at = Hape në:
gui-web-fingerprint = Shfletuesi paralajmëron për certifikatën; fingerprint-i i saj është { $fp }.
gui-web-trusted = Shfletuesit në këtë kompjuter i besojnë certifikatës; kompjuterat e tjerë paralajmërojnë. Fingerprint-i i saj është { $fp }.
gui-policy-title = Kush mund t'i përdorë pajisjet e ndara
gui-policy-body = Kompjuterat duhet gjithmonë të çiftohen fillimisht. Ky është parazgjedhja për çdo pajisje të ndarë; çdo pajisje mund ta anashkalojë.
gui-policy-first-title = Kush mund t'i përdorë pajisjet tuaja të ndara?
gui-policy-first-body = Kompjuterat duhet gjithmonë të çiftohen fillimisht me PIN. Zgjidhni çfarë mund të bëjnë kompjuterat e çiftuar:
gui-policy-later = Këtë mund ta ndryshoni më vonë te Cilësimet.
gui-policy-open = Çdo kompjuter i çiftuar
gui-policy-open-body = Çdo kompjuter i çiftuar mund të përdorë çdo pajisje të ndarë.
gui-policy-restricted = Vetëm kompjuterat e lejuar
gui-policy-restricted-body = Ju zgjidhni, për secilën pajisje, cilët kompjuterat e çiftuar mund ta përdorin.
gui-retention-title = Regjistri i përdorimit
gui-retention-body = Çiftimet, PIN-et e gabuar, përdorimi i pajisjeve dhe kërkesat e refuzuara regjistrohen. Hyrjet e vjetra fshihen automatikisht.
gui-retention-days = Ruaj hyrjet për (ditë)
gui-settings-title = Cilësimet
gui-startup-title = Nisja
gui-startup-body = Mbyllja e dritares e mban USB Nexus në zonën e njoftimeve; shërbimi vazhdon ndarjet dhe lidhjet gjithsesi.
gui-startup-enabled = Nise automatikisht kur hyj në sistem
gui-tray-open = Hap USB Nexus
gui-tray-quit = Mbyll
gui-history-title = Historiku
gui-history-subtitle = Kush i ka përdorur pajisjet e këtij kompjuteri dhe kur. Hyrjet ruhen për { $days } ditë.
gui-history-empty = Ende nuk është regjistruar asgjë.
gui-history-export = Eksporto CSV
gui-history-time = Koha
gui-history-event = Ngjarja
gui-history-computer = Kompjuteri
gui-history-device = Pajisja
gui-history-device-id = ID e pajisjes
gui-history-duration = Kohëzgjatja
gui-history-paired = U çiftua
gui-history-pairing-failed = PIN i gabuar
gui-history-attached = Filloi t'e përdorë
gui-history-detached = Ndaloi t'e përdorë
gui-history-denied = Refuzuar: pa leje

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = Si do të përdorni USB Nexus?
setup-roles-subtitle = Zgjidhni çfarë do të bëjë ky kompjuter.
setup-role-server = Përdore si server
setup-role-client = Përdore si klient
setup-role-web = Qasje nga web
setup-usbip-install-note = Do të instalohet gjithashtu drejtuesi usbip-win2. Pajisjet USB ndalojnë për disa sekonda gjatë instalimit, dhe Windows duhet të rinisë pas kësaj.
setup-usbip-update-note = Drejtuesi usbip-win2 i instaluar është shumë i vjetër dhe do të përditësohet. Pajisjet USB ndalojnë për disa sekonda gjatë instalimit, dhe Windows duhet të rinisë pas kësaj.
setup-usbip-present-note = Drejtuesi usbip-win2 është tashmë i instaluar në këtë kompjuter.
setup-usbip-failed = Drejtuesi usbip-win2 nuk mund të instalohej. Mund të ekzekutoni sërish instalimin e USB Nexus më vonë për të provuar përsëri.
setup-service-failed = Shërbimi USB Nexus nuk mund të konfigurohej. Detajet janë në regjistrin e instalimit.
setup-web-title = Ndërfaqja web
setup-web-subtitle = Cilësimet për menaxhimin e këtij kompjuteri nga një shfletues.
setup-web-access = Qasja:
setup-web-local = Vetëm ky kompjuter
setup-web-network = I gjithë rrjeti
setup-web-port = Porti:
setup-web-port-free = ✓ Porti është i disponueshëm
setup-web-port-busy = ✗ Ky port përdoret nga një program tjetër
setup-web-port-invalid = ✗ Shkruani një numër midis 1 dhe 65535
setup-web-password = Fjalëkalimi:
setup-web-password-repeat = Fjalëkalimi (përsëri):
setup-web-password-hint = Të paktën 8 shenja.
setup-web-password-keep = Të paktën 8 shenja. Lëreni bosh për të ruajtur fjalëkalimin aktual.
