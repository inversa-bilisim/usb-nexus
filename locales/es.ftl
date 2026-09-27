# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Spanish.
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = USB-over-IP seguro: comparta dispositivos USB por la red con cifrado, emparejamiento y reconexión automática.
arg-lang = Idioma de la interfaz (p. ej., en, tr). Predeterminado: idioma del sistema.
arg-state-dir = Carpeta para las claves y los equipos emparejados.
arg-name = Nombre de este equipo, tal como lo verán los demás.
arg-verbose = Mostrar mensajes de registro detallados.

cmd-daemon-about = Ejecutar el servicio de USB Nexus (lo usa la aplicación de escritorio).
arg-allow-all-users = Permitir que cualquier usuario local controle el servicio (predeterminado: los miembros del grupo usbnexus).
arg-socket = Ruta del socket de control local del servicio.
cmd-service-about = Instalar o quitar el servicio de USB Nexus en Windows.
cmd-install-about = Instalar el servicio, iniciarlo ahora y en cada arranque (ejecute como administrador).
cmd-uninstall-about = Detener y quitar el servicio.
cmd-serve-about = Compartir los dispositivos USB de este equipo.
arg-export = ID de bus de un dispositivo para compartir (se puede repetir). Vea: usbnexus local
arg-listen = Dirección y puerto en los que escuchar.
arg-pair = Abrir una ventana de emparejamiento al iniciar y mostrar el PIN.
arg-no-mdns = No anunciar este servidor en la red local.

cmd-pin-about = Mostrar un PIN para emparejar un equipo nuevo con el servidor en ejecución.
arg-seconds = Tiempo durante el que el PIN es válido, en segundos.

cmd-local-about = Mostrar los dispositivos USB conectados a este equipo.
cmd-discover-about = Buscar servidores de USB Nexus en la red local.
arg-timeout = Tiempo de búsqueda, en segundos.
cmd-pair-about = Emparejar con un servidor usando el PIN que este muestra.
arg-server = Servidor: nombre emparejado, huella digital o host[:puerto].
arg-pin = PIN mostrado por el servidor (se pedirá si se omite).
cmd-list-about = Mostrar los dispositivos compartidos por un servidor.
cmd-allow-user-about = Permitir que un usuario controle el servicio desde la aplicación de escritorio (lo añade al grupo usbnexus).
arg-user = Nombre de usuario.
allow-user-invalid = «{ $user }» no es un nombre de usuario válido.
allow-user-failed = No se pudo añadir a «{ $user }» al grupo usbnexus.
allow-user-done = «{ $user }» ya puede controlar el servicio de USB Nexus.
allow-user-relogin = Esto se aplicará al volver a iniciar sesión.
cmd-attach-about = Usar en este equipo un dispositivo remoto; se reconecta automáticamente.
arg-busid = ID de bus del dispositivo remoto.
cmd-peers-about = Mostrar los equipos emparejados.
cmd-forget-about = Quitar un equipo emparejado.
arg-peer = Nombre o huella digital del equipo emparejado.

## General

error-prefix = Error: { $detail }
hint-root = Esta operación necesita permisos de administrador. Vuelva a intentarlo con sudo.
unsupported-os = Este comando aún no es compatible con este sistema operativo.
yes = sí
no = no

service-installed = El servicio de USB Nexus se instaló y se inició.
service-removed = El servicio de USB Nexus se quitó.

## Server

serve-started = El servidor «{ $name }» está escuchando en { $addr }.
serve-fingerprint = Huella digital: { $fp }
serve-exporting = Dispositivos compartidos:
serve-no-exports = No se comparte ningún dispositivo. Añada --export BUSID (muestre los dispositivos con: usbnexus local).
serve-pairing-pin = PIN de emparejamiento: { $pin } (válido durante { $seconds } segundos)
serve-paired = Emparejado con «{ $name }».
serve-pairing-failed = Intento de emparejamiento fallido desde { $addr }.
serve-exported = { $busid } ahora lo usa «{ $client }».
serve-released = { $busid } fue liberado por «{ $client }».
serve-stopping = Deteniendo; devolviendo los dispositivos a sus controladores normales…
serve-mdns-failed = La detección en la red local no está disponible: { $detail }
serve-bind-failed = No se pudo preparar { $busid } para compartirlo: { $detail }

## Pairing

