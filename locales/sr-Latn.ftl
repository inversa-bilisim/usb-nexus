# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Serbian (Latin).
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = Bezbedan USB preko IP-a: deljenje USB uređaja preko mreže sa šifrovanjem, uparivanjem i automatskim ponovnim povezivanjem.
arg-lang = Jezik interfejsa (npr. en, tr). Podrazumevano: jezik sistema.
arg-state-dir = Direktorijum za ključeve i uparene računare.
arg-name = Naziv ovog računara koji vide drugi.
arg-verbose = Prikaži detaljne poruke dnevnika.

cmd-daemon-about = Pokreni USB Nexus servis (koristi ga desktop aplikacija).
arg-allow-all-users = Dozvoli svim lokalnim korisnicima da upravljaju servisom (podrazumevano: članovi grupe usbnexus).
arg-socket = Putanja lokalnog kontrolnog soketa servisa.
cmd-service-about = Instaliraj ili ukloni USB Nexus Windows servis.
cmd-install-about = Instaliraj servis, pokreni ga odmah i pri svakom pokretanju sistema (pokrenite kao administrator).
cmd-uninstall-about = Zaustavi i ukloni servis.
cmd-serve-about = Deli USB uređaje ovog računara.
arg-export = Bus ID uređaja za deljenje (može se navesti više puta). Vidi: usbnexus local
arg-listen = Adresa i port za slušanje.
arg-pair = Otvori prozor za uparivanje pri pokretanju i prikaži PIN.
arg-no-mdns = Ne oglašavaj ovaj server na lokalnoj mreži.

cmd-pin-about = Prikaži PIN za uparivanje novog računara sa aktivnim serverom.
arg-seconds = Koliko dugo PIN važi, u sekundama.

cmd-local-about = Prikaži listu USB uređaja povezanih na ovaj računar.
cmd-discover-about = Pronađi USB Nexus servere na lokalnoj mreži.
arg-timeout = Koliko dugo tražiti, u sekundama.
cmd-pair-about = Uparite se sa serverom pomoću PIN-a koji on prikazuje.
arg-server = Server: uparen naziv, otisak (fingerprint) ili host[:port].
arg-pin = PIN koji prikazuje server (biće zatražen ako se ne navede).
cmd-list-about = Prikaži listu uređaja koje deli server.
cmd-allow-user-about = Dozvoli korisniku da upravlja servisom iz desktop aplikacije (dodaje ga u grupu usbnexus).
arg-user = Korisničko ime.
allow-user-invalid = „{ $user }” nije važeće korisničko ime.
allow-user-failed = Nije moguće dodati „{ $user }” u grupu usbnexus.
allow-user-done = „{ $user }” sada može upravljati USB Nexus servisom.
allow-user-relogin = Primenjuje se nakon ponovne prijave.
cmd-attach-about = Koristi udaljeni uređaj na ovom računaru; automatski se ponovo povezuje.
arg-busid = Bus ID udaljenog uređaja.
cmd-peers-about = Prikaži listu uparenih računara.
cmd-forget-about = Ukloni uparen računar.
arg-peer = Naziv ili otisak (fingerprint) uparenog računara.

## General

error-prefix = Greška: { $detail }
hint-root = Za ovu radnju su potrebna administratorska ovlašćenja. Pokušajte ponovo sa sudo.
unsupported-os = Ova komanda još nije podržana na ovom operativnom sistemu.
yes = da
no = ne

service-installed = USB Nexus servis je instaliran i pokrenut.
service-removed = USB Nexus servis je uklonjen.

## Server

serve-started = Server „{ $name }” slušа na { $addr }.
serve-fingerprint = Otisak: { $fp }
serve-exporting = Deljeni uređaji:
serve-no-exports = Nijedan uređaj nije deljen. Dodajte --export BUSID (lista uređaja: usbnexus local).
serve-pairing-pin = PIN za uparivanje: { $pin } (važi { $seconds } sekundi)
serve-paired = Uparen sa „{ $name }”.
serve-pairing-failed = Neuspešan pokušaj uparivanja sa adrese { $addr }.
serve-exported = { $busid } sada koristi „{ $client }”.
serve-released = { $busid } je oslobodio „{ $client }”.
serve-stopping = Zaustavljanje; uređaji se vraćaju svojim normalnim upravljačkim programima…
serve-mdns-failed = Otkrivanje na lokalnoj mreži nije dostupno: { $detail }
serve-bind-failed = Nije moguće pripremiti { $busid } za deljenje: { $detail }

