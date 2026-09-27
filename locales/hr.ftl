# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Croatian.
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = Siguran USB preko IP-a: dijeljenje USB uređaja putem mreže uz šifriranje, uparivanje i automatsko ponovno povezivanje.
arg-lang = Jezik sučelja (npr. en, tr). Zadano: jezik sustava.
arg-state-dir = Mapa za ključeve i uparena računala.
arg-name = Naziv ovog računala kako ga vide drugi.
arg-verbose = Prikaži detaljne poruke dnevnika.

cmd-daemon-about = Pokreni USB Nexus uslugu (koristi ju aplikacija za radnu površinu).
arg-allow-all-users = Dopusti svim lokalnim korisnicima upravljanje uslugom (zadano: članovi grupe usbnexus).
arg-socket = Putanja lokalne kontrolne priključnice usluge.
cmd-service-about = Instaliraj ili ukloni USB Nexus Windows uslugu.
cmd-install-about = Instaliraj uslugu, pokreni ju odmah i pri svakom pokretanju sustava (pokrenite kao administrator).
cmd-uninstall-about = Zaustavi i ukloni uslugu.
cmd-serve-about = Podijeli USB uređaje ovog računala.
arg-export = Bus ID uređaja za dijeljenje (može se navesti više puta). Pogledajte: usbnexus local
arg-listen = Adresa i port za slušanje.
arg-pair = Otvori prozor za uparivanje pri pokretanju i prikaži PIN.
arg-no-mdns = Ne oglašavaj ovaj poslužitelj na lokalnoj mreži.

cmd-pin-about = Prikaži PIN za uparivanje novog računala s pokrenutim poslužiteljem.
arg-seconds = Koliko dugo PIN vrijedi, u sekundama.

cmd-local-about = Prikaži popis USB uređaja priključenih na ovo računalo.
cmd-discover-about = Pronađi USB Nexus poslužitelje na lokalnoj mreži.
arg-timeout = Koliko dugo pretraživati, u sekundama.
cmd-pair-about = Uparite se s poslužiteljem pomoću PIN-a koji on prikazuje.
arg-server = Poslužitelj: upareni naziv, otisak (fingerprint) ili host[:port].
arg-pin = PIN koji prikazuje poslužitelj (bit će zatražen ako se ne navede).
cmd-list-about = Prikaži popis uređaja koje dijeli poslužitelj.
cmd-allow-user-about = Dopusti korisniku upravljanje uslugom iz aplikacije za radnu površinu (dodaje ga u grupu usbnexus).
arg-user = Korisničko ime.
allow-user-invalid = „{ $user }” nije valjano korisničko ime.
allow-user-failed = Nije moguće dodati „{ $user }” u grupu usbnexus.
allow-user-done = „{ $user }” sada može upravljati USB Nexus uslugom.
allow-user-relogin = Primjenjuje se nakon ponovne prijave.
cmd-attach-about = Koristi udaljeni uređaj na ovom računalu; automatski se ponovno povezuje.
arg-busid = Bus ID udaljenog uređaja.
cmd-peers-about = Prikaži popis uparenih računala.
cmd-forget-about = Ukloni upareno računalo.
arg-peer = Naziv ili otisak (fingerprint) uparenog računala.

## General

error-prefix = Pogreška: { $detail }
hint-root = Za ovu radnju potrebna su administratorska ovlaštenja. Pokušajte ponovno s sudo.
unsupported-os = Ova naredba još nije podržana na ovom operacijskom sustavu.
yes = da
no = ne

service-installed = USB Nexus usluga je instalirana i pokrenuta.
service-removed = USB Nexus usluga je uklonjena.

## Server