pin-show = PIN de emparejamiento: { $pin }
pin-hint = En el otro equipo, ejecute «usbnexus pair { $name }» antes de { $seconds } segundos.
pin-no-server = No se encontró ningún servidor de USB Nexus en ejecución. Inicie uno con: usbnexus serve
pair-enter-pin = Introduzca el PIN mostrado en el servidor:
pair-ok = Emparejado con «{ $name }» ({ $fp }).
pair-already = Ya está emparejado con «{ $name }».

## Devices

local-header = Dispositivos USB de este equipo:
local-empty = No se encontró ningún dispositivo USB.
list-header = Dispositivos compartidos por «{ $name }»:
list-empty = Este servidor no comparte ningún dispositivo.
list-in-use = en uso
col-busid = ID DE BUS
col-id = VID:PID
col-speed = VELOCIDAD
col-product = PRODUCTO
col-driver = CONTROLADOR
col-state = ESTADO
col-name = NOMBRE
col-fingerprint = HUELLA DIGITAL
col-address = DIRECCIÓN

## Discovery and peers

discover-searching = Buscando en la red local…
discover-none = No se encontró ningún servidor de USB Nexus.
discover-paired = emparejado
peers-empty = Todavía no hay equipos emparejados.
forget-ok = «{ $name }» se quitó.
forget-unknown = Ningún equipo emparejado coincide con «{ $peer }».

## Attaching

attach-connecting = Conectando con { $addr }…
attach-attached = { $busid } está conectado (puerto virtual { $port }).
attach-disconnected = Conexión perdida: { $reason }
attach-retrying = Reconectando en { $seconds } segundos…
attach-detached = El dispositivo se desconectó.
attach-stop-hint = Pulse Ctrl+C para desconectarlo.

## Errors

err-pairing-required = «{ $name }» todavía no está emparejado con este equipo. Ejecute: usbnexus pair { $target }
err-not-found = No se pudo encontrar el servidor en la red.
err-version = El servidor ejecuta una versión incompatible de USB Nexus.
err-not-trusted = Este equipo no está emparejado con el servidor.
err-pairing-closed = El emparejamiento no está abierto en el servidor. En el servidor, ejecute: usbnexus pin
err-pairing-failed = El emparejamiento falló. Compruebe el PIN e inténtelo de nuevo.
err-no-such-device = El servidor no comparte este dispositivo.
err-device-busy = Otro equipo está usando el dispositivo.
err-internal = El servidor notificó un error interno.
err-protocol = Respuesta inesperada del servidor.
err-connection-lost = Se perdió la conexión con el equipo.
err-unsupported = Esto todavía no está disponible en este sistema operativo.
err-driver-missing = El controlador usbip-win2 no está instalado. Vuelva a ejecutar la instalación de USB Nexus y acepte instalar usbip-win2.
err-driver-outdated = El controlador usbip-win2 instalado es demasiado antiguo. Vuelva a ejecutar la instalación de USB Nexus y acepte actualizar usbip-win2.
err-vboxusb-missing = No están instalados los controladores USB de VirtualBox necesarios para compartir. Reinstale USB Nexus.
err-device-in-use-by-os = El sistema operativo está usando este dispositivo con su propio controlador, por lo que no se puede compartir desde este equipo.
err-unreachable = No se pudo contactar con el equipo. Compruebe que está encendido y conectado a la red.
err-cancelled = La acción se canceló.
err-permission-denied = USB Nexus no tiene los permisos que necesita.
err-invalid = No se entendió la solicitud.
err-other = Algo salió mal: { $detail }

## Help layout

help-usage = Uso:
help-arguments = Argumentos
help-options = Opciones
help-commands = Comandos
arg-help = Mostrar la ayuda.
arg-version = Mostrar la versión.

## Peers

peers-servers = Servidores emparejados (este equipo puede usar sus dispositivos):
peers-clients = Clientes emparejados (pueden usar los dispositivos de este equipo):

## Desktop app