## Pairing

pin-show = PIN za uparivanje: { $pin }
pin-hint = Na drugom računaru pokrenite „usbnexus pair { $name }” u naredne { $seconds } sekunde.
pin-no-server = Nije pronađen nijedan aktivan USB Nexus server. Pokrenite jedan komandom: usbnexus serve
pair-enter-pin = Unesite PIN prikazan na serveru:
pair-ok = Uparen sa „{ $name }” ({ $fp }).
pair-already = Već uparen sa „{ $name }”.

## Devices

local-header = USB uređaji na ovom računaru:
local-empty = Nije pronađen nijedan USB uređaj.
list-header = Uređaji koje deli „{ $name }”:
list-empty = Ovaj server ne deli nijedan uređaj.
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

discover-searching = Pretraga lokalne mreže…
discover-none = Nije pronađen nijedan USB Nexus server.
discover-paired = uparen
peers-empty = Još nema uparenih računara.
forget-ok = „{ $name }” je uklonjen.
forget-unknown = Nijedan uparen računar ne odgovara nazivu „{ $peer }”.

## Attaching

attach-connecting = Povezivanje sa { $addr }…
attach-attached = { $busid } je povezan (virtuelni port { $port }).
attach-disconnected = Veza je izgubljena: { $reason }
attach-retrying = Ponovno povezivanje za { $seconds } sekundi…
attach-detached = Uređaj je isključen.
attach-stop-hint = Pritisnite Ctrl+C da isključite uređaj.

## Errors

err-pairing-required = „{ $name }” još nije uparen sa ovim računarom. Pokrenite: usbnexus pair { $target }
err-not-found = Server nije pronađen na mreži.
err-version = Server koristi nekompatibilnu verziju USB Nexus-a.
err-not-trusted = Ovaj računar nije uparen sa serverom.
err-pairing-closed = Uparivanje nije otvoreno na serveru. Na serveru pokrenite: usbnexus pin
err-pairing-failed = Uparivanje nije uspelo. Provjerite PIN i pokušajte ponovo.
err-no-such-device = Server ne deli ovaj uređaj.
err-device-busy = Uređaj koristi drugi računar.
err-internal = Server je prijavio internu grešku.
err-protocol = Neočekivan odgovor servera.
err-connection-lost = Veza sa računarom je izgubljena.
err-unsupported = Ovo još nije dostupno na ovom operativnom sistemu.
err-driver-missing = Upravljački program usbip-win2 nije instaliran. Ponovo pokrenite instalaciju USB Nexus-a i prihvatite instalaciju usbip-win2.
err-driver-outdated = Instalirani upravljački program usbip-win2 je previše star. Ponovo pokrenite instalaciju USB Nexus-a i prihvatite ažuriranje usbip-win2.
err-vboxusb-missing = VirtualBox USB upravljački programi potrebni za deljenje nisu instalirani. Ponovo instalirajte USB Nexus.
err-device-in-use-by-os = Operativni sistem koristi ovaj uređaj sopstvenim upravljačkim programom, tako da se ne može deliti sa ovog računara.
err-unreachable = Računar nije dostupan. Provjerite da li je uključen i povezan na mrežu.
err-cancelled = Radnja je otkazana.
err-permission-denied = USB Nexus nema potrebna ovlašćenja.
err-invalid = Zahtev nije razumljiv.
err-other = Nešto je pošlo po zlu: { $detail }

## Help layout

help-usage = Upotreba:
help-arguments = Argumenti
help-options = Opcije
help-commands = Komande
arg-help = Prikaži pomoć.
arg-version = Prikaži verziju.

## Peers

peers-servers = Upareni serveri (ovaj računar može koristiti njihove uređaje):
peers-clients = Upareni klijenti (mogu koristiti uređaje ovog računara):

## Desktop app