serve-started = Poslužitelj „{ $name }” sluša na { $addr }.
serve-fingerprint = Otisak: { $fp }
serve-exporting = Dijeljeni uređaji:
serve-no-exports = Nijedan uređaj nije podijeljen. Dodajte --export BUSID (popis uređaja: usbnexus local).
serve-pairing-pin = PIN za uparivanje: { $pin } (vrijedi { $seconds } sekundi)
serve-paired = Uparen s „{ $name }”.
serve-pairing-failed = Neuspješan pokušaj uparivanja s adrese { $addr }.
serve-exported = { $busid } sada koristi „{ $client }”.
serve-released = { $busid } je oslobodio „{ $client }”.
serve-stopping = Zaustavljanje; uređaji se vraćaju svojim normalnim upravljačkim programima…
serve-mdns-failed = Otkrivanje na lokalnoj mreži nije dostupno: { $detail }
serve-bind-failed = Nije moguće pripremiti { $busid } za dijeljenje: { $detail }

## Pairing

pin-show = PIN za uparivanje: { $pin }
pin-hint = Na drugom računalu pokrenite „usbnexus pair { $name }” u sljedećih { $seconds } sekundi.
pin-no-server = Nije pronađen nijedan pokrenuti USB Nexus poslužitelj. Pokrenite jedan naredbom: usbnexus serve
pair-enter-pin = Unesite PIN prikazan na poslužitelju:
pair-ok = Uparen s „{ $name }” ({ $fp }).
pair-already = Već uparen s „{ $name }”.

## Devices

local-header = USB uređaji na ovom računalu:
local-empty = Nije pronađen nijedan USB uređaj.
list-header = Uređaji koje dijeli „{ $name }”:
list-empty = Ovaj poslužitelj ne dijeli nijedan uređaj.
list-in-use = u upotrebi
col-busid = BUS ID
col-id = VID:PID
col-speed = BRZINA
col-product = PROIZVOD
col-driver = UPRAVLJAČKI PROGRAM
col-state = STANJE
col-name = NAZIV
col-fingerprint = OTISAK
col-address = ADRESA

## Discovery and peers

discover-searching = Pretraživanje lokalne mreže…
discover-none = Nije pronađen nijedan USB Nexus poslužitelj.
discover-paired = upareno
peers-empty = Još nema uparenih računala.
forget-ok = „{ $name }” je uklonjen.
forget-unknown = Nijedno upareno računalo ne odgovara nazivu „{ $peer }”.

## Attaching

attach-connecting = Povezivanje s { $addr }…
attach-attached = { $busid } je povezan (virtualni port { $port }).
attach-disconnected = Veza je izgubljena: { $reason }
attach-retrying = Ponovno povezivanje za { $seconds } sekundi…
attach-detached = Uređaj je isključen.
attach-stop-hint = Pritisnite Ctrl+C za isključivanje uređaja.

## Errors

err-pairing-required = „{ $name }” još nije uparen s ovim računalom. Pokrenite: usbnexus pair { $target }
err-not-found = Poslužitelj nije pronađen na mreži.
err-version = Poslužitelj koristi nekompatibilnu verziju USB Nexusa.
err-not-trusted = Ovo računalo nije upareno s poslužiteljem.
err-pairing-closed = Uparivanje nije otvoreno na poslužitelju. Na poslužitelju pokrenite: usbnexus pin
err-pairing-failed = Uparivanje nije uspjelo. Provjerite PIN i pokušajte ponovno.
err-no-such-device = Poslužitelj ne dijeli ovaj uređaj.
err-device-busy = Uređaj koristi drugo računalo.
err-internal = Poslužitelj je prijavio internu pogrešku.
err-protocol = Neočekivan odgovor poslužitelja.
err-connection-lost = Veza s računalom je izgubljena.
err-unsupported = Ovo još nije dostupno na ovom operacijskom sustavu.
err-driver-missing = Upravljački program usbip-win2 nije instaliran. Ponovno pokrenite instalaciju USB Nexusa i prihvatite instaliranje usbip-win2.
err-driver-outdated = Instalirani upravljački program usbip-win2 je previše zastario. Ponovno pokrenite instalaciju USB Nexusa i prihvatite ažuriranje usbip-win2.
err-vboxusb-missing = VirtualBox USB upravljački programi potrebni za dijeljenje nisu instalirani. Ponovno instalirajte USB Nexus.
err-device-in-use-by-os = Operacijski sustav koristi ovaj uređaj svojim vlastitim upravljačkim programom, pa se ne može dijeliti s ovog računala.
err-unreachable = Računalo nije dostupno. Provjerite je li uključeno i povezano na mrežu.
err-cancelled = Radnja je otkazana.
err-permission-denied = USB Nexus nema potrebna ovlaštenja.
err-invalid = Zahtjev nije razumljiv.
err-other = Nešto je pošlo po zlu: { $detail }

