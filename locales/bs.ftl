# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Bosnian.
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = Sigurni USB preko IP-a: dijeljenje USB uređaja putem mreže uz šifriranje, uparivanje i automatsko ponovno povezivanje.
arg-lang = Jezik interfejsa (npr. en, tr). Podrazumijevano: jezik sistema.
arg-state-dir = Direktorij za ključeve i uparene računare.
arg-name = Naziv ovog računara kako ga vide drugi.
arg-verbose = Prikaži detaljne poruke dnevnika.

cmd-daemon-about = Pokreni USB Nexus servis (koristi ga desktop aplikacija).
arg-allow-all-users = Dozvoli svim lokalnim korisnicima da upravljaju servisom (podrazumijevano: članovi grupe usbnexus).
arg-socket = Putanja lokalnog kontrolnog soketa servisa.
cmd-service-about = Instaliraj ili ukloni USB Nexus Windows servis.
cmd-install-about = Instaliraj servis, pokreni ga odmah i pri svakom podizanju sistema (pokrenite kao administrator).
cmd-uninstall-about = Zaustavi i ukloni servis.
cmd-serve-about = Dijeli USB uređaje ovog računara.
arg-export = Bus ID uređaja za dijeljenje (može se navesti više puta). Pogledajte: usbnexus local
arg-listen = Adresa i port za slušanje.
arg-pair = Otvori prozor za uparivanje pri pokretanju i prikaži PIN.
arg-no-mdns = Ne oglašavaj ovaj server na lokalnoj mreži.

cmd-pin-about = Prikaži PIN za uparivanje novog računara s aktivnim serverom.
arg-seconds = Koliko dugo PIN važi, u sekundama.

cmd-local-about = Prikaži listu USB uređaja povezanih na ovaj računar.
cmd-discover-about = Pronađi USB Nexus servere na lokalnoj mreži.
arg-timeout = Koliko dugo tražiti, u sekundama.
cmd-pair-about = Uparite se sa serverom pomoću PIN-a koji on prikazuje.
arg-server = Server: upareni naziv, otisak (fingerprint) ili host[:port].
arg-pin = PIN koji prikazuje server (bit će zatražen ako se ne navede).
cmd-list-about = Prikaži listu uređaja koje dijeli server.
cmd-allow-user-about = Dozvoli korisniku da upravlja servisom iz desktop aplikacije (dodaje ga u grupu usbnexus).
arg-user = Korisničko ime.
allow-user-invalid = „{ $user }” nije važeće korisničko ime.
allow-user-failed = Nije moguće dodati „{ $user }” u grupu usbnexus.
allow-user-done = „{ $user }” sada može upravljati USB Nexus servisom.
allow-user-relogin = Primjenjuje se nakon ponovne prijave.
cmd-attach-about = Koristi udaljeni uređaj na ovom računaru; automatski se ponovo povezuje.
arg-busid = Bus ID udaljenog uređaja.
cmd-peers-about = Prikaži listu uparenih računara.
cmd-forget-about = Ukloni upareni računar.
arg-peer = Naziv ili otisak (fingerprint) uparenog računara.

## General

error-prefix = Greška: { $detail }
hint-root = Za ovu radnju su potrebna administratorska ovlaštenja. Pokušajte ponovo sa sudo.
unsupported-os = Ova komanda još nije podržana na ovom operativnom sistemu.
yes = da
no = ne

service-installed = USB Nexus servis je instaliran i pokrenut.
service-removed = USB Nexus servis je uklonjen.

## Server

serve-started = Server „{ $name }” sluša na { $addr }.
serve-fingerprint = Otisak: { $fp }
serve-exporting = Dijeljeni uređaji:
serve-no-exports = Nijedan uređaj nije dijeljen. Dodajte --export BUSID (lista uređaja: usbnexus local).
serve-pairing-pin = PIN za uparivanje: { $pin } (važi { $seconds } sekundi)
serve-paired = Uparen sa „{ $name }”.
serve-pairing-failed = Neuspješan pokušaj uparivanja sa adrese { $addr }.
serve-exported = { $busid } sada koristi „{ $client }”.
serve-released = { $busid } je oslobodio „{ $client }”.
serve-stopping = Zaustavljanje; uređaji se vraćaju svojim normalnim upravljačkim programima…
serve-mdns-failed = Otkrivanje na lokalnoj mreži nije dostupno: { $detail }
serve-bind-failed = Nije moguće pripremiti { $busid } za dijeljenje: { $detail }

## Pairing

