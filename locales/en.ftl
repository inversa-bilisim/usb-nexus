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