## Help layout

help-usage = Upotreba:
help-arguments = Argumenti
help-options = Mogućnosti
help-commands = Naredbe
arg-help = Prikaži pomoć.
arg-version = Prikaži verziju.

## Peers

peers-servers = Upareni poslužitelji (ovo računalo može koristiti njihove uređaje):
peers-clients = Upareni klijenti (mogu koristiti uređaje ovog računala):

## Desktop app

gui-nav-this-computer = Ovo računalo
gui-nav-network = Računala na mreži
gui-nav-connected = Povezani uređaji
gui-nav-paired = Uparena računala
gui-language = Jezik
gui-this-title = Uređaji na ovom računalu
gui-this-subtitle = Odaberite koje uređaje mogu koristiti druga računala.
gui-share = Podijeli
gui-shared = Podijeljeno
gui-not-shared = Nije podijeljeno
gui-used-by = Koristi { $name }
gui-no-local-devices = Nije pronađen nijedan USB uređaj na ovom računalu.
gui-unnamed-device = USB uređaj
gui-pair-new = Upari novo računalo
gui-pin-title = PIN za uparivanje
gui-pin-body = Unesite ovaj PIN na drugom računalu.
gui-pin-remaining = Vrijedi još { $seconds } sekundi
gui-pin-expired = PIN je istekao.
gui-pin-new = Novi PIN
gui-pin-stop = Zaustavi uparivanje
gui-close = Zatvori
gui-cancel = Odustani
gui-network-title = Računala na mreži
gui-network-subtitle = USB Nexus računala pronađena na vašoj lokalnoj mreži.
gui-refresh = Osvježi
gui-searching = Pretraživanje mreže…
gui-none-found = Nije pronađeno nijedno računalo. Provjerite je li USB Nexus pokrenut na drugom računalu, ili ga dodajte po adresi.
gui-add-by-address = Dodaj po adresi
gui-address = Adresa
gui-address-hint = npr. 192.168.1.20
gui-pair = Upari
gui-paired = Upareno
gui-pair-title = Uparivanje s { $name }
gui-pair-body = Na { $name } odaberite „Upari novo računalo” i unesite PIN koji je tamo prikazan.
gui-pin = PIN
gui-pair-done = Uparen s { $name }.
gui-devices-of = Uređaji koje dijeli { $name }
gui-no-remote-devices = Ovo računalo ne dijeli nijedan uređaj.
gui-connect = Poveži
gui-disconnect = Prekini veze
gui-in-use-elsewhere = Koristi ga drugo računalo
gui-connected-here = Povezan s ovim računalom
gui-back = Natrag
gui-connected-title = Uređaji povezani s ovim računalom
gui-connected-subtitle = Udaljeni uređaji ostaju povezani i sami se ponovno povezuju ako mreža prekine veze.
gui-connected-empty = Nema povezanih udaljenih uređaja. Otvorite „Mreža” za povezivanje uređaja.
gui-on-computer = na { $name }
gui-state-connecting = Povezivanje…
gui-state-attached = Povezan
gui-state-retrying = Ponovno povezivanje za { $seconds } s
gui-state-stopped = Nije povezan
gui-state-failed = Neuspješno
gui-reconnect = Ponovno poveži
gui-remove = Ukloni
gui-paired-title = Uparena računala
gui-paired-subtitle = Računala se uparuju jednom, PIN-om, a nakon toga se automatski prepoznaju.
gui-paired-servers = Računala čije uređaje možete koristiti
gui-paired-clients = Računala koja mogu koristiti vaše uređaje
gui-paired-empty = Još nema.
gui-remove-confirm = Uklonite { $name }? Za ponovnu upotrebu bit će potrebno ponovno se upariti.
gui-fingerprint = Otisak
gui-service-denied-title = Ovaj korisnik ne može upravljati USB Nexus uslugom
gui-service-denied-body = To mogu samo administratori i članovi grupe usbnexus. Dopustite ovom korisniku (zatražit će se administratorska lozinka) ili pokrenite kao administrator:
gui-service-denied-command = Naredba:
gui-service-denied-user = KORISNIK
gui-grant-access = Dopusti ovom korisniku
gui-grant-access-done = Pristup je dodijeljen.
gui-setup-kernel-modules = Nedostaju kernel moduli potrebni za USB preko IP-a: { $modules }. Instalirajte ih; usluga će ih zatim sama učitati:
gui-setup-kernel-modules-nocmd = Dolaze uz kernel većine distribucija (Ubuntu: linux-modules-extra, Fedora: kernel-modules-extra).
gui-service-down-title = USB Nexus usluga nije pokrenuta
gui-service-down-body = Pokrenite uslugu; ovaj prozor se automatski povezuje s njom.
gui-service-down-linux = Na Linuxu pokrenite:
gui-service-down-windows = Na Windowsu, kao administrator, pokrenite:
gui-service-down-macos = Na macOS-u pokrenite:
gui-retry = Pokušaj ponovno
gui-details = Pojedinosti
err-service-unavailable = USB Nexus usluga nije dostupna.