pin-show = PIN za uparivanje: { $pin }
pin-hint = Na drugom računaru pokrenite „usbnexus pair { $name }” u naredne { $seconds } sekunde.
pin-no-server = Nije pronađen nijedan aktivni USB Nexus server. Pokrenite jedan komandom: usbnexus serve
pair-enter-pin = Unesite PIN prikazan na serveru:
pair-ok = Uparen sa „{ $name }” ({ $fp }).
pair-already = Već uparen sa „{ $name }”.

## Devices

local-header = USB uređaji na ovom računaru:
local-empty = Nije pronađen nijedan USB uređaj.
list-header = Uređaji koje dijeli „{ $name }”:
list-empty = Ovaj server ne dijeli nijedan uređaj.
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
forget-unknown = Nijedan upareni računar ne odgovara nazivu „{ $peer }”.

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
err-pairing-failed = Uparivanje nije uspjelo. Provjerite PIN i pokušajte ponovo.
err-no-such-device = Server ne dijeli ovaj uređaj.
err-device-busy = Uređaj koristi drugi računar.
err-internal = Server je prijavio internu grešku.
err-protocol = Neočekivan odgovor servera.
err-connection-lost = Veza sa računarom je izgubljena.
err-unsupported = Ovo još nije dostupno na ovom operativnom sistemu.
err-driver-missing = Upravljački program usbip-win2 nije instaliran. Ponovo pokrenite instalaciju USB Nexus-a i prihvatite instalaciju usbip-win2.
err-driver-outdated = Instalirani upravljački program usbip-win2 je previše zastario. Ponovo pokrenite instalaciju USB Nexus-a i prihvatite ažuriranje usbip-win2.
err-vboxusb-missing = VirtualBox USB upravljački programi potrebni za dijeljenje nisu instalirani. Ponovo instalirajte USB Nexus.
err-device-in-use-by-os = Operativni sistem koristi ovaj uređaj svojim upravljačkim programom, pa se ne može dijeliti s ovog računara.
err-unreachable = Računar nije dostupan. Provjerite da li je uključen i povezan na mrežu.
err-cancelled = Radnja je otkazana.
err-permission-denied = USB Nexus nema potrebna ovlaštenja.
err-invalid = Zahtjev nije razumljiv.
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
gui-this-subtitle = Odaberite koje uređaje mogu koristiti drugi računari.
gui-share = Dijeli
gui-shared = Dijeljen
gui-not-shared = Nije dijeljen
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
gui-refresh = Osvježi
gui-searching = Pretraga mreže…
gui-none-found = Nije pronađen nijedan računar. Provjerite da li je USB Nexus pokrenut na drugom računaru, ili ga dodajte po adresi.
gui-add-by-address = Dodaj po adresi
gui-address = Adresa
gui-address-hint = npr. 192.168.1.20
gui-pair = Upari
gui-paired = Upareno
gui-pair-title = Uparivanje sa { $name }
gui-pair-body = Na { $name } izaberite „Uparite novi računar” i unesite PIN koji se tamo prikazuje.
gui-pin = PIN
gui-pair-done = Upareno sa { $name }.
gui-devices-of = Uređaji koje dijeli { $name }
gui-no-remote-devices = Ovaj računar ne dijeli nijedan uređaj.
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
gui-state-failed = Neuspješno
gui-reconnect = Ponovo poveži
gui-remove = Ukloni
gui-paired-title = Upareni računari
gui-paired-subtitle = Računari se uparuju jednom, PIN-om, i nakon toga se automatski prepoznaju.
gui-paired-servers = Računari čije uređaje možete koristiti
gui-paired-clients = Računari koji mogu koristiti vaše uređaje
gui-paired-empty = Još nema.
gui-remove-confirm = Ukloniti { $name }? Za ponovnu upotrebu bit će potrebno ponovo se upariti.
gui-fingerprint = Otisak
gui-service-denied-title = Ovaj korisnik ne može upravljati USB Nexus servisom
gui-service-denied-body = To mogu samo administratori i članovi grupe usbnexus. Dozvolite ovom korisniku (zatražit će se administratorska lozinka), ili pokrenite kao administrator:
gui-service-denied-command = Komanda:
gui-service-denied-user = KORISNIK
gui-grant-access = Dozvoli ovom korisniku
gui-grant-access-done = Pristup je dodijeljen.
gui-setup-kernel-modules = Nedostaju kernel moduli potrebni za USB preko IP-a: { $modules }. Instalirajte ih; servis će ih zatim sam učitati:
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
gui-web-wrong-password = Pogrešna lozinka.
gui-web-locked = Previše pokušaja. Pokušajte ponovo za { $seconds } sekundi.
cmd-web-about = Uključi ili isključi web interfejs.
cmd-enable-about = Uključi web interfejs (prvi put traži lozinku).
cmd-disable-about = Isključi web interfejs.
cmd-password-about = Promijeni lozinku web interfejsa.
cmd-status-about = Prikaži da li je web interfejs uključen i gdje.
arg-lan = Dozvoli pristup s drugih računara na mreži.
arg-port = TCP port web interfejsa.
web-on = Web interfejs je uključen:
web-off = Web interfejs je isključen.
web-local-only = Samo ovaj računar može ga otvoriti. Koristite --lan da dozvolite drugim računarima.
web-fingerprint = Preglednik će upozoriti na certifikat; njegov otisak treba da bude: { $fp }
web-trusted = Preglednici na ovom računaru vjeruju certifikatu; drugi računari će prikazati upozorenje. Njegov otisak je: { $fp }
web-not-running = Web interfejs je uključen, ali se nije pokrenuo: { $detail }
web-password-prompt = Nova lozinka web interfejsa:
web-password-repeat = Ponovite lozinku:
web-password-mismatch = Lozinke se ne poklapaju.
web-password-set = Lozinka web interfejsa je promijenjena.
err-weak-password = Lozinka mora imati najmanje 8 znakova.
err-port-in-use = Ovaj port koristi drugi program. Izaberite drugi port.
err-password-required = Prvo postavite lozinku web interfejsa: usbnexus web password
err-forbidden = Ovo se može promijeniti samo na samom računaru.
err-not-logged-in = Molimo prijavite se ponovo.