gui-nav-this-computer = Este equipo
gui-nav-network = Equipos en la red
gui-nav-connected = Dispositivos conectados
gui-nav-paired = Equipos emparejados
gui-language = Idioma
gui-this-title = Dispositivos de este equipo
gui-this-subtitle = Elija qué dispositivos pueden usar otros equipos.
gui-share = Compartir
gui-shared = Compartido
gui-not-shared = No compartido
gui-used-by = En uso por { $name }
gui-no-local-devices = No se encontró ningún dispositivo USB en este equipo.
gui-unnamed-device = Dispositivo USB
gui-pair-new = Emparejar un equipo nuevo
gui-pin-title = PIN de emparejamiento
gui-pin-body = Introduzca este PIN en el otro equipo.
gui-pin-remaining = Válido { $seconds } segundos más
gui-pin-expired = El PIN ha caducado.
gui-pin-new = Nuevo PIN
gui-pin-stop = Detener el emparejamiento
gui-close = Cerrar
gui-cancel = Cancelar
gui-network-title = Equipos en la red
gui-network-subtitle = Equipos con USB Nexus encontrados en su red local.
gui-refresh = Actualizar
gui-searching = Buscando en la red…
gui-none-found = No se encontró ningún equipo. Asegúrese de que USB Nexus se está ejecutando en el otro equipo, o añádalo por dirección.
gui-add-by-address = Añadir por dirección
gui-address = Dirección
gui-address-hint = p. ej., 192.168.1.20
gui-pair = Emparejar
gui-paired = Emparejado
gui-pair-title = Emparejar con { $name }
gui-pair-body = En { $name }, elija «Emparejar un equipo nuevo» y escriba el PIN que aparece allí.
gui-pin = PIN
gui-pair-done = Emparejado con { $name }.
gui-devices-of = Dispositivos compartidos por { $name }
gui-no-remote-devices = Este equipo no comparte ningún dispositivo.
gui-connect = Conectar
gui-disconnect = Desconectar
gui-in-use-elsewhere = En uso por otro equipo
gui-connected-here = Conectado a este equipo
gui-back = Atrás
gui-connected-title = Dispositivos conectados a este equipo
gui-connected-subtitle = Los dispositivos remotos permanecen conectados y se reconectan por sí solos si la red se interrumpe.
gui-connected-empty = No hay dispositivos remotos conectados. Abra «Red» para conectar uno.
gui-on-computer = en { $name }
gui-state-connecting = Conectando…
gui-state-attached = Conectado
gui-state-retrying = Reconectando en { $seconds } s
gui-state-stopped = Desconectado
gui-state-failed = Fallido
gui-reconnect = Reconectar
gui-remove = Quitar
gui-paired-title = Equipos emparejados
gui-paired-subtitle = Los equipos se emparejan una vez con un PIN y luego se reconocen automáticamente.
gui-paired-servers = Equipos cuyos dispositivos puede usar
gui-paired-clients = Equipos que pueden usar sus dispositivos
gui-paired-empty = Todavía ninguno.
gui-remove-confirm = ¿Quitar { $name }? Tendrá que volver a emparejarlo para usarlo.
gui-fingerprint = Huella digital
gui-service-denied-title = Este usuario no puede controlar el servicio de USB Nexus
gui-service-denied-body = Solo los administradores y los miembros del grupo usbnexus pueden hacerlo. Permita el acceso a este usuario (se pedirá la contraseña de administrador) o ejecute como administrador:
gui-service-denied-command = Comando:
gui-service-denied-user = USUARIO
gui-grant-access = Permitir el acceso a este usuario
gui-grant-access-done = Acceso concedido.
gui-setup-kernel-modules = Faltan los módulos del kernel necesarios para USB over IP: { $modules }. Instálelos; el servicio los carga por sí mismo:
gui-setup-kernel-modules-nocmd = Vienen con el kernel de la mayoría de las distribuciones (Ubuntu: linux-modules-extra, Fedora: kernel-modules-extra).
gui-service-down-title = El servicio de USB Nexus no se está ejecutando
gui-service-down-body = Inicie el servicio; esta ventana se conectará a él automáticamente.
gui-service-down-linux = En Linux, ejecute:
gui-service-down-windows = En Windows, como administrador, ejecute:
gui-service-down-macos = En macOS, ejecute:
gui-retry = Reintentar
gui-details = Detalles
err-service-unavailable = No se pudo contactar con el servicio de USB Nexus.

## Web interface

