# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — French.
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = USB over IP sécurisé : partagez des périphériques USB sur le réseau avec chiffrement, appairage et reconnexion automatique.
arg-lang = Langue de l'interface (par ex. en, tr). Par défaut : la langue du système.
arg-state-dir = Dossier des clés et des ordinateurs appairés.
arg-name = Nom de cet ordinateur, tel qu'affiché aux autres.
arg-verbose = Afficher les messages de journal détaillés.

cmd-daemon-about = Exécuter le service USB Nexus (utilisé par l'application de bureau).
arg-allow-all-users = Autoriser tous les utilisateurs locaux à contrôler le service (par défaut : membres du groupe usbnexus).
arg-socket = Chemin du socket de contrôle local du service.
cmd-service-about = Installer ou supprimer le service Windows USB Nexus.
cmd-install-about = Installer le service, le démarrer maintenant et à chaque démarrage (à exécuter en tant qu'administrateur).
cmd-uninstall-about = Arrêter et supprimer le service.
cmd-serve-about = Partager les périphériques USB de cet ordinateur.
arg-export = Bus id d'un périphérique à partager (répétable). Voir : usbnexus local
arg-listen = Adresse et port d'écoute.
arg-pair = Ouvrir une fenêtre d'appairage au démarrage et afficher le code PIN.
arg-no-mdns = Ne pas annoncer ce serveur sur le réseau local.

cmd-pin-about = Afficher un code PIN pour appairer un nouvel ordinateur avec le serveur en cours d'exécution.
arg-seconds = Durée de validité du code PIN, en secondes.

cmd-local-about = Lister les périphériques USB connectés à cet ordinateur.
cmd-discover-about = Rechercher les serveurs USB Nexus sur le réseau local.
arg-timeout = Durée de la recherche, en secondes.
cmd-pair-about = S'appairer avec un serveur à l'aide du code PIN qu'il affiche.
arg-server = Serveur : nom appairé, empreinte ou hôte[:port].
arg-pin = Code PIN affiché par le serveur (demandé s'il est omis).
cmd-list-about = Lister les périphériques partagés par un serveur.
cmd-allow-user-about = Autoriser un utilisateur à contrôler le service depuis l'application de bureau (l'ajoute au groupe usbnexus).
arg-user = Nom d'utilisateur.
allow-user-invalid = « { $user } » n'est pas un nom d'utilisateur valide.
allow-user-failed = Impossible d'ajouter « { $user } » au groupe usbnexus.
allow-user-done = « { $user } » peut désormais contrôler le service USB Nexus.
allow-user-relogin = Cela s'applique après une nouvelle connexion.
cmd-attach-about = Utiliser un périphérique distant sur cet ordinateur ; se reconnecte automatiquement.
arg-busid = Bus id du périphérique distant.
cmd-peers-about = Lister les ordinateurs appairés.
cmd-forget-about = Supprimer un ordinateur appairé.
arg-peer = Nom ou empreinte de l'ordinateur appairé.

## General

error-prefix = Erreur : { $detail }
hint-root = Cette opération nécessite les droits d'administrateur. Réessayez avec sudo.
unsupported-os = Cette commande n'est pas encore prise en charge sur ce système d'exploitation.
yes = oui
no = non

service-installed = Le service USB Nexus a été installé et démarré.
service-removed = Le service USB Nexus a été supprimé.

## Server

serve-started = Le serveur « { $name } » écoute sur { $addr }.
serve-fingerprint = Empreinte : { $fp }
serve-exporting = Périphériques partagés :
serve-no-exports = Aucun périphérique n'est partagé. Ajoutez --export BUSID (liste des périphériques : usbnexus local).
serve-pairing-pin = Code PIN d'appairage : { $pin } (valide { $seconds } secondes)
serve-paired = Appairé avec « { $name } ».
serve-pairing-failed = Tentative d'appairage échouée depuis { $addr }.
serve-exported = { $busid } est désormais utilisé par « { $client } ».
serve-released = { $busid } a été libéré par « { $client } ».
serve-stopping = Arrêt en cours ; restitution des périphériques à leurs pilotes normaux…
serve-mdns-failed = La découverte sur le réseau local n'est pas disponible : { $detail }
serve-bind-failed = Impossible de préparer { $busid } pour le partage : { $detail }

## Pairing

pin-show = Code PIN d'appairage : { $pin }
pin-hint = Sur l'autre ordinateur, exécutez « usbnexus pair { $name } » dans les { $seconds } secondes.
pin-no-server = Aucun serveur USB Nexus en cours d'exécution n'a été trouvé. Démarrez-en un avec : usbnexus serve
pair-enter-pin = Entrez le code PIN affiché sur le serveur :
pair-ok = Appairé avec « { $name } » ({ $fp }).
pair-already = Déjà appairé avec « { $name } ».

## Devices

local-header = Périphériques USB de cet ordinateur :
local-empty = Aucun périphérique USB trouvé.
list-header = Périphériques partagés par « { $name } » :
list-empty = Ce serveur ne partage aucun périphérique.
list-in-use = en cours d'utilisation
col-busid = BUS ID
col-id = VID:PID
col-speed = VITESSE
col-product = PRODUIT
col-driver = PILOTE
col-state = ÉTAT
col-name = NOM
col-fingerprint = EMPREINTE
col-address = ADRESSE

## Discovery and peers

discover-searching = Recherche sur le réseau local…
discover-none = Aucun serveur USB Nexus n'a été trouvé.
discover-paired = appairé
peers-empty = Aucun ordinateur appairé pour l'instant.
forget-ok = « { $name } » a été supprimé.
forget-unknown = Aucun ordinateur appairé ne correspond à « { $peer } ».

## Attaching

attach-connecting = Connexion à { $addr }…
attach-attached = { $busid } est connecté (port virtuel { $port }).
attach-disconnected = Connexion perdue : { $reason }
attach-retrying = Nouvelle tentative dans { $seconds } secondes…
attach-detached = Le périphérique a été déconnecté.
attach-stop-hint = Appuyez sur Ctrl+C pour déconnecter.

## Errors

err-pairing-required = « { $name } » n'est pas encore appairé avec cet ordinateur. Exécutez : usbnexus pair { $target }
err-not-found = Le serveur n'a pas pu être trouvé sur le réseau.
err-version = Le serveur exécute une version incompatible d'USB Nexus.
err-not-trusted = Cet ordinateur n'est pas appairé avec le serveur.
err-pairing-closed = L'appairage n'est pas ouvert sur le serveur. Sur le serveur, exécutez : usbnexus pin
err-pairing-failed = L'appairage a échoué. Vérifiez le code PIN et réessayez.
err-no-such-device = Le serveur ne partage pas ce périphérique.
err-device-busy = Le périphérique est utilisé par un autre ordinateur.
err-internal = Le serveur a signalé une erreur interne.
err-protocol = Réponse inattendue du serveur.
err-connection-lost = La connexion à l'ordinateur a été perdue.
err-unsupported = Cette fonctionnalité n'est pas encore disponible sur ce système d'exploitation.
err-driver-missing = Le pilote usbip-win2 n'est pas installé. Relancez l'installation d'USB Nexus et acceptez l'installation d'usbip-win2.
err-driver-outdated = Le pilote usbip-win2 installé est trop ancien. Relancez l'installation d'USB Nexus et acceptez la mise à jour d'usbip-win2.
err-vboxusb-missing = Les pilotes USB VirtualBox nécessaires au partage ne sont pas installés. Réinstallez USB Nexus.
err-device-in-use-by-os = Le système d'exploitation utilise ce périphérique avec son propre pilote ; il ne peut donc pas être partagé depuis cet ordinateur.
err-unreachable = Impossible d'atteindre l'ordinateur. Vérifiez qu'il est allumé et connecté au réseau.
err-cancelled = L'action a été annulée.
err-permission-denied = USB Nexus ne dispose pas des autorisations nécessaires.
err-invalid = La requête n'a pas été comprise.
err-other = Une erreur s'est produite : { $detail }

## Help layout

help-usage = Utilisation :
help-arguments = Arguments
help-options = Options
help-commands = Commandes
arg-help = Afficher l'aide.
arg-version = Afficher la version.

## Peers

peers-servers = Serveurs appairés (cet ordinateur peut utiliser leurs périphériques) :
peers-clients = Clients appairés (autorisés à utiliser les périphériques de cet ordinateur) :

## Desktop app

gui-nav-this-computer = Cet ordinateur
gui-nav-network = Ordinateurs sur le réseau
gui-nav-connected = Périphériques connectés
gui-nav-paired = Ordinateurs appairés
gui-language = Langue
gui-this-title = Périphériques de cet ordinateur
gui-this-subtitle = Choisissez les périphériques que les autres ordinateurs peuvent utiliser.
gui-share = Partager
gui-shared = Partagé
gui-not-shared = Non partagé
gui-used-by = Utilisé par { $name }
gui-no-local-devices = Aucun périphérique USB trouvé sur cet ordinateur.
gui-unnamed-device = Périphérique USB
gui-pair-new = Appairer un nouvel ordinateur
gui-pin-title = Code PIN d'appairage
gui-pin-body = Entrez ce code PIN sur l'autre ordinateur.
gui-pin-remaining = Valide encore { $seconds } secondes
gui-pin-expired = Le code PIN a expiré.
gui-pin-new = Nouveau code PIN
gui-pin-stop = Arrêter l'appairage
gui-close = Fermer
gui-cancel = Annuler
gui-network-title = Ordinateurs sur le réseau
gui-network-subtitle = Ordinateurs USB Nexus trouvés sur votre réseau local.
gui-refresh = Actualiser
gui-searching = Recherche sur le réseau…
gui-none-found = Aucun ordinateur trouvé. Vérifiez qu'USB Nexus est en cours d'exécution sur l'autre ordinateur, ou ajoutez-le par adresse.
gui-add-by-address = Ajouter par adresse
gui-address = Adresse
gui-address-hint = par ex. 192.168.1.20
gui-pair = Appairer
gui-paired = Appairé
gui-pair-title = Appairer avec { $name }
gui-pair-body = Sur { $name }, choisissez « Appairer un nouvel ordinateur » et saisissez le code PIN qui y est affiché.
gui-pin = Code PIN
gui-pair-done = Appairé avec { $name }.
gui-devices-of = Périphériques partagés par { $name }
gui-no-remote-devices = Cet ordinateur ne partage aucun périphérique.
gui-connect = Connecter
gui-disconnect = Déconnecter
gui-in-use-elsewhere = Utilisé par un autre ordinateur
gui-connected-here = Connecté à cet ordinateur
gui-back = Retour
gui-connected-title = Périphériques connectés à cet ordinateur
gui-connected-subtitle = Les périphériques distants restent connectés et se reconnectent automatiquement en cas de coupure réseau.
gui-connected-empty = Aucun périphérique distant n'est connecté. Ouvrez « Réseau » pour en connecter un.
gui-on-computer = sur { $name }
gui-state-connecting = Connexion…
gui-state-attached = Connecté
gui-state-retrying = Nouvelle tentative dans { $seconds } s
gui-state-stopped = Déconnecté
gui-state-failed = Échec
gui-reconnect = Reconnecter
gui-remove = Supprimer
gui-paired-title = Ordinateurs appairés
gui-paired-subtitle = Les ordinateurs sont appairés une fois avec un code PIN, puis reconnus automatiquement.
gui-paired-servers = Ordinateurs dont vous pouvez utiliser les périphériques
gui-paired-clients = Ordinateurs pouvant utiliser vos périphériques
gui-paired-empty = Aucun pour l'instant.
gui-remove-confirm = Supprimer { $name } ? Vous devrez l'appairer à nouveau pour l'utiliser.
gui-fingerprint = Empreinte
gui-service-denied-title = Cet utilisateur ne peut pas contrôler le service USB Nexus
gui-service-denied-body = Seuls les administrateurs et les membres du groupe usbnexus le peuvent. Autorisez cet utilisateur (demande le mot de passe administrateur), ou exécutez en tant qu'administrateur :
gui-service-denied-command = Commande :
gui-service-denied-user = UTILISATEUR
gui-grant-access = Autoriser cet utilisateur
gui-grant-access-done = Accès accordé.
gui-setup-kernel-modules = Les modules du noyau nécessaires à l'USB over IP sont manquants : { $modules }. Installez-les ; le service les chargera lui-même :
gui-setup-kernel-modules-nocmd = Ils sont fournis avec le noyau de la plupart des distributions (Ubuntu : linux-modules-extra, Fedora : kernel-modules-extra).
gui-service-down-title = Le service USB Nexus n'est pas en cours d'exécution
gui-service-down-body = Démarrez le service ; cette fenêtre s'y connectera automatiquement.
gui-service-down-linux = Sous Linux, exécutez :
gui-service-down-windows = Sous Windows, en tant qu'administrateur, exécutez :
gui-service-down-macos = Sous macOS, exécutez :
gui-retry = Réessayer
gui-details = Détails
err-service-unavailable = Le service USB Nexus n'a pas pu être contacté.

## Web interface

gui-web-login-title = Connexion à { $name }
gui-web-password = Mot de passe
gui-web-sign-in = Se connecter
gui-web-sign-out = Se déconnecter
gui-web-wrong-password = Mot de passe incorrect.
gui-web-locked = Trop de tentatives. Réessayez dans { $seconds } secondes.
cmd-web-about = Activer ou désactiver l'interface web.
cmd-enable-about = Activer l'interface web (demande un mot de passe la première fois).
cmd-disable-about = Désactiver l'interface web.
cmd-password-about = Changer le mot de passe de l'interface web.
cmd-status-about = Indiquer si l'interface web est active et où.
arg-lan = Autoriser l'accès depuis d'autres ordinateurs du réseau.
arg-port = Port TCP de l'interface web.
web-on = L'interface web est active :
web-off = L'interface web est désactivée.
web-local-only = Seul cet ordinateur peut y accéder. Utilisez --lan pour autoriser les autres ordinateurs.
web-fingerprint = Le navigateur affichera un avertissement sur le certificat ; son empreinte doit être : { $fp }
web-trusted = Les navigateurs de cet ordinateur font confiance au certificat ; les autres ordinateurs afficheront un avertissement. Son empreinte est : { $fp }
web-not-running = L'interface web est activée mais n'a pas pu démarrer : { $detail }
web-password-prompt = Nouveau mot de passe de l'interface web :
web-password-repeat = Répétez le mot de passe :
web-password-mismatch = Les mots de passe ne correspondent pas.
web-password-set = Le mot de passe de l'interface web a été changé.
err-weak-password = Le mot de passe doit contenir au moins 8 caractères.
err-password-required = Définissez d'abord un mot de passe pour l'interface web : usbnexus web password
err-forbidden = Cela ne peut être modifié que depuis l'ordinateur lui-même.
err-not-logged-in = Veuillez vous reconnecter.

## Hotplug, access control and usage log

arg-device = Périphérique : identité ou bus id (voir : usbnexus list SERVEUR).
cmd-policy-about = Afficher ou choisir qui peut utiliser les périphériques partagés.
arg-policy = open : tout ordinateur appairé peut utiliser tout périphérique partagé ; restricted : seuls les ordinateurs autorisés par périphérique.
arg-no-server = Ne pas configurer le partage des périphériques USB de cet ordinateur.
arg-no-client = Ne pas configurer l'utilisation des périphériques USB d'autres ordinateurs.
arg-web = Interface web : off (désactivée), local (cet ordinateur uniquement) ou network (tout le réseau).
arg-web-port = Port de l'interface web (par défaut 3242).
arg-web-password-file = Fichier contenant le nouveau mot de passe de l'interface web.
cmd-history-about = Afficher le journal d'utilisation des périphériques partagés par cet ordinateur.
arg-csv = Afficher toutes les entrées au format CSV (par ex. pour les enregistrer dans un fichier).
arg-limit = Nombre d'entrées à afficher.
col-device-id = ID PÉRIPHÉRIQUE
col-time = HEURE (UTC)
col-event = ÉVÉNEMENT
col-computer = ORDINATEUR
col-duration = DURÉE
state-unplugged = non branché
state-no-permission = pas d'autorisation
serve-denied = « { $client } » n'est pas autorisé à utiliser { $busid }.
attach-waiting-device = Le périphérique n'est pas branché sur le serveur ; en attente…
attach-queued = Un autre ordinateur utilise le périphérique ; celui-ci est en position { $position } dans la file d'attente.
policy-open = Tout ordinateur appairé peut utiliser tout périphérique partagé (ouvert).
policy-restricted = Les ordinateurs appairés ne peuvent utiliser que les périphériques qu'ils sont autorisés à utiliser (restreint).
history-header = Journal d'utilisation (entrées conservées { $days } jours) :
history-empty = Le journal d'utilisation est vide.
history-paired = appairé
history-pairing-failed = code PIN incorrect
history-attached = a commencé à utiliser
history-detached = a arrêté d'utiliser
history-denied = refusé (pas d'autorisation)
err-access-denied = Cet ordinateur n'est pas autorisé à utiliser le périphérique.

gui-nav-history = Historique
gui-nav-settings = Paramètres
gui-not-plugged-in = Non branché
gui-tracked-by-port = suivi par port
gui-tracked-by-port-hint = Ce périphérique n'a pas de numéro de série ; il est donc reconnu par le port USB sur lequel il est branché. Rebranchez-le sur le même port.
gui-no-permission = Pas d'autorisation
gui-no-permission-hint = Le propriétaire de cet ordinateur n'a pas autorisé cet ordinateur à utiliser le périphérique.
gui-connect-when-plugged-in = Se connecte automatiquement dès que le périphérique est branché.
gui-state-waiting-device = En attente du périphérique
gui-state-queued = Utilisé ailleurs ; position { $position } dans la file d'attente
gui-save = Enregistrer
gui-saved = Enregistré.
gui-skip = Ignorer
gui-access-title = Qui peut utiliser ce périphérique
gui-access-everyone = Tous les ordinateurs appairés
gui-access-some = Ordinateurs sélectionnés ({ $count })
gui-access-nobody = Aucun ordinateur pour l'instant
gui-access-mode-default = Suivre le paramètre par défaut
gui-access-default-open = Actuellement : tous les ordinateurs appairés.
gui-access-default-restricted = Actuellement : uniquement les ordinateurs sélectionnés ci-dessous.
gui-access-mode-open = Tous les ordinateurs appairés
gui-access-mode-open-body = Tout ordinateur appairé avec celui-ci peut l'utiliser.
gui-access-mode-selected = Seulement les ordinateurs sélectionnés
gui-access-mode-selected-body = Seuls les ordinateurs cochés ci-dessous peuvent l'utiliser.
gui-access-computers = Ordinateurs autorisés à l'utiliser
gui-access-revoke-note = Un ordinateur qui perd l'autorisation est immédiatement déconnecté du périphérique.
gui-client-devices = Périphériques
gui-client-devices-title = Périphériques que { $name } peut utiliser
gui-client-devices-body = Cochez les périphériques partagés que cet ordinateur peut utiliser.
gui-client-devices-after-pairing = Seuls les ordinateurs autorisés peuvent utiliser les périphériques partagés. Cochez les périphériques que cet ordinateur peut utiliser ; si vous ignorez cette étape, il ne pourra en utiliser aucun pour l'instant.
gui-no-shared-devices = Cet ordinateur ne partage encore aucun périphérique.
gui-roles-title = Utilisation de cet ordinateur
gui-roles-body = Les écrans d'une utilisation non choisie sont masqués. En ajouter une installe ce qui est nécessaire.
gui-role-server = Utiliser comme serveur (partager les périphériques USB de cet ordinateur)
gui-role-client = Utiliser comme client (utiliser les périphériques USB d'autres ordinateurs)
gui-roles-client-note = Si nécessaire, le pilote usbip-win2 est installé ; les périphériques USB s'interrompent pendant quelques secondes et Windows devra peut-être être redémarré.
gui-roles-applying = Application en cours…
gui-reboot-required = Redémarrez l'ordinateur pour terminer la configuration.
gui-used-by-waiting = { $name } l'utilise · { $count } en attente
gui-col-permissions = Autorisations
gui-col-status = État
gui-in-use-title = Utilisé par
gui-nobody-using = Personne n'utilise le périphérique actuellement.
gui-since = depuis { $time }
gui-disconnect-user = Déconnecter
gui-disconnected-note = { $name } a été déconnecté. S'il redemande le périphérique, il se reconnectera dans les secondes qui suivent ; pour l'écarter définitivement, décochez-le dans la liste de gauche et enregistrez.
gui-queue-title = En attente ({ $count })
gui-queue-empty = Personne n'attend.
gui-badge-using = en cours d'utilisation
gui-badge-queued = en attente ({ $position })
gui-handover-title = Transfert automatique
gui-handover-default-on = Par défaut (activé, { $seconds } s)
gui-handover-default-off = Par défaut (désactivé)
gui-handover-on = Activé
gui-handover-off = Désactivé
gui-handover-before = Tant qu'un autre ordinateur attend, après
gui-handover-after = secondes sans utilisation, il passe au suivant.
gui-kind-storage = Stockage
gui-kind-input = Clavier / souris
gui-kind-printer = Imprimante
gui-kind-dongle = Dongle de licence
gui-kind-other = Autre périphérique
gui-web-title = Interface web
gui-web-body = Gérez cet ordinateur depuis un navigateur, avec un mot de passe.
gui-web-confirm-off = Désactiver l'interface web ? Cette page ne fonctionnera plus. Elle peut être réactivée depuis l'application de bureau ou avec « usbnexus web enable » sur l'ordinateur lui-même.
gui-web-confirm-local = Autoriser l'accès uniquement depuis cet ordinateur ? Cette page a été ouverte depuis le réseau et ne fonctionnera plus.
gui-web-turned-off = L'interface web est désactivée. Elle peut être réactivée depuis l'application de bureau ou avec « usbnexus web enable » sur l'ordinateur lui-même.
gui-web-enabled = Interface web active
gui-web-access-local = Cet ordinateur uniquement
gui-web-access-network = Tout le réseau
gui-web-port = Port
gui-web-new-password = Nouveau mot de passe
gui-web-repeat-password = Nouveau mot de passe (à nouveau)
gui-web-password-keep = Au moins 8 caractères. Laissez vide pour conserver le mot de passe actuel.
gui-web-password-required = Au moins 8 caractères.
gui-web-mismatch = Les mots de passe ne correspondent pas.
gui-web-open-at = Ouvrir à l'adresse :
gui-web-fingerprint = Le navigateur affiche un avertissement sur le certificat ; son empreinte est { $fp }.
gui-web-trusted = Les navigateurs de cet ordinateur font confiance au certificat ; les autres ordinateurs affichent un avertissement. Son empreinte est { $fp }.
gui-policy-title = Qui peut utiliser les périphériques partagés
gui-policy-body = Les ordinateurs doivent toujours être appairés au préalable. Ceci est le paramètre par défaut pour chaque périphérique partagé ; chaque périphérique peut le modifier.
gui-policy-first-title = Qui peut utiliser vos périphériques partagés ?
gui-policy-first-body = Les ordinateurs doivent toujours être appairés au préalable avec un code PIN. Choisissez ce que les ordinateurs appairés peuvent faire :
gui-policy-later = Vous pourrez modifier ce choix plus tard dans les Paramètres.
gui-policy-open = Tout ordinateur appairé
gui-policy-open-body = Tout ordinateur appairé peut utiliser tout périphérique partagé.
gui-policy-restricted = Seulement les ordinateurs autorisés
gui-policy-restricted-body = Vous choisissez, pour chaque périphérique, quels ordinateurs appairés peuvent l'utiliser.
gui-retention-title = Journal d'utilisation
gui-retention-body = Les appairages, les codes PIN erronés, l'utilisation des périphériques et les demandes refusées sont enregistrés. Les entrées les plus anciennes sont supprimées automatiquement.
gui-retention-days = Conserver les entrées pendant (jours)
gui-settings-title = Paramètres
gui-startup-title = Démarrage
gui-startup-body = Fermer la fenêtre laisse USB Nexus dans la zone de notification ; le service continue de gérer les partages et les connexions.
gui-startup-enabled = Démarrer automatiquement à l'ouverture de session
gui-tray-open = Ouvrir USB Nexus
gui-tray-quit = Quitter
gui-history-title = Historique
gui-history-subtitle = Qui a utilisé les périphériques de cet ordinateur, et quand. Les entrées sont conservées { $days } jours.
gui-history-empty = Rien n'a encore été enregistré.
gui-history-export = Exporter en CSV
gui-history-time = Heure
gui-history-event = Événement
gui-history-computer = Ordinateur
gui-history-device = Périphérique
gui-history-device-id = ID du périphérique
gui-history-duration = Durée
gui-history-paired = Appairé
gui-history-pairing-failed = Code PIN incorrect
gui-history-attached = A commencé à utiliser
gui-history-detached = A arrêté d'utiliser
gui-history-denied = Refusé : pas d'autorisation

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = Comment allez-vous utiliser USB Nexus ?
setup-roles-subtitle = Choisissez ce que cet ordinateur va faire.
setup-role-server = Utiliser comme serveur
setup-role-client = Utiliser comme client
setup-role-web = Accès web
setup-usbip-install-note = Le pilote usbip-win2 sera également installé. Les périphériques USB s'interrompent pendant quelques secondes durant l'installation, et Windows devra ensuite être redémarré.
setup-usbip-update-note = Le pilote usbip-win2 installé est trop ancien et sera mis à jour. Les périphériques USB s'interrompent pendant quelques secondes durant l'installation, et Windows devra ensuite être redémarré.
setup-usbip-present-note = Le pilote usbip-win2 est déjà installé sur cet ordinateur.
setup-usbip-failed = Le pilote usbip-win2 n'a pas pu être installé. Vous pouvez relancer l'installation d'USB Nexus plus tard pour réessayer.
setup-service-failed = Le service USB Nexus n'a pas pu être configuré. Les détails figurent dans le journal d'installation.
setup-web-title = Interface web
setup-web-subtitle = Paramètres pour gérer cet ordinateur depuis un navigateur.
setup-web-access = Accès :
setup-web-local = Cet ordinateur uniquement
setup-web-network = Tout le réseau
setup-web-port = Port :
setup-web-port-free = ✓ Le port est disponible
setup-web-port-busy = ✗ Ce port est utilisé par un autre programme
setup-web-port-invalid = ✗ Entrez un nombre entre 1 et 65535
setup-web-password = Mot de passe :
setup-web-password-repeat = Mot de passe (à nouveau) :
setup-web-password-hint = Au moins 8 caractères.
setup-web-password-keep = Au moins 8 caractères. Laissez vide pour conserver le mot de passe actuel.