## Hotplug, access control and usage log

arg-device = Uređaj: identitet ili bus id (pogledajte: usbnexus list SERVER).
cmd-policy-about = Prikaži ili odaberite ko može koristiti dijeljene uređaje.
arg-policy = open: svaki upareni računar može koristiti svaki dijeljeni uređaj; restricted: samo računari dozvoljeni po uređaju.
arg-no-server = Ne podešavaj dijeljenje USB uređaja ovog računara.
arg-no-client = Ne podešavaj korištenje USB uređaja drugih računara.
arg-web = Web interfejs: off (isključen), local (samo ovaj računar) ili network (cijela mreža).
arg-web-port = Port web interfejsa (podrazumijevano 3242).
arg-web-password-file = Datoteka koja sadrži novu lozinku web interfejsa.
cmd-history-about = Prikaži dnevnik korištenja dijeljenih uređaja ovog računara.
arg-csv = Prikaži sve stavke u CSV formatu (npr. za čuvanje u datoteku).
arg-limit = Broj stavki za prikaz.
cmd-log-about = Prikaži ili promijeni koliko detaljno servis piše dnevnik.
arg-level = info (podrazumijevano), debug ili trace. Odmah se primjenjuje; više od info koristi se za traženje problema.
col-device-id = ID UREĐAJA
col-time = VRIJEME (UTC)
col-event = DOGAĐAJ
col-computer = RAČUNAR
col-duration = TRAJANJE
state-unplugged = nije priključen
state-no-permission = nema dozvolu
serve-denied = „{ $client }” nema dozvolu da koristi { $busid }.
attach-waiting-device = Uređaj nije priključen na serveru; čeka se…
attach-queued = Drugi računar koristi uređaj; ovaj računar je { $position }. u redu čekanja.
policy-open = Svaki upareni računar može koristiti svaki dijeljeni uređaj (otvoreno).
policy-restricted = Upareni računari mogu koristiti samo uređaje za koje imaju dozvolu (ograničeno).
log-level = Nivo dnevnika: { $level }
history-header = Dnevnik korištenja (stavke se čuvaju { $days } dana):
history-empty = Dnevnik korištenja je prazan.
history-paired = upareno
history-pairing-failed = pogrešan PIN
history-attached = počeo korištenje
history-detached = prestao korištenje
history-denied = odbijeno (nema dozvolu)
err-access-denied = Ovaj računar nema dozvolu da koristi uređaj.

