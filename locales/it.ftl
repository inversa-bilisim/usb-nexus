# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Italian.

## Command line help

app-about = USB over IP sicuro: condividi dispositivi USB in rete con crittografia, associazione e riconnessione automatica.
arg-lang = Lingua dell'interfaccia (es. en, tr). Il valore predefinito è la lingua del sistema.
arg-state-dir = Cartella per le chiavi e i computer associati.
arg-name = Nome di questo computer mostrato agli altri.
arg-verbose = Mostra messaggi di registro dettagliati.

cmd-daemon-about = Esegui il servizio USB Nexus (usato dall'app desktop).
arg-allow-all-users = Consenti a tutti gli utenti locali di controllare il servizio (impostazione predefinita: membri del gruppo usbnexus).
arg-socket = Percorso del socket di controllo locale del servizio.
cmd-service-about = Installa o rimuovi il servizio Windows di USB Nexus.
cmd-install-about = Installa il servizio, avvialo ora e a ogni avvio (eseguire come amministratore).
cmd-uninstall-about = Arresta e rimuovi il servizio.
cmd-serve-about = Condividi i dispositivi USB di questo computer.
arg-export = Bus ID di un dispositivo da condividere (ripetibile). Vedi: usbnexus local
arg-listen = Indirizzo e porta su cui restare in ascolto.
arg-pair = Apri una finestra di associazione all'avvio e mostra il PIN.
arg-no-mdns = Non annunciare questo server sulla rete locale.

cmd-pin-about = Mostra un PIN per associare un nuovo computer al server in esecuzione.
arg-seconds = Per quanto tempo il PIN resta valido, in secondi.

cmd-local-about = Elenca i dispositivi USB collegati a questo computer.
cmd-discover-about = Trova i server USB Nexus sulla rete locale.
arg-timeout = Durata della ricerca, in secondi.
cmd-pair-about = Associati a un server usando il PIN che mostra.
arg-server = Server: nome associato, fingerprint o host[:porta].
arg-pin = PIN mostrato dal server (richiesto se omesso).
cmd-list-about = Elenca i dispositivi condivisi da un server.
cmd-allow-user-about = Consenti a un utente di controllare il servizio dall'app desktop (lo aggiunge al gruppo usbnexus).
arg-user = Nome utente.
allow-user-invalid = “{ $user }” non è un nome utente valido.
allow-user-failed = Non è stato possibile aggiungere “{ $user }” al gruppo usbnexus.
allow-user-done = “{ $user }” può ora controllare il servizio USB Nexus.
allow-user-relogin = Ha effetto dopo aver eseguito nuovamente l'accesso.
cmd-attach-about = Usa un dispositivo remoto su questo computer; si riconnette automaticamente.
arg-busid = Bus ID del dispositivo remoto.
cmd-peers-about = Elenca i computer associati.
cmd-forget-about = Rimuovi un computer associato.
arg-peer = Nome o fingerprint del computer associato.

## General

error-prefix = Errore: { $detail }
hint-root = Questa operazione richiede i diritti di amministratore. Riprovare con sudo.
unsupported-os = Questo comando non è ancora supportato su questo sistema operativo.
yes = sì
no = no

service-installed = Il servizio USB Nexus è stato installato e avviato.
service-removed = Il servizio USB Nexus è stato rimosso.

## Server

serve-started = Il server “{ $name }” è in ascolto su { $addr }.
serve-fingerprint = Fingerprint: { $fp }
serve-exporting = Dispositivi condivisi:
serve-no-exports = Nessun dispositivo condiviso. Aggiungere --export BUSID (elencare i dispositivi con: usbnexus local).
serve-pairing-pin = PIN di associazione: { $pin } (valido per { $seconds } secondi)
serve-paired = Associato a “{ $name }”.
serve-pairing-failed = Tentativo di associazione non riuscito da { $addr }.
serve-exported = { $busid } è ora usato da “{ $client }”.
serve-released = { $busid } è stato rilasciato da “{ $client }”.
serve-stopping = Arresto in corso; i dispositivi vengono restituiti ai driver normali…
serve-mdns-failed = La rilevabilità sulla rete locale non è disponibile: { $detail }
serve-bind-failed = Non è stato possibile preparare { $busid } per la condivisione: { $detail }

## Pairing

pin-show = PIN di associazione: { $pin }
pin-hint = Sull'altro computer eseguire “usbnexus pair { $name }” entro { $seconds } secondi.
pin-no-server = Non è stato trovato nessun server USB Nexus in esecuzione. Avviarne uno con: usbnexus serve
pair-enter-pin = Inserire il PIN mostrato sul server:
pair-ok = Associato a “{ $name }” ({ $fp }).
pair-already = Già associato a “{ $name }”.

## Devices

local-header = Dispositivi USB su questo computer:
local-empty = Nessun dispositivo USB trovato.
list-header = Dispositivi condivisi da “{ $name }”:
list-empty = Questo server non condivide alcun dispositivo.
list-in-use = in uso
col-busid = BUS ID
col-id = VID:PID
col-speed = VELOCITÀ
col-product = PRODOTTO
col-driver = DRIVER
col-state = STATO
col-name = NOME
col-fingerprint = FINGERPRINT
col-address = INDIRIZZO

## Discovery and peers

discover-searching = Ricerca sulla rete locale in corso…
discover-none = Non è stato trovato nessun server USB Nexus.
discover-paired = associato
peers-empty = Nessun computer associato finora.
forget-ok = “{ $name }” è stato rimosso.
forget-unknown = Nessun computer associato corrisponde a “{ $peer }”.

## Attaching

attach-connecting = Connessione a { $addr } in corso…
attach-attached = { $busid } è collegato (porta virtuale { $port }).
attach-disconnected = Connessione persa: { $reason }
attach-retrying = Nuovo tentativo di connessione in { $seconds } secondi…
attach-detached = Il dispositivo è stato scollegato.
attach-stop-hint = Premere Ctrl+C per scollegare.

## Errors

err-pairing-required = “{ $name }” non è ancora associato a questo computer. Eseguire: usbnexus pair { $target }
err-not-found = Il server non è stato trovato in rete.
err-version = Il server esegue una versione incompatibile di USB Nexus.
err-not-trusted = Questo computer non è associato al server.
err-pairing-closed = L'associazione non è attiva sul server. Sul server eseguire: usbnexus pin
err-pairing-failed = Associazione non riuscita. Verificare il PIN e riprovare.
err-no-such-device = Il server non condivide questo dispositivo.
err-device-busy = Il dispositivo è in uso da un altro computer.
err-internal = Il server ha segnalato un errore interno.
err-protocol = Risposta imprevista dal server.
err-connection-lost = La connessione con il computer è stata persa.
err-unsupported = Questa funzione non è ancora disponibile su questo sistema operativo.
err-driver-missing = Il driver usbip-win2 non è installato. Eseguire di nuovo l'installazione di USB Nexus e accettare l'installazione di usbip-win2.
err-driver-outdated = Il driver usbip-win2 installato è troppo vecchio. Eseguire di nuovo l'installazione di USB Nexus e accettare l'aggiornamento di usbip-win2.
err-vboxusb-missing = I driver USB di VirtualBox necessari per la condivisione non sono installati. Reinstallare USB Nexus.
err-device-in-use-by-os = Il sistema operativo sta usando questo dispositivo con un proprio driver, quindi non può essere condiviso da questo computer.
err-unreachable = Non è stato possibile raggiungere il computer. Verificare che sia acceso e collegato alla rete.
err-cancelled = L'operazione è stata annullata.
err-permission-denied = USB Nexus non dispone delle autorizzazioni necessarie.
err-invalid = La richiesta non è stata compresa.
err-other = Si è verificato un problema: { $detail }

## Help layout

help-usage = Utilizzo:
help-arguments = Argomenti
help-options = Opzioni
help-commands = Comandi
arg-help = Mostra la guida.
arg-version = Mostra la versione.

## Peers

peers-servers = Server associati (questo computer può usare i loro dispositivi):
peers-clients = Client associati (autorizzati a usare i dispositivi di questo computer):

## Desktop app

gui-nav-this-computer = Questo computer
gui-nav-network = Computer in rete
gui-nav-connected = Dispositivi collegati
gui-nav-paired = Computer associati
gui-language = Lingua
gui-this-title = Dispositivi su questo computer
gui-this-subtitle = Scegli quali dispositivi possono essere usati da altri computer.
gui-share = Condividi
gui-shared = Condiviso
gui-not-shared = Non condiviso
gui-used-by = In uso da { $name }
gui-no-local-devices = Non è stato trovato nessun dispositivo USB su questo computer.
gui-unnamed-device = Dispositivo USB
gui-pair-new = Associa un nuovo computer
gui-pin-title = PIN di associazione
gui-pin-body = Inserisci questo PIN sull'altro computer.
gui-pin-remaining = Valido per altri { $seconds } secondi
gui-pin-expired = Il PIN è scaduto.
gui-pin-new = Nuovo PIN
gui-pin-stop = Interrompi l'associazione
gui-close = Chiudi
gui-cancel = Annulla
gui-network-title = Computer in rete
gui-network-subtitle = Computer USB Nexus trovati sulla rete locale.
gui-refresh = Aggiorna
gui-searching = Ricerca in rete in corso…
gui-none-found = Non è stato trovato nessun computer. Verifica che USB Nexus sia in esecuzione sull'altro computer, oppure aggiungilo tramite indirizzo.
gui-add-by-address = Aggiungi tramite indirizzo
gui-address = Indirizzo
gui-address-hint = es. 192.168.1.20
gui-pair = Associa
gui-paired = Associato
gui-pair-title = Associa a { $name }
gui-pair-body = Su { $name }, scegli “Associa un nuovo computer” e digita il PIN mostrato lì.
gui-pin = PIN
gui-pair-done = Associato a { $name }.
gui-devices-of = Dispositivi condivisi da { $name }
gui-no-remote-devices = Questo computer non condivide alcun dispositivo.
gui-connect = Connetti
gui-disconnect = Disconnetti
gui-in-use-elsewhere = In uso da un altro computer
gui-connected-here = Collegato a questo computer
gui-back = Indietro
gui-connected-title = Dispositivi collegati a questo computer
gui-connected-subtitle = I dispositivi remoti restano collegati e si riconnettono da soli se la rete si interrompe.
gui-connected-empty = Nessun dispositivo remoto collegato. Apri “Rete” per connetterne uno.
gui-on-computer = su { $name }
gui-state-connecting = Connessione in corso…
gui-state-attached = Collegato
gui-state-retrying = Nuova connessione in { $seconds } s
gui-state-stopped = Disconnesso
gui-state-failed = Non riuscito
gui-reconnect = Riconnetti
gui-remove = Rimuovi
gui-paired-title = Computer associati
gui-paired-subtitle = I computer vengono associati una sola volta con un PIN e in seguito riconosciuti automaticamente.
gui-paired-servers = Computer di cui puoi usare i dispositivi
gui-paired-clients = Computer che possono usare i tuoi dispositivi
gui-paired-empty = Nessuno finora.
gui-remove-confirm = Rimuovere { $name }? Per usarlo di nuovo sarà necessario associarlo nuovamente.
gui-fingerprint = Fingerprint
gui-service-denied-title = Questo utente non può controllare il servizio USB Nexus
gui-service-denied-body = Solo gli amministratori e i membri del gruppo usbnexus possono farlo. Consenti l'accesso a questo utente (verrà richiesta la password di amministratore), oppure esegui come amministratore:
gui-service-denied-command = Comando:
gui-service-denied-user = UTENTE
gui-grant-access = Consenti l'accesso a questo utente
gui-grant-access-done = Accesso consentito.
gui-setup-kernel-modules = Mancano i moduli del kernel necessari per USB over IP: { $modules }. Installali; il servizio li carica da solo:
gui-setup-kernel-modules-nocmd = Sono forniti con il kernel della maggior parte delle distribuzioni (Ubuntu: linux-modules-extra, Fedora: kernel-modules-extra).
gui-service-down-title = Il servizio USB Nexus non è in esecuzione
gui-service-down-body = Avvia il servizio; questa finestra vi si collegherà automaticamente.
gui-service-down-linux = Su Linux eseguire:
gui-service-down-windows = Su Windows, come amministratore, eseguire:
gui-service-down-macos = Su macOS eseguire:
gui-retry = Riprova
gui-details = Dettagli
err-service-unavailable = Non è stato possibile raggiungere il servizio USB Nexus.

## Web interface

gui-web-login-title = Accedi a { $name }
gui-web-password = Password
gui-web-sign-in = Accedi
gui-web-sign-out = Esci
gui-web-wrong-password = Password errata.
gui-web-locked = Troppi tentativi. Riprovare tra { $seconds } secondi.
cmd-web-about = Attiva o disattiva l'interfaccia web.
cmd-enable-about = Attiva l'interfaccia web (la prima volta chiede una password).
cmd-disable-about = Disattiva l'interfaccia web.
cmd-password-about = Cambia la password dell'interfaccia web.
cmd-status-about = Mostra se l'interfaccia web è attiva e dove.
arg-lan = Consenti l'accesso da altri computer della rete.
arg-port = Porta TCP dell'interfaccia web.
web-on = L'interfaccia web è attiva:
web-off = L'interfaccia web è disattivata.
web-local-only = Può essere aperta solo da questo computer. Usa --lan per consentire ad altri computer.
web-fingerprint = Il browser avviserà per il certificato; il suo fingerprint dovrebbe essere: { $fp }
web-trusted = I browser di questo computer considerano attendibile il certificato; gli altri computer avviseranno. Il suo fingerprint è: { $fp }
web-not-running = L'interfaccia web è abilitata ma non è stato possibile avviarla: { $detail }
web-password-prompt = Nuova password dell'interfaccia web:
web-password-repeat = Ripetere la password:
web-password-mismatch = Le password non corrispondono.
web-password-set = La password dell'interfaccia web è stata modificata.
err-weak-password = La password deve essere di almeno 8 caratteri.
err-port-in-use = Questa porta è usata da un altro programma. Scegliere un'altra porta.
err-password-required = Imposta prima una password per l'interfaccia web: usbnexus web password
err-forbidden = Questa impostazione può essere modificata solo dal computer stesso.
err-not-logged-in = Effettua nuovamente l'accesso.

## Hotplug, access control and usage log

arg-device = Dispositivo: identità o bus ID (vedi: usbnexus list SERVER).
cmd-policy-about = Mostra o scegli chi può usare i dispositivi condivisi.
arg-policy = open: ogni computer associato può usare ogni dispositivo condiviso; restricted: solo i computer autorizzati per ogni dispositivo.
arg-no-server = Non configurare la condivisione dei dispositivi USB di questo computer.
arg-no-client = Non configurare l'uso dei dispositivi USB di altri computer.
arg-web = Interfaccia web: off, local (solo questo computer) o network (tutta la rete).
arg-web-port = Porta dell'interfaccia web (predefinita 3242).
arg-web-password-file = File contenente la nuova password dell'interfaccia web.
cmd-history-about = Mostra il registro d'uso dei dispositivi condivisi di questo computer.
arg-csv = Stampa tutte le voci come CSV (ad es. per salvarle in un file).
arg-limit = Numero di voci da mostrare.
cmd-log-about = Mostra o modifica quanto registra il servizio.
arg-level = info (predefinito), debug o trace. Ha effetto subito; oltre info serve per cercare problemi.
col-device-id = ID DISPOSITIVO
col-time = ORA (UTC)
col-event = EVENTO
col-computer = COMPUTER
col-duration = DURATA
state-unplugged = non collegato
state-no-permission = nessuna autorizzazione
serve-denied = “{ $client }” non è autorizzato a usare { $busid }.
attach-waiting-device = Il dispositivo non è collegato al server; in attesa…
attach-queued = Un altro computer sta usando il dispositivo; questo è il numero { $position } in coda.
policy-open = Ogni computer associato può usare ogni dispositivo condiviso (aperta).
policy-restricted = I computer associati possono usare solo i dispositivi che sono autorizzati a usare (limitata).
log-level = Livello di registrazione: { $level }
history-header = Registro d'uso (le voci vengono conservate per { $days } giorni):
history-empty = Il registro d'uso è vuoto.
history-paired = associato
history-pairing-failed = PIN errato
history-attached = ha iniziato a usarlo
history-detached = ha smesso di usarlo
history-denied = rifiutato (nessuna autorizzazione)
err-access-denied = Questo computer non è autorizzato a usare il dispositivo.

gui-nav-history = Cronologia
gui-nav-settings = Impostazioni
gui-not-plugged-in = Non collegato
gui-tracked-by-port = tracciato tramite porta
gui-tracked-by-port-hint = Questo dispositivo non ha un numero di serie, quindi viene riconosciuto in base alla porta USB in cui è collegato. Collegalo di nuovo nella stessa porta.
gui-no-permission = Nessuna autorizzazione
gui-no-permission-hint = Il proprietario di quel computer non ha autorizzato questo computer a usare il dispositivo.
gui-connect-when-plugged-in = Si connette automaticamente non appena il dispositivo viene collegato.
gui-state-waiting-device = In attesa del dispositivo
gui-state-queued = In uso altrove; numero { $position } in coda
gui-save = Salva
gui-saved = Salvato.
gui-skip = Salta
gui-access-title = Chi può usare questo dispositivo
gui-access-everyone = Tutti i computer associati
gui-access-some = Computer selezionati ({ $count })
gui-access-nobody = Nessun computer per ora
gui-access-mode-default = Segui l'impostazione predefinita
gui-access-default-open = Attualmente: tutti i computer associati.
gui-access-default-restricted = Attualmente: solo i computer selezionati qui sotto.
gui-access-mode-open = Tutti i computer associati
gui-access-mode-open-body = Ogni computer associato a questo può usarlo.
gui-access-mode-selected = Solo i computer selezionati
gui-access-mode-selected-body = Solo i computer selezionati qui sotto possono usarlo.
gui-access-computers = Computer autorizzati a usarlo
gui-access-revoke-note = Un computer a cui viene revocata l'autorizzazione viene disconnesso immediatamente dal dispositivo.
gui-client-devices = Dispositivi
gui-client-devices-title = Dispositivi che { $name } può usare
gui-client-devices-body = Seleziona i dispositivi condivisi che questo computer può usare.
gui-client-devices-after-pairing = Solo i computer autorizzati possono usare i dispositivi condivisi. Seleziona i dispositivi che questo computer può usare; se salti questo passaggio, per ora non potrà usarne nessuno.
gui-no-shared-devices = Questo computer non condivide ancora alcun dispositivo.
gui-roles-title = Come viene usato questo computer
gui-roles-body = Le schermate di un uso non scelto vengono nascoste. Aggiungerne uno installa ciò che serve.
gui-role-server = Usa come server (condividi i dispositivi USB di questo computer)
gui-role-client = Usa come client (usa i dispositivi USB di altri computer)
gui-roles-client-note = Se necessario, verrà installato il driver usbip-win2; i dispositivi USB si interromperanno per qualche secondo ed è possibile che sia necessario riavviare Windows.
gui-roles-applying = Applicazione in corso…
gui-reboot-required = Riavvia il computer per completare la configurazione.
gui-used-by-waiting = { $name } lo sta usando · { $count } in attesa
gui-col-permissions = Autorizzazioni
gui-col-status = Stato
gui-in-use-title = In uso da
gui-nobody-using = Al momento nessuno sta usando il dispositivo.
gui-since = dalle { $time }
gui-disconnect-user = Disconnetti
gui-disconnected-note = { $name } è stato disconnesso. Se continua a richiedere il dispositivo si riconnetterà entro pochi secondi; per escluderlo definitivamente, deselezionalo nell'elenco a sinistra e salva.
gui-queue-title = In coda ({ $count })
gui-queue-empty = Nessuno in attesa.
gui-badge-using = in uso
gui-badge-queued = in coda ({ $position })
gui-handover-title = Consegna automatica
gui-handover-default-on = Predefinito (attivo, { $seconds } s)
gui-handover-default-off = Predefinito (disattivo)
gui-handover-on = Attivo
gui-handover-off = Disattivo
gui-handover-before = Mentre un altro computer è in attesa, dopo
gui-handover-after = secondi di inattività passa al successivo.
gui-kind-storage = Archiviazione
gui-kind-input = Tastiera / mouse
gui-kind-printer = Stampante
gui-kind-dongle = Chiave hardware
gui-kind-other = Altro dispositivo
gui-web-title = Interfaccia web
gui-web-body = Gestisci questo computer da un browser, con una password.
gui-web-confirm-off = Disattivare l'interfaccia web? Questa pagina smetterà di funzionare. Può essere riattivata dall'app desktop o con “usbnexus web enable” sul computer stesso.
gui-web-confirm-local = Consentire l'accesso solo da questo computer? Questa pagina è stata aperta dalla rete e smetterà di funzionare.
gui-web-turned-off = L'interfaccia web è disattivata. Può essere riattivata dall'app desktop o con “usbnexus web enable” sul computer stesso.
gui-web-enabled = Interfaccia web attiva
gui-web-access-local = Solo questo computer
gui-web-access-network = Tutta la rete
gui-web-port = Porta
gui-web-new-password = Nuova password
gui-web-repeat-password = Nuova password (di nuovo)
gui-web-password-keep = Almeno 8 caratteri. Lascia vuoto per mantenere la password attuale.
gui-web-password-required = Almeno 8 caratteri.
gui-web-mismatch = Le password non corrispondono.
gui-web-open-at = Apri all'indirizzo:
gui-web-fingerprint = Il browser avvisa per il certificato; il suo fingerprint è { $fp }.
gui-web-trusted = I browser di questo computer considerano attendibile il certificato; gli altri computer avvisano. Il suo fingerprint è { $fp }.
gui-policy-title = Chi può usare i dispositivi condivisi
gui-policy-body = I computer devono sempre essere associati prima. Questa è l'impostazione predefinita per ogni dispositivo condiviso; ogni dispositivo può sovrascriverla.
gui-policy-first-title = Chi può usare i tuoi dispositivi condivisi?
gui-policy-first-body = I computer devono sempre essere associati prima con un PIN. Scegli cosa possono fare i computer associati:
gui-policy-later = Puoi cambiarlo in seguito in Impostazioni.
gui-policy-open = Ogni computer associato
gui-policy-open-body = Ogni computer associato può usare ogni dispositivo condiviso.
gui-policy-restricted = Solo i computer autorizzati
gui-policy-restricted-body = Scegli tu, per ogni dispositivo, quali computer associati possono usarlo.
gui-retention-title = Registro d'uso
gui-retention-body = Vengono registrati associazioni, PIN errati, uso dei dispositivi e richieste rifiutate. Le voci meno recenti vengono eliminate automaticamente.
gui-retention-days = Conserva le voci per (giorni)
gui-settings-title = Impostazioni
gui-startup-title = Avvio
gui-startup-body = Chiudere la finestra mantiene USB Nexus nell'area di notifica; il servizio continua comunque a gestire condivisioni e connessioni.
gui-startup-enabled = Avvia automaticamente all'accesso
gui-tray-open = Apri USB Nexus
gui-tray-quit = Esci
gui-history-title = Cronologia
gui-history-subtitle = Chi ha usato i dispositivi di questo computer e quando. Le voci vengono conservate per { $days } giorni.
gui-history-empty = Non è stato ancora registrato nulla.
gui-history-export = Esporta CSV
gui-history-time = Ora
gui-history-event = Evento
gui-history-computer = Computer
gui-history-device = Dispositivo
gui-history-device-id = ID dispositivo
gui-history-duration = Durata
gui-history-paired = Associato
gui-history-pairing-failed = PIN errato
gui-history-attached = Ha iniziato a usarlo
gui-history-detached = Ha smesso di usarlo
gui-history-denied = Rifiutato: nessuna autorizzazione

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = Come vuoi usare USB Nexus?
setup-roles-subtitle = Scegli cosa farà questo computer.
setup-role-server = Usa come server
setup-role-client = Usa come client
setup-role-web = Accesso web
setup-usbip-install-note = Verrà installato anche il driver usbip-win2. I dispositivi USB si interrompono per qualche secondo durante l'installazione e sarà necessario riavviare Windows in seguito.
setup-usbip-update-note = Il driver usbip-win2 installato è troppo vecchio e verrà aggiornato. I dispositivi USB si interrompono per qualche secondo durante l'installazione e sarà necessario riavviare Windows in seguito.
setup-usbip-present-note = Il driver usbip-win2 è già installato su questo computer.
setup-usbip-failed = Non è stato possibile installare il driver usbip-win2. Puoi eseguire di nuovo l'installazione di USB Nexus più tardi per riprovare.
setup-service-failed = Non è stato possibile configurare il servizio USB Nexus. I dettagli sono nel registro di installazione.
setup-web-title = Interfaccia web
setup-web-subtitle = Impostazioni per gestire questo computer da un browser.
setup-web-access = Accesso:
setup-web-local = Solo questo computer
setup-web-network = Tutta la rete
setup-web-port = Porta:
setup-web-port-free = ✓ La porta è disponibile
setup-web-port-busy = ✗ Questa porta è usata da un altro programma
setup-web-port-invalid = ✗ Inserire un numero tra 1 e 65535
setup-web-password = Password:
setup-web-password-repeat = Password (di nuovo):
setup-web-password-hint = Almeno 8 caratteri.
setup-web-password-keep = Almeno 8 caratteri. Lascia vuoto per mantenere la password attuale.