gui-web-login-title = Iniciar sesión en { $name }
gui-web-password = Contraseña
gui-web-sign-in = Iniciar sesión
gui-web-sign-out = Cerrar sesión
gui-web-wrong-password = Contraseña incorrecta.
gui-web-locked = Demasiados intentos. Vuelva a intentarlo en { $seconds } segundos.
cmd-web-about = Activar o desactivar la interfaz web.
cmd-enable-about = Activar la interfaz web (la primera vez pedirá una contraseña).
cmd-disable-about = Desactivar la interfaz web.
cmd-password-about = Cambiar la contraseña de la interfaz web.
cmd-status-about = Mostrar si la interfaz web está activa y dónde.
arg-lan = Permitir el acceso desde otros equipos de la red.
arg-port = Puerto TCP de la interfaz web.
web-on = La interfaz web está activa:
web-off = La interfaz web está desactivada.
web-local-only = Solo se puede abrir desde este equipo. Use --lan para permitir otros equipos.
web-fingerprint = El navegador advertirá sobre el certificado; su huella digital debe ser: { $fp }
web-trusted = Los navegadores de este equipo confían en el certificado; otros equipos mostrarán una advertencia. Su huella digital es: { $fp }
web-not-running = La interfaz web está activada, pero no pudo iniciarse: { $detail }
web-password-prompt = Nueva contraseña de la interfaz web:
web-password-repeat = Repita la contraseña:
web-password-mismatch = Las contraseñas no coinciden.
web-password-set = La contraseña de la interfaz web se cambió.
err-weak-password = La contraseña debe tener al menos 8 caracteres.
err-port-in-use = Este puerto lo usa otro programa. Elija otro puerto.
err-password-required = Establezca primero una contraseña para la interfaz web: usbnexus web password
err-forbidden = Esto solo se puede cambiar en el propio equipo.
err-not-logged-in = Vuelva a iniciar sesión.

## Hotplug, access control and usage log

arg-device = Dispositivo: identidad o ID de bus (vea: usbnexus list SERVIDOR).
cmd-policy-about = Mostrar o elegir quién puede usar los dispositivos compartidos.
arg-policy = open: cualquier equipo emparejado puede usar cualquier dispositivo compartido; restricted: solo los equipos permitidos por dispositivo.
arg-no-server = No configurar el compartir los dispositivos USB de este equipo.
arg-no-client = No configurar el uso de dispositivos USB de otros equipos.
arg-web = Interfaz web: off (desactivada), local (solo este equipo) o network (toda la red).
arg-web-port = Puerto de la interfaz web (predeterminado 3242).
arg-web-password-file = Archivo que contiene la nueva contraseña de la interfaz web.
cmd-history-about = Mostrar el registro de uso de los dispositivos compartidos de este equipo.
arg-csv = Mostrar todas las entradas como CSV (p. ej., para guardarlas en un archivo).
arg-limit = Número de entradas que se mostrarán.
cmd-log-about = Mostrar o cambiar cuánto registra el servicio.
arg-level = info (predeterminado), debug o trace. Se aplica de inmediato; más que info sirve para buscar problemas.
col-device-id = ID DE DISPOSITIVO
col-time = HORA (UTC)
col-event = EVENTO
col-computer = EQUIPO
col-duration = DURACIÓN
state-unplugged = no conectado
state-no-permission = sin permiso
serve-denied = «{ $client }» no tiene permiso para usar { $busid }.
attach-waiting-device = El dispositivo no está conectado en el servidor; esperando a que lo esté…
attach-queued = Otro equipo está usando el dispositivo; este es el número { $position } en la cola.
policy-open = Cualquier equipo emparejado puede usar cualquier dispositivo compartido (abierta).
policy-restricted = Los equipos emparejados solo pueden usar los dispositivos que tengan permitidos (restringida).
log-level = Nivel de registro: { $level }
history-header = Registro de uso (las entradas se conservan durante { $days } días):
history-empty = El registro de uso está vacío.
history-paired = emparejado
history-pairing-failed = PIN incorrecto
history-attached = empezó a usarlo
history-detached = dejó de usarlo
history-denied = rechazado (sin permiso)
err-access-denied = Este equipo no tiene permiso para usar el dispositivo.

