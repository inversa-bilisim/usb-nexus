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
err-driver-missing = The usbip-win2 driver is not installed. Install it from github.com/vadimgrn/usbip-win2/releases and try again.
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
gui-nav-network = Network
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
