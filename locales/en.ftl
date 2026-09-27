# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — English (reference language).
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = Secure USB over IP: share USB devices across the network with encryption, pairing and automatic reconnection.
arg-lang = Interface language (e.g. en, tr). Defaults to the system language.
arg-state-dir = Directory for keys and paired computers.
arg-name = Name of this computer as shown to others.
arg-verbose = Show detailed log messages.

cmd-daemon-about = Run the USB Nexus service (used by the desktop app).
arg-allow-all-users = Let every local user control the service (default: members of the usbnexus group).
arg-socket = Path of the service's local control socket.
cmd-service-about = Install or remove the USB Nexus Windows service.
cmd-install-about = Install the service, start it now and at every boot (run as administrator).
cmd-uninstall-about = Stop and remove the service.
cmd-serve-about = Share USB devices of this computer.
arg-export = Bus ID of a device to share (repeatable). See: usbnexus local
arg-listen = Address and port to listen on.
arg-pair = Open a pairing window at start-up and show the PIN.
arg-no-mdns = Do not announce this server on the local network.

cmd-pin-about = Show a PIN to pair a new computer with the running server.
arg-seconds = How long the PIN stays valid, in seconds.

cmd-local-about = List USB devices connected to this computer.
cmd-discover-about = Find USB Nexus servers on the local network.
arg-timeout = How long to search, in seconds.
cmd-pair-about = Pair with a server using the PIN it shows.
arg-server = Server: paired name, fingerprint, or host[:port].
arg-pin = PIN shown by the server (asked for if omitted).
cmd-list-about = List devices shared by a server.
cmd-attach-about = Use a remote device on this computer; reconnects automatically.
arg-busid = Bus ID of the remote device.
cmd-peers-about = List paired computers.
cmd-forget-about = Remove a paired computer.
arg-peer = Name or fingerprint of the paired computer.

## General

error-prefix = Error: { $detail }
hint-root = This operation needs administrator rights. Try again with sudo.
unsupported-os = This command is not supported on this operating system yet.
yes = yes
no = no

service-installed = The USB Nexus service was installed and started.
service-removed = The USB Nexus service was removed.

## Server

serve-started = Server “{ $name }” is listening on { $addr }.
serve-fingerprint = Fingerprint: { $fp }
serve-exporting = Shared devices:
serve-no-exports = No devices are shared. Add --export BUSID (list devices with: usbnexus local).
serve-pairing-pin = Pairing PIN: { $pin } (valid for { $seconds } seconds)
serve-paired = Paired with “{ $name }”.
serve-pairing-failed = Failed pairing attempt from { $addr }.
serve-exported = { $busid } is now used by “{ $client }”.
serve-released = { $busid } was released by “{ $client }”.
serve-stopping = Stopping; returning devices to their normal drivers…
serve-mdns-failed = Local network discovery is unavailable: { $detail }
serve-bind-failed = Could not prepare { $busid } for sharing: { $detail }

## Pairing

pin-show = Pairing PIN: { $pin }
pin-hint = On the other computer run “usbnexus pair { $name }” within { $seconds } seconds.
pin-no-server = No running USB Nexus server was found. Start one with: usbnexus serve
pair-enter-pin = Enter the PIN shown on the server:
pair-ok = Paired with “{ $name }” ({ $fp }).
pair-already = Already paired with “{ $name }”.

## Devices

local-header = USB devices on this computer:
local-empty = No USB devices found.
list-header = Devices shared by “{ $name }”:
list-empty = This server does not share any devices.
list-in-use = in use
col-busid = BUS ID
col-id = VID:PID
col-speed = SPEED
col-product = PRODUCT
col-driver = DRIVER
col-state = STATE
col-name = NAME
col-fingerprint = FINGERPRINT
col-address = ADDRESS

## Discovery and peers

discover-searching = Searching the local network…
discover-none = No USB Nexus servers were found.
discover-paired = paired
peers-empty = No paired computers yet.
forget-ok = “{ $name }” was removed.
forget-unknown = No paired computer matches “{ $peer }”.

## Attaching