gui-nav-history = Historija
gui-nav-settings = Postavke
gui-not-plugged-in = Nije priključen
gui-tracked-by-port = praćen po portu
gui-tracked-by-port-hint = Ovaj uređaj nema serijski broj, pa se prepoznaje po USB portu na koji je priključen. Priključite ga ponovo na isti port.
gui-no-permission = Nema dozvolu
gui-no-permission-hint = Vlasnik tog računara nije dozvolio ovom računaru da koristi uređaj.
gui-connect-when-plugged-in = Automatski se povezuje čim se uređaj priključi.
gui-state-waiting-device = Čekanje na uređaj
gui-state-queued = U upotrebi na drugom mjestu; { $position }. u redu čekanja
gui-save = Sačuvaj
gui-saved = Sačuvano.
gui-skip = Preskoči
gui-access-title = Ko može koristiti ovaj uređaj
gui-access-everyone = Svi upareni računari
gui-access-some = Odabrani računari ({ $count })
gui-access-nobody = Još nema računara
gui-access-mode-default = Prati podrazumijevanu postavku
gui-access-default-open = Trenutno: svi upareni računari.
gui-access-default-restricted = Trenutno: samo računari odabrani ispod.
gui-access-mode-open = Svi upareni računari
gui-access-mode-open-body = Svaki računar uparen s ovim može ga koristiti.
gui-access-mode-selected = Samo odabrani računari
gui-access-mode-selected-body = Samo računari označeni ispod mogu ga koristiti.
gui-access-computers = Računari kojima je dozvoljeno korištenje
gui-access-revoke-note = Računaru kojem je oduzeta dozvola veza sa uređajem se odmah prekida.
gui-client-devices = Uređaji
gui-client-devices-title = Uređaji koje { $name } može koristiti
gui-client-devices-body = Označite dijeljene uređaje koje ovaj računar može koristiti.
gui-client-devices-after-pairing = Dijeljene uređaje mogu koristiti samo dozvoljeni računari. Označite uređaje koje ovaj računar može koristiti; ako preskočite, za sada ne može koristiti nijedan.
gui-no-shared-devices = Ovaj računar još ne dijeli nijedan uređaj.
gui-roles-title = Kako se ovaj računar koristi
gui-roles-body = Ekrani za način korištenja koji nije odabran su skriveni. Dodavanje instalira ono što je potrebno.
gui-role-server = Koristi kao server (dijeli USB uređaje ovog računara)
gui-role-client = Koristi kao klijent (koristi USB uređaje drugih računara)
gui-roles-client-note = Ako je potrebno, instalira se upravljački program usbip-win2; USB uređaji se prekidaju na nekoliko sekundi i možda će biti potrebno ponovo pokrenuti Windows.
gui-roles-applying = Primjenjivanje…
gui-reboot-required = Ponovo pokrenite računar da biste završili postavljanje.
gui-used-by-waiting = { $name } koristi uređaj · { $count } čeka
gui-col-permissions = Dozvole
gui-col-status = Status
gui-in-use-title = Trenutno koristi
gui-nobody-using = Trenutno niko ne koristi uređaj.
gui-since = od { $time }
gui-disconnect-user = Prekini vezu
gui-disconnected-note = { $name } je isključen. Ako i dalje zahtijeva uređaj, ponovo se povezuje za nekoliko sekundi; da ga trajno onemogućite, uklonite oznaku na listi lijevo i sačuvajte.
gui-queue-title = Čekanje u redu ({ $count })
gui-queue-empty = Niko ne čeka.
gui-badge-using = koristi
gui-badge-queued = u redu ({ $position })
gui-handover-title = Automatska predaja
gui-handover-default-on = Podrazumijevano (uključeno, { $seconds } s)
gui-handover-default-off = Podrazumijevano (isključeno)
gui-handover-on = Uključeno
gui-handover-off = Isključeno
gui-handover-before = Dok drugi računar čeka, nakon
gui-handover-after = sekundi bez korištenja prelazi na sljedećeg.
gui-kind-storage = Skladište
gui-kind-input = Tastatura / miš
gui-kind-printer = Štampač
gui-kind-dongle = Licencni dongle
gui-kind-other = Ostali uređaji
gui-web-title = Web interfejs
gui-web-body = Upravljajte ovim računarom iz preglednika, lozinkom.
gui-web-confirm-off = Isključiti web interfejs? Ova stranica će prestati da radi. Može se ponovo uključiti u desktop aplikaciji ili komandom „usbnexus web enable” na samom računaru.
gui-web-confirm-local = Dozvoliti pristup samo sa ovog računara? Ova stranica je otvorena preko mreže i prestat će da radi.
gui-web-turned-off = Web interfejs je isključen. Može se ponovo uključiti u desktop aplikaciji ili komandom „usbnexus web enable” na samom računaru.
gui-web-enabled = Web interfejs uključen
gui-web-access-local = Samo ovaj računar
gui-web-access-network = Cijela mreža
gui-web-port = Port
gui-web-new-password = Nova lozinka
gui-web-repeat-password = Nova lozinka (ponovo)
gui-web-password-keep = Najmanje 8 znakova. Ostavite prazno da zadržite trenutnu lozinku.
gui-web-password-required = Najmanje 8 znakova.
gui-web-mismatch = Lozinke se ne poklapaju.
gui-web-open-at = Otvori na:
gui-web-fingerprint = Preglednik upozorava na certifikat; njegov otisak je { $fp }.
gui-web-trusted = Preglednici na ovom računaru vjeruju certifikatu; drugi računari upozoravaju na njega. Njegov otisak je { $fp }.
gui-policy-title = Ko može koristiti dijeljene uređaje
gui-policy-body = Računari uvijek moraju prvo biti upareni. Ovo je podrazumijevana postavka za svaki dijeljeni uređaj; svaki uređaj je može zaobići.
gui-policy-first-title = Ko može koristiti vaše dijeljene uređaje?
gui-policy-first-body = Računari uvijek moraju prvo biti upareni PIN-om. Odaberite šta upareni računari mogu raditi:
gui-policy-later = Ovo možete promijeniti kasnije u Postavkama.
gui-policy-open = Svaki upareni računar
gui-policy-open-body = Svaki upareni računar može koristiti svaki dijeljeni uređaj.
gui-policy-restricted = Samo dozvoljeni računari
gui-policy-restricted-body = Za svaki uređaj sami birate koji upareni računari mogu ga koristiti.
gui-retention-title = Dnevnik korištenja
gui-retention-body = Uparivanja, pogrešni PIN-ovi, korištenje uređaja i odbijeni zahtjevi se bilježe. Starije stavke se automatski brišu.
gui-retention-days = Čuvaj stavke (dana)
gui-settings-title = Postavke
gui-startup-title = Pokretanje
gui-startup-body = Zatvaranje prozora zadržava USB Nexus u području obavještenja; servis nastavlja dijeliti uređaje i održavati veze.
gui-startup-enabled = Pokreni automatski pri prijavi
gui-tray-open = Otvori USB Nexus
gui-tray-quit = Izlaz
gui-history-title = Historija
gui-history-subtitle = Ko je i kada koristio uređaje ovog računara. Stavke se čuvaju { $days } dana.
gui-history-empty = Još ništa nije zabilježeno.
gui-history-export = Izvezi kao CSV
gui-history-time = Vrijeme
gui-history-event = Događaj
gui-history-computer = Računar
gui-history-device = Uređaj
gui-history-device-id = ID uređaja
gui-history-duration = Trajanje
gui-history-paired = Upareno
gui-history-pairing-failed = Pogrešan PIN
gui-history-attached = Počeo korištenje
gui-history-detached = Prestao korištenje
gui-history-denied = Odbijeno: nema dozvolu

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = Kako ćete koristiti USB Nexus?
setup-roles-subtitle = Odaberite šta će ovaj računar raditi.
setup-role-server = Koristi kao server
setup-role-client = Koristi kao klijent
setup-role-web = Web pristup
setup-usbip-install-note = Bit će instaliran i upravljački program usbip-win2. USB uređaji se prekidaju na nekoliko sekundi tokom instalacije, a Windows nakon toga mora biti ponovo pokrenut.
setup-usbip-update-note = Instalirani upravljački program usbip-win2 je zastario i bit će ažuriran. USB uređaji se prekidaju na nekoliko sekundi tokom instalacije, a Windows nakon toga mora biti ponovo pokrenut.
setup-usbip-present-note = Upravljački program usbip-win2 je već instaliran na ovom računaru.
setup-usbip-failed = Upravljački program usbip-win2 nije moguće instalirati. Kasnije možete ponovo pokrenuti instalaciju USB Nexus-a da pokušate ponovo.
setup-service-failed = USB Nexus servis nije moguće postaviti. Detalji se nalaze u dnevniku instalacije.
setup-web-title = Web interfejs
setup-web-subtitle = Postavke za upravljanje ovim računarom iz preglednika.
setup-web-access = Pristup:
setup-web-local = Samo ovaj računar
setup-web-network = Cijela mreža
setup-web-port = Port:
setup-web-port-free = ✓ Port je slobodan
setup-web-port-busy = ✗ Ovaj port koristi drugi program
setup-web-port-invalid = ✗ Unesite broj između 1 i 65535
setup-web-password = Lozinka:
setup-web-password-repeat = Lozinka (ponovo):
setup-web-password-hint = Najmanje 8 znakova.
setup-web-password-keep = Najmanje 8 znakova. Ostavite prazno da zadržite trenutnu lozinku.