gui-nav-this-computer = Ovaj računar
gui-nav-network = Računari na mreži
gui-nav-connected = Povezani uređaji
gui-nav-paired = Upareni računari
gui-language = Jezik
gui-this-title = Uređaji na ovom računaru
gui-this-subtitle = Izaberite koje uređaje mogu koristiti drugi računari.
gui-share = Deli
gui-shared = Deljen
gui-not-shared = Nije deljen
gui-used-by = Koristi { $name }
gui-no-local-devices = Nije pronađen nijedan USB uređaj na ovom računaru.
gui-unnamed-device = USB uređaj
gui-pair-new = Uparite novi računar
gui-pin-title = PIN za uparivanje
gui-pin-body = Unesite ovaj PIN na drugom računaru.
gui-pin-remaining = Važi još { $seconds } sekundi
gui-pin-expired = PIN je istekao.
gui-pin-new = Novi PIN
gui-pin-stop = Zaustavi uparivanje
gui-close = Zatvori
gui-cancel = Otkaži
gui-network-title = Računari na mreži
gui-network-subtitle = USB Nexus računari pronađeni na vašoj lokalnoj mreži.
gui-refresh = Osveži
gui-searching = Pretraga mreže…
gui-none-found = Nije pronađen nijedan računar. Provjerite da li je USB Nexus pokrenut na drugom računaru, ili ga dodajte po adresi.
gui-add-by-address = Dodaj po adresi
gui-address = Adresa
gui-address-hint = npr. 192.168.1.20
gui-pair = Uparite
gui-paired = Uparen
gui-pair-title = Uparivanje sa { $name }
gui-pair-body = Na { $name } izaberite „Uparite novi računar” i unesite PIN koji se tamo prikazuje.
gui-pin = PIN
gui-pair-done = Uparen sa { $name }.
gui-devices-of = Uređaji koje deli { $name }
gui-no-remote-devices = Ovaj računar ne deli nijedan uređaj.
gui-connect = Poveži
gui-disconnect = Prekini vezu
gui-in-use-elsewhere = Koristi ga drugi računar
gui-connected-here = Povezan sa ovim računarom
gui-back = Nazad
gui-connected-title = Uređaji povezani sa ovim računarom
gui-connected-subtitle = Udaljeni uređaji ostaju povezani i sami se ponovo povezuju ako mreža prekine vezu.
gui-connected-empty = Nema povezanih udaljenih uređaja. Otvorite „Mreža” da povežete jedan.
gui-on-computer = na { $name }
gui-state-connecting = Povezivanje…
gui-state-attached = Povezan
gui-state-retrying = Ponovno povezivanje za { $seconds } s
gui-state-stopped = Nije povezan
gui-state-failed = Neuspešno
gui-reconnect = Ponovo poveži
gui-remove = Ukloni
gui-paired-title = Upareni računari
gui-paired-subtitle = Računari se uparuju jednom, PIN-om, i nakon toga se automatski prepoznaju.
gui-paired-servers = Računari čije uređaje možete koristiti
gui-paired-clients = Računari koji mogu koristiti vaše uređaje
gui-paired-empty = Još nema.
gui-remove-confirm = Ukloniti { $name }? Za ponovnu upotrebu biće potrebno ponovo se upariti.
gui-fingerprint = Otisak
gui-service-denied-title = Ovaj korisnik ne može upravljati USB Nexus servisom
gui-service-denied-body = To mogu samo administratori i članovi grupe usbnexus. Dozvolite ovom korisniku (zatražiće se administratorska lozinka), ili pokrenite kao administrator:
gui-service-denied-command = Komanda:
gui-service-denied-user = KORISNIK
gui-grant-access = Dozvoli ovom korisniku
gui-grant-access-done = Pristup je dodeljen.
gui-setup-kernel-modules = Nedostaju kernel moduli potrebni za USB preko IP-a: { $modules }. Instalirajte ih; servis ih zatim sam učitava:
gui-setup-kernel-modules-nocmd = Dolaze uz kernel većine distribucija (Ubuntu: linux-modules-extra, Fedora: kernel-modules-extra).
gui-service-down-title = USB Nexus servis nije pokrenut
gui-service-down-body = Pokrenite servis; ovaj prozor se automatski povezuje na njega.
gui-service-down-linux = Na Linuxu pokrenite:
gui-service-down-windows = Na Windowsu, kao administrator, pokrenite:
gui-service-down-macos = Na macOS-u pokrenite:
gui-retry = Pokušaj ponovo
gui-details = Detalji
err-service-unavailable = USB Nexus servis nije dostupan.

## Web interface