attach-connecting = Connecting to { $addr }…
attach-attached = { $busid } is attached (virtual port { $port }).
attach-disconnected = Connection lost: { $reason }
attach-retrying = Reconnecting in { $seconds } seconds…
attach-detached = The device was detached.
attach-stop-hint = Press Ctrl+C to detach.

## Errors

err-pairing-required = “{ $name }” is not paired with this computer yet. Run: usbnexus pair { $target }
err-not-found = The server could not be found on the network.
err-version = The server runs an incompatible version of USB Nexus.
err-not-trusted = This computer is not paired with the server.
err-pairing-closed = Pairing is not open on the server. On the server run: usbnexus pin
err-pairing-failed = Pairing failed. Check the PIN and try again.
err-no-such-device = The server does not share this device.
err-device-busy = The device is being used by another computer.
err-internal = The server reported an internal error.
err-protocol = Unexpected response from the server.
err-connection-lost = The connection to the computer was lost.
err-unsupported = This is not available on this operating system yet.
err-driver-missing = The usbip-win2 driver is not installed. Run the USB Nexus setup again and accept installing usbip-win2.
err-driver-outdated = The installed usbip-win2 driver is too old. Run the USB Nexus setup again and accept updating usbip-win2.
err-vboxusb-missing = The VirtualBox USB drivers needed for sharing are not installed. Reinstall USB Nexus.
err-device-in-use-by-os = The operating system is using this device with its own driver, so it cannot be shared from this computer.
err-unreachable = The computer could not be reached. Check that it is on and connected to the network.
err-permission-denied = USB Nexus does not have the permissions it needs.
err-invalid = The request was not understood.
err-other = Something went wrong: { $detail }

## Help layout

help-usage = Usage:
help-arguments = Arguments
help-options = Options
help-commands = Commands
arg-help = Show help.
arg-version = Show version.

## Peers