gui-nav-history = Historial
gui-nav-settings = Configuración
gui-not-plugged-in = No conectado
gui-tracked-by-port = seguido por puerto
gui-tracked-by-port-hint = Este dispositivo no tiene número de serie, por lo que se reconoce por el puerto USB en el que está conectado. Conéctelo de nuevo en el mismo puerto.
gui-no-permission = Sin permiso
gui-no-permission-hint = El propietario de ese equipo no ha permitido que este equipo use el dispositivo.
gui-connect-when-plugged-in = Se conecta automáticamente en cuanto se enchufa el dispositivo.
gui-state-waiting-device = Esperando el dispositivo
gui-state-queued = En uso en otro lugar; número { $position } en la cola
gui-save = Guardar
gui-saved = Guardado.
gui-skip = Omitir
gui-access-title = Quién puede usar este dispositivo
gui-access-everyone = Todos los equipos emparejados
gui-access-some = Equipos seleccionados ({ $count })
gui-access-nobody = Todavía ningún equipo
gui-access-mode-default = Seguir la configuración predeterminada
gui-access-default-open = Actualmente: todos los equipos emparejados.
gui-access-default-restricted = Actualmente: solo los equipos seleccionados a continuación.
gui-access-mode-open = Todos los equipos emparejados
gui-access-mode-open-body = Cualquier equipo emparejado con este puede usarlo.
gui-access-mode-selected = Solo equipos seleccionados
gui-access-mode-selected-body = Solo pueden usarlo los equipos marcados a continuación.
gui-access-computers = Equipos autorizados a usarlo
gui-access-revoke-note = Un equipo que pierde el permiso se desconecta del dispositivo de inmediato.
gui-client-devices = Dispositivos
gui-client-devices-title = Dispositivos que { $name } puede usar
gui-client-devices-body = Marque los dispositivos compartidos que puede usar este equipo.
gui-client-devices-after-pairing = Solo los equipos autorizados pueden usar los dispositivos compartidos. Marque los dispositivos que puede usar este equipo; si lo omite, no podrá usar ninguno por ahora.
gui-no-shared-devices = Este equipo todavía no comparte ningún dispositivo.
gui-roles-title = Cómo se usa este equipo
gui-roles-body = Las pantallas de un uso que no está elegido permanecen ocultas. Añadir uno instala lo que necesite.
gui-role-server = Usar como servidor (compartir los dispositivos USB de este equipo)
gui-role-client = Usar como cliente (usar dispositivos USB de otros equipos)
gui-roles-client-note = Si es necesario, se instala el controlador usbip-win2; los dispositivos USB se detienen unos segundos y puede que haya que reiniciar Windows.
gui-roles-applying = Aplicando…
gui-reboot-required = Reinicie el equipo para terminar la configuración.
gui-used-by-waiting = { $name } lo está usando · { $count } esperando
gui-col-permissions = Permisos
gui-col-status = Estado
gui-in-use-title = En uso por
gui-nobody-using = Nadie está usando el dispositivo ahora mismo.
gui-since = desde las { $time }
gui-disconnect-user = Desconectar
gui-disconnected-note = { $name } fue desconectado. Si sigue solicitando el dispositivo, se reconectará en unos segundos; para mantenerlo alejado definitivamente, desmárquelo en la lista de la izquierda y guarde.
gui-queue-title = En espera ({ $count })
gui-queue-empty = Nadie está esperando.
gui-badge-using = usándolo
gui-badge-queued = en cola ({ $position })
gui-handover-title = Traspaso automático
gui-handover-default-on = Predeterminado (activado, { $seconds } s)
gui-handover-default-off = Predeterminado (desactivado)
gui-handover-on = Activado
gui-handover-off = Desactivado
gui-handover-before = Mientras otro equipo espera, tras
gui-handover-after = segundos sin uso pasa al siguiente.
gui-kind-storage = Almacenamiento
gui-kind-input = Teclado / ratón
gui-kind-printer = Impresora
gui-kind-dongle = Dongle de licencia
gui-kind-other = Otro dispositivo
gui-web-title = Interfaz web
gui-web-body = Administre este equipo desde un navegador, con una contraseña.
gui-web-confirm-off = ¿Desactivar la interfaz web? Esta página dejará de funcionar. Se puede volver a activar en la aplicación de escritorio o con «usbnexus web enable» en el propio equipo.
gui-web-confirm-local = ¿Permitir el acceso solo desde este equipo? Esta página se abrió desde la red y dejará de funcionar.
gui-web-turned-off = La interfaz web está desactivada. Se puede volver a activar en la aplicación de escritorio o con «usbnexus web enable» en el propio equipo.
gui-web-enabled = Interfaz web activa
gui-web-access-local = Solo este equipo
gui-web-access-network = Toda la red
gui-web-port = Puerto
gui-web-new-password = Nueva contraseña
gui-web-repeat-password = Nueva contraseña (de nuevo)
gui-web-password-keep = Al menos 8 caracteres. Déjelo en blanco para conservar la contraseña actual.
gui-web-password-required = Al menos 8 caracteres.
gui-web-mismatch = Las contraseñas no coinciden.
gui-web-open-at = Abrir en:
gui-web-fingerprint = El navegador advierte sobre el certificado; su huella digital es { $fp }.
gui-web-trusted = Los navegadores de este equipo confían en el certificado; otros equipos mostrarán una advertencia. Su huella digital es { $fp }.
gui-policy-title = Quién puede usar los dispositivos compartidos
gui-policy-body = Los equipos siempre deben emparejarse primero. Esta es la configuración predeterminada para cada dispositivo compartido; cada dispositivo puede tener su propia configuración.
gui-policy-first-title = ¿Quién puede usar sus dispositivos compartidos?
gui-policy-first-body = Los equipos siempre deben emparejarse primero con un PIN. Elija lo que pueden hacer los equipos emparejados:
gui-policy-later = Puede cambiar esto más adelante en Configuración.
gui-policy-open = Todos los equipos emparejados
gui-policy-open-body = Cualquier equipo emparejado puede usar cualquier dispositivo compartido.
gui-policy-restricted = Solo los equipos autorizados
gui-policy-restricted-body = Usted elige, por dispositivo, qué equipos emparejados pueden usarlo.
gui-retention-title = Registro de uso
gui-retention-body = Se registran los emparejamientos, los PIN incorrectos, el uso de dispositivos y las solicitudes rechazadas. Las entradas antiguas se eliminan automáticamente.
gui-retention-days = Conservar las entradas durante (días)
gui-settings-title = Configuración
gui-startup-title = Inicio
gui-startup-body = Al cerrar la ventana, USB Nexus permanece en el área de notificación; el servicio sigue compartiendo y manteniendo las conexiones de todos modos.
gui-startup-enabled = Iniciar automáticamente al iniciar sesión
gui-tray-open = Abrir USB Nexus
gui-tray-quit = Salir
gui-history-title = Historial
gui-history-subtitle = Quién usó los dispositivos de este equipo y cuándo. Las entradas se conservan durante { $days } días.
gui-history-empty = Todavía no se ha registrado nada.
gui-history-export = Exportar CSV
gui-history-time = Hora
gui-history-event = Evento
gui-history-computer = Equipo
gui-history-device = Dispositivo
gui-history-device-id = ID de dispositivo
gui-history-duration = Duración
gui-history-paired = Emparejado
gui-history-pairing-failed = PIN incorrecto
gui-history-attached = Empezó a usarlo
gui-history-detached = Dejó de usarlo
gui-history-denied = Rechazado: sin permiso

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = ¿Cómo va a usar USB Nexus?
setup-roles-subtitle = Elija lo que hará este equipo.
setup-role-server = Usar como servidor
setup-role-client = Usar como cliente
setup-role-web = Acceso web
setup-usbip-install-note = También se instalará el controlador usbip-win2. Los dispositivos USB se detienen unos segundos durante la instalación y después hay que reiniciar Windows.
setup-usbip-update-note = El controlador usbip-win2 instalado es demasiado antiguo y se actualizará. Los dispositivos USB se detienen unos segundos durante la instalación y después hay que reiniciar Windows.
setup-usbip-present-note = El controlador usbip-win2 ya está instalado en este equipo.
setup-usbip-failed = No se pudo instalar el controlador usbip-win2. Puede volver a ejecutar la instalación de USB Nexus más adelante para reintentarlo.
setup-service-failed = No se pudo configurar el servicio de USB Nexus. Encontrará los detalles en el registro de instalación.
setup-web-title = Interfaz web
setup-web-subtitle = Configuración para administrar este equipo desde un navegador.
setup-web-access = Acceso:
setup-web-local = Solo este equipo
setup-web-network = Toda la red
setup-web-port = Puerto:
setup-web-port-free = ✓ El puerto está disponible
setup-web-port-busy = ✗ Este puerto lo está usando otro programa
setup-web-port-invalid = ✗ Introduzca un número entre 1 y 65535
setup-web-password = Contraseña:
setup-web-password-repeat = Contraseña (de nuevo):
setup-web-password-hint = Al menos 8 caracteres.
setup-web-password-keep = Al menos 8 caracteres. Déjelo en blanco para conservar la contraseña actual.