gui-web-login-title = Prijava na { $name }
gui-web-password = Lozinka
gui-web-sign-in = Prijavi se
gui-web-sign-out = Odjavi se
gui-web-wrong-password = Netačna lozinka.
gui-web-locked = Previše pokušaja. Pokušajte ponovo za { $seconds } sekundi.
cmd-web-about = Uključi ili isključi veb interfejs.
cmd-enable-about = Uključi veb interfejs (prvi put traži lozinku).
cmd-disable-about = Isključi veb interfejs.
cmd-password-about = Promeni lozinku veb interfejsa.
cmd-status-about = Prikaži da li je veb interfejs uključen i gde.
arg-lan = Dozvoli pristup s drugih računara na mreži.
arg-port = TCP port veb interfejsa.
web-on = Veb interfejs je uključen:
web-off = Veb interfejs je isključen.
web-local-only = Samo ovaj računar može da ga otvori. Koristite --lan da dozvolite drugim računarima.
web-fingerprint = Pregledač će upozoriti na sertifikat; njegov otisak treba da bude: { $fp }
web-trusted = Pregledači na ovom računaru veruju sertifikatu; drugi računari će prikazati upozorenje. Njegov otisak je: { $fp }
web-not-running = Veb interfejs je uključen, ali se nije pokrenuo: { $detail }
web-password-prompt = Nova lozinka veb interfejsa:
web-password-repeat = Ponovite lozinku:
web-password-mismatch = Lozinke se ne poklapaju.
web-password-set = Lozinka veb interfejsa je promenjena.
err-weak-password = Lozinka mora imati najmanje 8 karaktera.
err-port-in-use = Ovaj port koristi drugi program. Izaberite drugi port.
err-password-required = Prvo postavite lozinku veb interfejsa: usbnexus web password
err-forbidden = Ovo se može promeniti samo na samom računaru.
err-not-logged-in = Prijavite se ponovo.

## Hotplug, access control and usage log

arg-device = Uređaj: identitet ili bus id (vidi: usbnexus list SERVER).
cmd-policy-about = Prikaži ili izaberite ko može koristiti deljene uređaje.
arg-policy = open: svaki upareni računar može koristiti svaki deljeni uređaj; restricted: samo računari dozvoljeni po uređaju.
arg-no-server = Ne podešavaj deljenje USB uređaja ovog računara.
arg-no-client = Ne podešavaj korišćenje USB uređaja drugih računara.
arg-web = Veb interfejs: off (isključen), local (samo ovaj računar) ili network (cela mreža).
arg-web-port = Port veb interfejsa (podrazumevano 3242).
arg-web-password-file = Fajl koji sadrži novu lozinku veb interfejsa.
cmd-history-about = Prikaži dnevnik korišćenja deljenih uređaja ovog računara.
arg-csv = Prikaži sve stavke u CSV formatu (npr. za čuvanje u fajl).
arg-limit = Broj stavki za prikaz.
cmd-log-about = Prikaži ili promeni koliko detaljno servis piše dnevnik.
arg-level = info (podrazumevano), debug ili trace. Odmah se primenjuje; više od info koristi se za traženje problema.
col-device-id = ID UREĐAJA
col-time = VREME (UTC)
col-event = DOGAĐAJ
col-computer = RAČUNAR
col-duration = TRAJANJE
state-unplugged = nije povezan
state-no-permission = nema dozvolu
serve-denied = „{ $client }” nema dozvolu da koristi { $busid }.
attach-waiting-device = Uređaj nije povezan na serveru; čekanje…
attach-queued = Uređaj koristi drugi računar; ovaj računar je { $position }. u redu čekanja.
policy-open = Svaki upareni računar može koristiti svaki deljeni uređaj (otvoreno).
policy-restricted = Upareni računari mogu koristiti samo uređaje za koje imaju dozvolu (ograničeno).
log-level = Nivo dnevnika: { $level }
history-header = Dnevnik korišćenja (stavke se čuvaju { $days } dana):
history-empty = Dnevnik korišćenja je prazan.
history-paired = uparen
history-pairing-failed = netačan PIN
history-attached = počeo korišćenje
history-detached = prestao korišćenje
history-denied = odbijeno (nema dozvolu)
err-access-denied = Ovaj računar nema dozvolu da koristi uređaj.