## Web interface

gui-web-login-title = Prijava na { $name }
gui-web-password = Lozinka
gui-web-sign-in = Prijava
gui-web-sign-out = Odjava
gui-web-wrong-password = Netočna lozinka.
gui-web-locked = Previše pokušaja. Pokušajte ponovno za { $seconds } sekundi.
cmd-web-about = Uključi ili isključi web sučelje.
cmd-enable-about = Uključi web sučelje (prvi put traži lozinku).
cmd-disable-about = Isključi web sučelje.
cmd-password-about = Promijeni lozinku web sučelja.
cmd-status-about = Prikaži je li web sučelje uključeno i gdje.
arg-lan = Dopusti pristup s drugih računala na mreži.
arg-port = TCP port web sučelja.
web-on = Web sučelje je uključeno:
web-off = Web sučelje je isključeno.
web-local-only = Samo ovo računalo može ga otvoriti. Koristite --lan za dopuštanje pristupa drugim računalima.
web-fingerprint = Preglednik će upozoriti na certifikat; njegov otisak treba biti: { $fp }
web-trusted = Preglednici na ovom računalu vjeruju certifikatu; druga računala će prikazati upozorenje. Njegov otisak je: { $fp }
web-not-running = Web sučelje je uključeno, ali se nije pokrenulo: { $detail }
web-password-prompt = Nova lozinka web sučelja:
web-password-repeat = Ponovite lozinku:
web-password-mismatch = Lozinke se ne podudaraju.
web-password-set = Lozinka web sučelja je promijenjena.
err-weak-password = Lozinka mora imati najmanje 8 znakova.
err-port-in-use = Ovaj port koristi drugi program. Odaberite drugi port.
err-password-required = Najprije postavite lozinku web sučelja: usbnexus web password
err-forbidden = Ovo se može promijeniti samo na samom računalu.
err-not-logged-in = Prijavite se ponovno.

## Hotplug, access control and usage log