peers-servers = Paired servers (this computer can use their devices):
peers-clients = Paired clients (allowed to use this computer's devices):

## Desktop app

gui-nav-this-computer = This computer
gui-nav-network = Computers on the network
gui-nav-connected = Connected devices
gui-nav-paired = Paired computers
gui-language = Language
gui-this-title = Devices on this computer
gui-this-subtitle = Choose which devices other computers may use.
gui-share = Share
gui-shared = Shared
gui-not-shared = Not shared
gui-used-by = In use by { $name }
gui-no-local-devices = No USB devices were found on this computer.
gui-unnamed-device = USB device
gui-pair-new = Pair a new computer
gui-pin-title = Pairing PIN
gui-pin-body = Enter this PIN on the other computer.
gui-pin-remaining = Valid for { $seconds } more seconds
gui-pin-expired = The PIN has expired.
gui-pin-new = New PIN
gui-pin-stop = Stop pairing
gui-close = Close
gui-cancel = Cancel
gui-network-title = Computers on the network
gui-network-subtitle = USB Nexus computers found on your local network.
gui-refresh = Refresh
gui-searching = Searching the network…
gui-none-found = No computers were found. Make sure USB Nexus is running on the other computer, or add it by address.
gui-add-by-address = Add by address
gui-address = Address
gui-address-hint = e.g. 192.168.1.20
gui-pair = Pair
gui-paired = Paired
gui-pair-title = Pair with { $name }
gui-pair-body = On { $name }, choose “Pair a new computer” and type the PIN shown there.
gui-pin = PIN
gui-pair-done = Paired with { $name }.
gui-devices-of = Devices shared by { $name }
gui-no-remote-devices = This computer does not share any devices.
gui-connect = Connect
gui-disconnect = Disconnect
gui-in-use-elsewhere = In use by another computer
gui-connected-here = Connected to this computer
gui-back = Back
gui-connected-title = Devices connected to this computer
gui-connected-subtitle = Remote devices stay connected and reconnect on their own if the network drops.
gui-connected-empty = No remote devices are connected. Open “Network” to connect one.
gui-on-computer = on { $name }
gui-state-connecting = Connecting…
gui-state-attached = Connected
gui-state-retrying = Reconnecting in { $seconds } s
gui-state-stopped = Disconnected
gui-state-failed = Failed
gui-reconnect = Reconnect
gui-remove = Remove
gui-paired-title = Paired computers
gui-paired-subtitle = Computers are paired once with a PIN and recognised automatically afterwards.
gui-paired-servers = Computers whose devices you can use
gui-paired-clients = Computers that can use your devices
gui-paired-empty = None yet.
gui-remove-confirm = Remove { $name }? You will need to pair again to use it.
gui-fingerprint = Fingerprint
gui-service-down-title = The USB Nexus service is not running
gui-service-down-body = Start the service; this window connects to it automatically.
gui-service-down-linux = On Linux run:
gui-service-down-windows = On Windows, as administrator, run:
gui-service-down-macos = On macOS run:
gui-retry = Try again
gui-details = Details
err-service-unavailable = The USB Nexus service could not be reached.

## Web interface

gui-web-login-title = Sign in to { $name }
gui-web-password = Password
gui-web-sign-in = Sign in
gui-web-sign-out = Sign out
gui-web-wrong-password = Wrong password.
gui-web-locked = Too many attempts. Try again in { $seconds } seconds.
cmd-web-about = Turn the web interface on or off.
cmd-enable-about = Turn the web interface on (asks for a password the first time).
cmd-disable-about = Turn the web interface off.
cmd-password-about = Change the web interface password.
cmd-status-about = Show whether the web interface is on and where.
arg-lan = Allow access from other computers on the network.
arg-port = TCP port of the web interface.
web-on = The web interface is on:
web-off = The web interface is off.
web-local-only = Only this computer can open it. Use --lan to allow other computers.
web-fingerprint = The browser will warn about the certificate; its fingerprint should be: { $fp }
web-not-running = The web interface is enabled but could not start: { $detail }
web-password-prompt = New web interface password:
web-password-repeat = Repeat the password:
web-password-mismatch = The passwords do not match.
web-password-set = The web interface password was changed.
err-weak-password = The password must be at least 8 characters long.
err-password-required = Set a web interface password first: usbnexus web password
err-forbidden = This can only be changed on the computer itself.
err-not-logged-in = Please sign in again.

## Hotplug, access control and usage log

arg-device = Device: identity or bus ID (see: usbnexus list SERVER).
cmd-policy-about = Show or choose who may use shared devices.
arg-policy = open: every paired computer may use every shared device; restricted: only computers allowed per device.
arg-no-server = Do not set up sharing this computer's USB devices.
arg-no-client = Do not set up using USB devices of other computers.
arg-web = Web interface: off, local (only this computer) or network (the whole network).
arg-web-port = Port of the web interface (default 3242).
arg-web-password-file = File containing the new web interface password.
cmd-history-about = Show the usage log of this computer's shared devices.
arg-csv = Print all entries as CSV (e.g. to save to a file).
arg-limit = Number of entries to show.
col-device-id = DEVICE ID
col-time = TIME (UTC)
col-event = EVENT
col-computer = COMPUTER
col-duration = DURATION
state-unplugged = not plugged in
state-no-permission = no permission
serve-denied = “{ $client }” is not allowed to use { $busid }.
attach-waiting-device = The device is not plugged in on the server; waiting for it…
attach-queued = Another computer is using the device; this one is number { $position } in the queue.
policy-open = Every paired computer may use every shared device (open).
policy-restricted = Paired computers may only use the devices they are allowed to use (restricted).
history-header = Usage log (entries are kept for { $days } days):
history-empty = The usage log is empty.
history-paired = paired
history-pairing-failed = wrong PIN
history-attached = started using
history-detached = stopped using
history-denied = refused (no permission)
err-access-denied = This computer is not allowed to use the device.

gui-nav-history = History
gui-nav-settings = Settings
gui-not-plugged-in = Not plugged in
gui-tracked-by-port = tracked by port
gui-tracked-by-port-hint = This device has no serial number, so it is recognised by the USB port it is plugged into. Plug it into the same port again.
gui-no-permission = No permission
gui-no-permission-hint = The owner of that computer has not allowed this computer to use the device.
gui-connect-when-plugged-in = Connects automatically as soon as the device is plugged in.
gui-state-waiting-device = Waiting for the device
gui-state-queued = In use elsewhere; number { $position } in the queue
gui-save = Save
gui-saved = Saved.
gui-skip = Skip
gui-access-title = Who may use this device
gui-access-everyone = All paired computers
gui-access-some = Selected computers ({ $count })
gui-access-nobody = No computer yet
gui-access-mode-default = Follow the default setting
gui-access-default-open = Currently: all paired computers.
gui-access-default-restricted = Currently: only the computers selected below.
gui-access-mode-open = All paired computers
gui-access-mode-open-body = Every computer paired with this one may use it.
gui-access-mode-selected = Only selected computers
gui-access-mode-selected-body = Only the computers ticked below may use it.
gui-access-computers = Computers allowed to use it
gui-access-revoke-note = A computer that loses permission is disconnected from the device immediately.
gui-client-devices = Devices
gui-client-devices-title = Devices { $name } may use
gui-client-devices-body = Tick the shared devices this computer may use.
gui-client-devices-after-pairing = Only allowed computers may use shared devices. Tick the devices this computer may use; if you skip this, it can use none for now.
gui-no-shared-devices = This computer does not share any devices yet.
gui-roles-title = How this computer is used
gui-roles-body = Screens for a use that is not chosen are hidden. Adding one installs what it needs.
gui-role-server = Use as a server (share this computer's USB devices)
gui-role-client = Use as a client (use USB devices of other computers)
gui-roles-client-note = If needed, the usbip-win2 driver is installed; USB devices stop for a few seconds and Windows may have to be restarted.
gui-roles-applying = Applying…
gui-reboot-required = Restart the computer to finish setting up.
gui-policy-title = Who may use shared devices
gui-policy-body = Computers always need to be paired first. This is the default for every shared device; each device can override it.
gui-policy-first-title = Who may use your shared devices?
gui-policy-first-body = Computers always need to be paired with a PIN first. Choose what paired computers may do:
gui-policy-later = You can change this later in Settings.
gui-policy-open = Every paired computer
gui-policy-open-body = Every paired computer may use every shared device.
gui-policy-restricted = Only allowed computers
gui-policy-restricted-body = You choose, per device, which paired computers may use it.
gui-retention-title = Usage log
gui-retention-body = Pairings, wrong PINs, device use and refused requests are recorded. Older entries are deleted automatically.
gui-retention-days = Keep entries for (days)
gui-settings-title = Settings
gui-history-title = History
gui-history-subtitle = Who used this computer's devices, and when. Entries are kept for { $days } days.
gui-history-empty = Nothing has been recorded yet.
gui-history-export = Export CSV
gui-history-time = Time
gui-history-event = Event
gui-history-computer = Computer
gui-history-device = Device
gui-history-device-id = Device ID
gui-history-duration = Duration
gui-history-paired = Paired
gui-history-pairing-failed = Wrong PIN
gui-history-attached = Started using
gui-history-detached = Stopped using
gui-history-denied = Refused: no permission

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = How will you use USB Nexus?
setup-roles-subtitle = Choose what this computer will do.
setup-role-server = Use as a server
setup-role-client = Use as a client
setup-role-web = Web access
setup-usbip-install-note = The usbip-win2 driver will also be installed. USB devices stop for a few seconds during the installation, and Windows must be restarted afterwards.
setup-usbip-update-note = The installed usbip-win2 driver is too old and will be updated. USB devices stop for a few seconds during the installation, and Windows must be restarted afterwards.
setup-usbip-present-note = The usbip-win2 driver is already installed on this computer.
setup-usbip-failed = The usbip-win2 driver could not be installed. You can run the USB Nexus setup again later to retry.
setup-service-failed = The USB Nexus service could not be set up. Details are in the installation log.
setup-web-title = Web interface
setup-web-subtitle = Settings for managing this computer from a browser.
setup-web-access = Access:
setup-web-local = Only this computer
setup-web-network = The whole network
setup-web-port = Port:
setup-web-port-free = ✓ The port is available
setup-web-port-busy = ✗ This port is used by another program
setup-web-port-invalid = ✗ Enter a number between 1 and 65535
setup-web-password = Password:
setup-web-password-repeat = Password (again):
setup-web-password-hint = At least 8 characters.
setup-web-password-keep = At least 8 characters. Leave empty to keep the current password.