gui-nav-history = Istorija
gui-nav-settings = Podešavanja
gui-not-plugged-in = Nije povezan
gui-tracked-by-port = praćen po portu
gui-tracked-by-port-hint = Ovaj uređaj nema serijski broj, pa se prepoznaje po USB portu na koji je povezan. Povežite ga ponovo na isti port.
gui-no-permission = Nema dozvolu
gui-no-permission-hint = Vlasnik tog računara nije dozvolio ovom računaru da koristi uređaj.
gui-connect-when-plugged-in = Automatski se povezuje čim se uređaj priključi.
gui-state-waiting-device = Čekanje na uređaj
gui-state-queued = U upotrebi na drugom mestu; { $position }. u redu čekanja
gui-save = Sačuvaj
gui-saved = Sačuvano.
gui-skip = Preskoči
gui-access-title = Ko može koristiti ovaj uređaj
gui-access-everyone = Svi upareni računari
gui-access-some = Izabrani računari ({ $count })
gui-access-nobody = Još nema računara
gui-access-mode-default = Prati podrazumevano podešavanje
gui-access-default-open = Trenutno: svi upareni računari.
gui-access-default-restricted = Trenutno: samo računari izabrani ispod.
gui-access-mode-open = Svi upareni računari
gui-access-mode-open-body = Svaki računar uparen sa ovim može ga koristiti.
gui-access-mode-selected = Samo izabrani računari
gui-access-mode-selected-body = Samo računari označeni ispod mogu ga koristiti.
gui-access-computers = Računari kojima je dozvoljeno korišćenje
gui-access-revoke-note = Računaru kome je oduzeta dozvola veza sa uređajem se odmah prekida.
gui-client-devices = Uređaji
gui-client-devices-title = Uređaji koje { $name } može koristiti
gui-client-devices-body = Označite deljene uređaje koje ovaj računar može koristiti.
gui-client-devices-after-pairing = Deljene uređaje mogu koristiti samo dozvoljeni računari. Označite uređaje koje ovaj računar može koristiti; ako preskočite, za sada ne može koristiti nijedan.
gui-no-shared-devices = Ovaj računar još ne deli nijedan uređaj.
gui-roles-title = Kako se ovaj računar koristi
gui-roles-body = Ekrani za način korišćenja koji nije izabran su skriveni. Dodavanje instalira ono što je potrebno.
gui-role-server = Koristi kao server (deli USB uređaje ovog računara)
gui-role-client = Koristi kao klijent (koristi USB uređaje drugih računara)
gui-roles-client-note = Ako je potrebno, instalira se upravljački program usbip-win2; USB uređaji se prekidaju na nekoliko sekundi i možda će biti potrebno ponovo pokrenuti Windows.
gui-roles-applying = Primenjivanje…
gui-reboot-required = Ponovo pokrenite računar da biste dovršili podešavanje.
gui-used-by-waiting = { $name } koristi uređaj · { $count } čeka
gui-col-permissions = Dozvole
gui-col-status = Status
gui-in-use-title = Trenutno koristi
gui-nobody-using = Trenutno niko ne koristi uređaj.
gui-since = od { $time }
gui-disconnect-user = Prekini vezu
gui-disconnected-note = { $name } je isključen. Ako i dalje zahteva uređaj, ponovo se povezuje za nekoliko sekundi; da ga trajno onemogućite, uklonite oznaku na listi levo i sačuvajte.
gui-queue-title = Čekanje u redu ({ $count })
gui-queue-empty = Niko ne čeka.
gui-badge-using = koristi
gui-badge-queued = u redu ({ $position })
gui-handover-title = Automatsko predavanje
gui-handover-default-on = Podrazumevano (uključeno, { $seconds } s)
gui-handover-default-off = Podrazumevano (isključeno)
gui-handover-on = Uključeno
gui-handover-off = Isključeno
gui-handover-before = Dok drugi računar čeka, nakon
gui-handover-after = sekundi bez korišćenja prelazi na sledećeg.
gui-kind-storage = Skladište
gui-kind-input = Tastatura / miš
gui-kind-printer = Štampač
gui-kind-dongle = Licencni dongle
gui-kind-other = Ostali uređaji
gui-web-title = Veb interfejs
gui-web-body = Upravljajte ovim računarom iz pregledača, lozinkom.
gui-web-confirm-off = Isključiti veb interfejs? Ova stranica će prestati da radi. Može se ponovo uključiti u desktop aplikaciji ili komandom „usbnexus web enable” na samom računaru.
gui-web-confirm-local = Dozvoliti pristup samo sa ovog računara? Ova stranica je otvorena preko mreže i prestaće da radi.
gui-web-turned-off = Veb interfejs je isključen. Može se ponovo uključiti u desktop aplikaciji ili komandom „usbnexus web enable” na samom računaru.
gui-web-enabled = Veb interfejs uključen
gui-web-access-local = Samo ovaj računar
gui-web-access-network = Cela mreža
gui-web-port = Port
gui-web-new-password = Nova lozinka
gui-web-repeat-password = Nova lozinka (ponovo)
gui-web-password-keep = Najmanje 8 karaktera. Ostavite prazno da zadržite trenutnu lozinku.
gui-web-password-required = Najmanje 8 karaktera.
gui-web-mismatch = Lozinke se ne poklapaju.
gui-web-open-at = Otvori na:
gui-web-fingerprint = Pregledač upozorava na sertifikat; njegov otisak je { $fp }.
gui-web-trusted = Pregledači na ovom računaru veruju sertifikatu; drugi računari upozoravaju na njega. Njegov otisak je { $fp }.
gui-policy-title = Ko može koristiti deljene uređaje
gui-policy-body = Računari uvek moraju prvo biti upareni. Ovo je podrazumevano podešavanje za svaki deljeni uređaj; svaki uređaj ga može zaobići.
gui-policy-first-title = Ko može koristiti vaše deljene uređaje?
gui-policy-first-body = Računari uvek moraju prvo biti upareni PIN-om. Izaberite šta upareni računari mogu da rade:
gui-policy-later = Ovo možete promeniti kasnije u Podešavanjima.
gui-policy-open = Svaki upareni računar
gui-policy-open-body = Svaki upareni računar može koristiti svaki deljeni uređaj.
gui-policy-restricted = Samo dozvoljeni računari
gui-policy-restricted-body = Za svaki uređaj sami birate koji upareni računari mogu da ga koriste.
gui-retention-title = Dnevnik korišćenja
gui-retention-body = Uparivanja, netačni PIN-ovi, korišćenje uređaja i odbijeni zahtevi se beleže. Starije stavke se automatski brišu.
gui-retention-days = Čuvaj stavke (dana)
gui-settings-title = Podešavanja
gui-startup-title = Pokretanje
gui-startup-body = Zatvaranje prozora zadržava USB Nexus u oblasti obaveštenja; deljenje i veze i dalje rade preko servisa.
gui-startup-enabled = Pokreni automatski pri prijavi
gui-tray-open = Otvori USB Nexus
gui-tray-quit = Izlaz
gui-history-title = Istorija
gui-history-subtitle = Ko je i kada koristio uređaje ovog računara. Stavke se čuvaju { $days } dana.
gui-history-empty = Još nije zabeleženo ništa.
gui-history-export = Izvezi kao CSV
gui-history-time = Vreme
gui-history-event = Događaj
gui-history-computer = Računar
gui-history-device = Uređaj
gui-history-device-id = ID uređaja
gui-history-duration = Trajanje
gui-history-paired = Uparen
gui-history-pairing-failed = Netačan PIN
gui-history-attached = Počeo korišćenje
gui-history-detached = Prestao korišćenje
gui-history-denied = Odbijeno: nema dozvolu

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = Kako ćete koristiti USB Nexus?
setup-roles-subtitle = Izaberite šta će ovaj računar raditi.
setup-role-server = Koristi kao server
setup-role-client = Koristi kao klijent
setup-role-web = Veb pristup
setup-usbip-install-note = Biće instaliran i upravljački program usbip-win2. USB uređaji se prekidaju na nekoliko sekundi tokom instalacije, a Windows nakon toga mora biti ponovo pokrenut.
setup-usbip-update-note = Instalirani upravljački program usbip-win2 je previše star i biće ažuriran. USB uređaji se prekidaju na nekoliko sekundi tokom instalacije, a Windows nakon toga mora biti ponovo pokrenut.
setup-usbip-present-note = Upravljački program usbip-win2 je već instaliran na ovom računaru.
setup-usbip-failed = Upravljački program usbip-win2 nije moguće instalirati. Kasnije možete ponovo pokrenuti instalaciju USB Nexus-a da pokušate ponovo.
setup-service-failed = USB Nexus servis nije moguće podesiti. Detalji se nalaze u dnevniku instalacije.
setup-web-title = Veb interfejs
setup-web-subtitle = Podešavanja za upravljanje ovim računarom iz pregledača.
setup-web-access = Pristup:
setup-web-local = Samo ovaj računar
setup-web-network = Cela mreža
setup-web-port = Port:
setup-web-port-free = ✓ Port je slobodan
setup-web-port-busy = ✗ Ovaj port koristi drugi program
setup-web-port-invalid = ✗ Unesite broj između 1 i 65535
setup-web-password = Lozinka:
setup-web-password-repeat = Lozinka (ponovo):
setup-web-password-hint = Najmanje 8 karaktera.
setup-web-password-keep = Najmanje 8 karaktera. Ostavite prazno da zadržite trenutnu lozinku.