arg-device = Uređaj: identitet ili bus id (pogledajte: usbnexus list SERVER).
cmd-policy-about = Prikaži ili odaberi tko može koristiti podijeljene uređaje.
arg-policy = open: svako upareno računalo može koristiti svaki podijeljeni uređaj; restricted: samo računala dopuštena po uređaju.
arg-no-server = Nemoj postaviti dijeljenje USB uređaja ovog računala.
arg-no-client = Nemoj postaviti korištenje USB uređaja drugih računala.
arg-web = Web sučelje: off (isključeno), local (samo ovo računalo) ili network (cijela mreža).
arg-web-port = Port web sučelja (zadano 3242).
arg-web-password-file = Datoteka koja sadrži novu lozinku web sučelja.
cmd-history-about = Prikaži dnevnik korištenja podijeljenih uređaja ovog računala.
arg-csv = Ispiši sve stavke u CSV formatu (npr. za spremanje u datoteku).
arg-limit = Broj stavki za prikaz.
cmd-log-about = Prikaži ili promijeni koliko detaljno usluga zapisuje dnevnik.
arg-level = info (zadano), debug ili trace. Odmah se primjenjuje; više od info služi za traženje problema.
col-device-id = ID UREĐAJA
col-time = VRIJEME (UTC)
col-event = DOGAĐAJ
col-computer = RAČUNALO
col-duration = TRAJANJE
state-unplugged = nije priključen
state-no-permission = nema dozvolu
serve-denied = „{ $client }” nema dozvolu koristiti { $busid }.
attach-waiting-device = Uređaj nije priključen na poslužitelju; čeka se…
attach-queued = Drugo računalo koristi uređaj; ovo je računalo { $position }. u redu čekanja.
policy-open = Svako upareno računalo može koristiti svaki podijeljeni uređaj (otvoreno).
policy-restricted = Uparena računala mogu koristiti samo uređaje za koje imaju dozvolu (ograničeno).
log-level = Razina dnevnika: { $level }
history-header = Dnevnik korištenja (stavke se čuvaju { $days } dana):
history-empty = Dnevnik korištenja je prazan.
history-paired = upareno
history-pairing-failed = netočan PIN
history-attached = počelo korištenje
history-detached = prestalo korištenje
history-denied = odbijeno (nema dozvolu)
err-access-denied = Ovo računalo nema dozvolu koristiti uređaj.

gui-nav-history = Povijest
gui-nav-settings = Postavke
gui-not-plugged-in = Nije priključen
gui-tracked-by-port = praćen po portu
gui-tracked-by-port-hint = Ovaj uređaj nema serijski broj, pa se prepoznaje po USB portu na koji je priključen. Priključite ga ponovno na isti port.
gui-no-permission = Nema dozvolu
gui-no-permission-hint = Vlasnik tog računala nije dopustio ovom računalu korištenje uređaja.
gui-connect-when-plugged-in = Povezuje se automatski čim se uređaj priključi.
gui-state-waiting-device = Čekanje na uređaj
gui-state-queued = U upotrebi na drugom mjestu; { $position }. u redu čekanja
gui-save = Spremi
gui-saved = Spremljeno.
gui-skip = Preskoči
gui-access-title = Tko može koristiti ovaj uređaj
gui-access-everyone = Sva uparena računala
gui-access-some = Odabrana računala ({ $count })
gui-access-nobody = Još nema računala
gui-access-mode-default = Slijedi zadanu postavku
gui-access-default-open = Trenutno: sva uparena računala.
gui-access-default-restricted = Trenutno: samo računala odabrana ispod.
gui-access-mode-open = Sva uparena računala
gui-access-mode-open-body = Svako računalo upareno s ovim može ga koristiti.
gui-access-mode-selected = Samo odabrana računala
gui-access-mode-selected-body = Samo računala označena ispod mogu ga koristiti.
gui-access-computers = Računala kojima je dopušteno korištenje
gui-access-revoke-note = Računalu kojemu se oduzme dozvola veza s uređajem se odmah prekida.
gui-client-devices = Uređaji
gui-client-devices-title = Uređaji koje { $name } može koristiti
gui-client-devices-body = Označite podijeljene uređaje koje ovo računalo može koristiti.
gui-client-devices-after-pairing = Podijeljene uređaje mogu koristiti samo dopuštena računala. Označite uređaje koje ovo računalo može koristiti; ako ovo preskočite, zasad ne može koristiti nijedan.
gui-no-shared-devices = Ovo računalo još ne dijeli nijedan uređaj.
gui-roles-title = Kako se ovo računalo koristi
gui-roles-body = Zasloni za način korištenja koji nije odabran su skriveni. Dodavanjem se instalira ono što je potrebno.
gui-role-server = Koristi kao poslužitelj (dijeli USB uređaje ovog računala)
gui-role-client = Koristi kao klijent (koristi USB uređaje drugih računala)
gui-roles-client-note = Ako je potrebno, instalira se upravljački program usbip-win2; USB uređaji se prekidaju na nekoliko sekundi, a možda će biti potrebno ponovno pokrenuti Windows.
gui-roles-applying = Primjenjivanje…
gui-reboot-required = Ponovno pokrenite računalo da biste dovršili postavljanje.
gui-used-by-waiting = { $name } koristi uređaj · { $count } čeka
gui-col-permissions = Dozvole
gui-col-status = Status
gui-in-use-title = Trenutno koristi
gui-nobody-using = Trenutno nitko ne koristi uređaj.
gui-since = od { $time }
gui-disconnect-user = Prekini veze
gui-disconnected-note = { $name } je odspojen. Ako i dalje zahtijeva uređaj, ponovno se povezuje za nekoliko sekundi; da ga trajno onemogućite, uklonite oznaku na popisu lijevo i spremite.
gui-queue-title = Čekanje u redu ({ $count })
gui-queue-empty = Nitko ne čeka.
gui-badge-using = koristi
gui-badge-queued = u redu ({ $position })
gui-handover-title = Automatska predaja
gui-handover-default-on = Zadano (uključeno, { $seconds } s)
gui-handover-default-off = Zadano (isključeno)
gui-handover-on = Uključeno
gui-handover-off = Isključeno
gui-handover-before = Dok drugo računalo čeka, nakon
gui-handover-after = sekundi bez korištenja prelazi na sljedeće.
gui-kind-storage = Skladište
gui-kind-input = Tipkovnica / miš
gui-kind-printer = Pisač
gui-kind-dongle = Licencni dongle
gui-kind-other = Ostali uređaji
gui-web-title = Web sučelje
gui-web-body = Upravljajte ovim računalom iz preglednika, lozinkom.
gui-web-confirm-off = Isključiti web sučelje? Ova stranica će prestati raditi. Može se ponovno uključiti u aplikaciji za radnu površinu ili naredbom „usbnexus web enable” na samom računalu.
gui-web-confirm-local = Dopustiti pristup samo s ovog računala? Ova stranica je otvorena putem mreže i prestat će raditi.
gui-web-turned-off = Web sučelje je isključeno. Može se ponovno uključiti u aplikaciji za radnu površinu ili naredbom „usbnexus web enable” na samom računalu.
gui-web-enabled = Web sučelje uključeno
gui-web-access-local = Samo ovo računalo
gui-web-access-network = Cijela mreža
gui-web-port = Port
gui-web-new-password = Nova lozinka
gui-web-repeat-password = Nova lozinka (ponovo)
gui-web-password-keep = Najmanje 8 znakova. Ostavite prazno za zadržavanje trenutne lozinke.
gui-web-password-required = Najmanje 8 znakova.
gui-web-mismatch = Lozinke se ne podudaraju.
gui-web-open-at = Otvori na:
gui-web-fingerprint = Preglednik upozorava na certifikat; njegov otisak je { $fp }.
gui-web-trusted = Preglednici na ovom računalu vjeruju certifikatu; druga računala upozoravaju na njega. Njegov otisak je { $fp }.
gui-policy-title = Tko može koristiti podijeljene uređaje
gui-policy-body = Računala uvijek prvo moraju biti uparena. Ovo je zadana postavka za svaki podijeljeni uređaj; svaki uređaj ju može zaobići.
gui-policy-first-title = Tko može koristiti vaše podijeljene uređaje?
gui-policy-first-body = Računala uvijek prvo moraju biti uparena PIN-om. Odaberite što uparena računala mogu raditi:
gui-policy-later = Ovo možete promijeniti kasnije u Postavkama.
gui-policy-open = Svako upareno računalo
gui-policy-open-body = Svako upareno računalo može koristiti svaki podijeljeni uređaj.
gui-policy-restricted = Samo dopuštena računala
gui-policy-restricted-body = Za svaki uređaj sami birate koja uparena računala ga mogu koristiti.
gui-retention-title = Dnevnik korištenja
gui-retention-body = Uparivanja, netočni PIN-ovi, korištenje uređaja i odbijeni zahtjevi se bilježe. Starije stavke se automatski brišu.
gui-retention-days = Čuvaj stavke (dana)
gui-settings-title = Postavke
gui-startup-title = Pokretanje
gui-startup-body = Zatvaranje prozora zadržava USB Nexus u području obavijesti; usluga i dalje nastavlja dijeljenje uređaja i održavanje veza.
gui-startup-enabled = Pokreni automatski pri prijavi
gui-tray-open = Otvori USB Nexus
gui-tray-quit = Izlaz
gui-history-title = Povijest
gui-history-subtitle = Tko je i kada koristio uređaje ovog računala. Stavke se čuvaju { $days } dana.
gui-history-empty = Još ništa nije zabilježeno.
gui-history-export = Izvezi kao CSV
gui-history-time = Vrijeme
gui-history-event = Događaj
gui-history-computer = Računalo
gui-history-device = Uređaj
gui-history-device-id = ID uređaja
gui-history-duration = Trajanje
gui-history-paired = Upareno
gui-history-pairing-failed = Netočan PIN
gui-history-attached = Počelo korištenje
gui-history-detached = Prestalo korištenje
gui-history-denied = Odbijeno: nema dozvolu

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = Kako ćete koristiti USB Nexus?
setup-roles-subtitle = Odaberite što će ovo računalo raditi.
setup-role-server = Koristi kao poslužitelj
setup-role-client = Koristi kao klijent
setup-role-web = Web pristup
setup-usbip-install-note = Bit će instaliran i upravljački program usbip-win2. USB uređaji se prekidaju na nekoliko sekundi tijekom instalacije, a Windows nakon toga mora biti ponovno pokrenut.
setup-usbip-update-note = Instalirani upravljački program usbip-win2 je zastario i bit će ažuriran. USB uređaji se prekidaju na nekoliko sekundi tijekom instalacije, a Windows nakon toga mora biti ponovno pokrenut.
setup-usbip-present-note = Upravljački program usbip-win2 je već instaliran na ovom računalu.
setup-usbip-failed = Upravljački program usbip-win2 nije moguće instalirati. Kasnije možete ponovno pokrenuti instalaciju USB Nexusa za novi pokušaj.
setup-service-failed = USB Nexus uslugu nije moguće postaviti. Pojedinosti se nalaze u dnevniku instalacije.
setup-web-title = Web sučelje
setup-web-subtitle = Postavke za upravljanje ovim računalom iz preglednika.
setup-web-access = Pristup:
setup-web-local = Samo ovo računalo
setup-web-network = Cijela mreža
setup-web-port = Port:
setup-web-port-free = ✓ Port je slobodan
setup-web-port-busy = ✗ Ovaj port koristi drugi program
setup-web-port-invalid = ✗ Unesite broj između 1 i 65535
setup-web-password = Lozinka:
setup-web-password-repeat = Lozinka (ponovo):
setup-web-password-hint = Najmanje 8 znakova.
setup-web-password-keep = Najmanje 8 znakova. Ostavite prazno za zadržavanje trenutne lozinke.
