# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Chinese (Simplified).
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = 安全的 USB over IP：通过网络共享 USB 设备，具备加密、配对与自动重新连接功能。
arg-lang = 界面语言（例如 en、tr）。默认使用系统语言。
arg-state-dir = 存放密钥和已配对计算机信息的目录。
arg-name = 本计算机在其他计算机上显示的名称。
arg-verbose = 显示详细日志信息。

cmd-daemon-about = 运行 USB Nexus 服务（供桌面应用使用）。
arg-allow-all-users = 让本机的所有用户都能控制该服务（默认仅限 usbnexus 用户组成员）。
arg-socket = 服务本地控制套接字的路径。
cmd-service-about = 安装或移除 USB Nexus Windows 服务。
cmd-install-about = 安装服务，立即启动并在每次开机时启动（请以管理员身份运行）。
cmd-uninstall-about = 停止并移除该服务。
cmd-serve-about = 共享本计算机的 USB 设备。
arg-export = 要共享的设备的 bus id（可重复指定）。参见：usbnexus local
arg-listen = 要监听的地址和端口。
arg-pair = 启动时打开配对窗口并显示 PIN 码。
arg-no-mdns = 不在本地网络上公告此服务器。

cmd-pin-about = 显示 PIN 码，用于将新计算机与正在运行的服务器配对。
arg-seconds = PIN 码的有效时间（秒）。

cmd-local-about = 列出连接到本计算机的 USB 设备。
cmd-discover-about = 在本地网络中查找 USB Nexus 服务器。
arg-timeout = 搜索的持续时间（秒）。
cmd-pair-about = 使用服务器显示的 PIN 码进行配对。
arg-server = 服务器：已配对的名称、证书指纹或 host[:port]。
arg-pin = 服务器显示的 PIN 码（未指定时会提示输入）。
cmd-list-about = 列出某个服务器共享的设备。
cmd-allow-user-about = 允许某用户通过桌面应用控制该服务（将其加入 usbnexus 用户组）。
arg-user = 用户名。
allow-user-invalid = “{ $user }”不是有效的用户名。
allow-user-failed = 无法将“{ $user }”加入 usbnexus 用户组。
allow-user-done = “{ $user }”现在可以控制 USB Nexus 服务了。
allow-user-relogin = 需要重新登录后才会生效。
cmd-attach-about = 在本计算机上使用远程设备；连接断开后会自动重新连接。
arg-busid = 远程设备的 bus id。
cmd-peers-about = 列出已配对的计算机。
cmd-forget-about = 移除一台已配对的计算机。
arg-peer = 已配对计算机的名称或证书指纹。

## General

error-prefix = 错误：{ $detail }
hint-root = 此操作需要管理员权限，请使用 sudo 重试。
unsupported-os = 此操作系统尚不支持该命令。
yes = 是
no = 否

service-installed = USB Nexus 服务已安装并启动。
service-removed = USB Nexus 服务已移除。

## Server

serve-started = 服务器“{ $name }”正在 { $addr } 上监听。
serve-fingerprint = 证书指纹：{ $fp }
serve-exporting = 已共享的设备：
serve-no-exports = 尚未共享任何设备。请添加 --export BUSID（使用 usbnexus local 查看设备列表）。
serve-pairing-pin = 配对 PIN 码：{ $pin }（有效期 { $seconds } 秒）
serve-paired = 已与“{ $name }”配对。
serve-pairing-failed = 来自 { $addr } 的配对尝试失败。
serve-exported = { $busid } 现在正被“{ $client }”使用。
serve-released = { $busid } 已被“{ $client }”释放。
serve-stopping = 正在停止；正在将设备归还给它们的常规驱动程序……
serve-mdns-failed = 本地网络发现功能不可用：{ $detail }
serve-bind-failed = 无法准备共享 { $busid }：{ $detail }

## Pairing

pin-show = 配对 PIN 码：{ $pin }
pin-hint = 请在另一台计算机上，在 { $seconds } 秒内运行“usbnexus pair { $name }”。
pin-no-server = 未找到正在运行的 USB Nexus 服务器。请使用以下命令启动一个：usbnexus serve
pair-enter-pin = 请输入服务器显示的 PIN 码：
pair-ok = 已与“{ $name }”配对（{ $fp }）。
pair-already = 已经与“{ $name }”配对。

## Devices

local-header = 本计算机上的 USB 设备：
local-empty = 未找到 USB 设备。
list-header = “{ $name }”共享的设备：
list-empty = 该服务器未共享任何设备。
list-in-use = 使用中
col-busid = BUS ID
col-id = VID:PID
col-speed = 速度
col-product = 产品
col-driver = 驱动程序
col-state = 状态
col-name = 名称
col-fingerprint = 证书指纹
col-address = 地址

## Discovery and peers

discover-searching = 正在搜索本地网络……
discover-none = 未找到 USB Nexus 服务器。
discover-paired = 已配对
peers-empty = 尚无已配对的计算机。
forget-ok = “{ $name }”已被移除。
forget-unknown = 没有与“{ $peer }”匹配的已配对计算机。

## Attaching

attach-connecting = 正在连接到 { $addr }……
attach-attached = { $busid } 已连接（虚拟端口 { $port }）。
attach-disconnected = 连接已断开：{ $reason }
attach-retrying = 将在 { $seconds } 秒后重新连接……
attach-detached = 设备已断开连接。
attach-stop-hint = 按 Ctrl+C 断开连接。

## Errors

err-pairing-required = “{ $name }”尚未与本计算机配对。请运行：usbnexus pair { $target }
err-not-found = 在网络中找不到该服务器。
err-version = 该服务器运行的 USB Nexus 版本不兼容。
err-not-trusted = 本计算机尚未与该服务器配对。
err-pairing-closed = 服务器未开放配对。请在服务器上运行：usbnexus pin
err-pairing-failed = 配对失败，请检查 PIN 码并重试。
err-no-such-device = 该服务器未共享此设备。
err-device-busy = 该设备正被另一台计算机使用。
err-internal = 服务器报告了一个内部错误。
err-protocol = 服务器返回了意外的响应。
err-connection-lost = 与该计算机的连接已断开。
err-unsupported = 此操作系统尚不支持此功能。
err-driver-missing = 尚未安装 usbip-win2 驱动程序。请重新运行 USB Nexus 安装程序，并同意安装 usbip-win2。
err-driver-outdated = 已安装的 usbip-win2 驱动程序过旧。请重新运行 USB Nexus 安装程序，并同意更新 usbip-win2。
err-vboxusb-missing = 未安装共享所需的 VirtualBox USB 驱动程序。请重新安装 USB Nexus。
err-device-in-use-by-os = 操作系统正在用自己的驱动程序使用该设备，因此无法从本计算机共享它。
err-unreachable = 无法连接到该计算机。请检查它是否已开机并已连接到网络。
err-cancelled = 操作已取消。
err-permission-denied = USB Nexus 没有所需的权限。
err-invalid = 无法理解该请求。
err-other = 发生了错误：{ $detail }

## Help layout

help-usage = 用法：
help-arguments = 参数
help-options = 选项
help-commands = 命令
arg-help = 显示帮助信息。
arg-version = 显示版本信息。

## Peers

peers-servers = 已配对的服务器（本计算机可以使用它们的设备）：
peers-clients = 已配对的客户端（允许使用本计算机的设备）：

## Desktop app

gui-nav-this-computer = 本计算机
gui-nav-network = 网络中的计算机
gui-nav-connected = 已连接的设备
gui-nav-paired = 已配对的计算机
gui-language = 语言
gui-this-title = 本计算机上的设备
gui-this-subtitle = 选择允许其他计算机使用哪些设备。
gui-share = 共享
gui-shared = 已共享
gui-not-shared = 未共享
gui-used-by = 正被 { $name } 使用
gui-no-local-devices = 未在本计算机上找到 USB 设备。
gui-unnamed-device = USB 设备
gui-pair-new = 配对新计算机
gui-pin-title = 配对 PIN 码
gui-pin-body = 请在另一台计算机上输入此 PIN 码。
gui-pin-remaining = 还有 { $seconds } 秒有效
gui-pin-expired = PIN 码已过期。
gui-pin-new = 新建 PIN 码
gui-pin-stop = 停止配对
gui-close = 关闭
gui-cancel = 取消
gui-network-title = 网络中的计算机
gui-network-subtitle = 在本地网络中找到的 USB Nexus 计算机。
gui-refresh = 刷新
gui-searching = 正在搜索网络……
gui-none-found = 未找到任何计算机。请确认 USB Nexus 正在另一台计算机上运行，或按地址添加。
gui-add-by-address = 按地址添加
gui-address = 地址
gui-address-hint = 例如 192.168.1.20
gui-pair = 配对
gui-paired = 已配对
gui-pair-title = 与 { $name } 配对
gui-pair-body = 在 { $name } 上选择“配对新计算机”，然后输入其中显示的 PIN 码。
gui-pin = PIN 码
gui-pair-done = 已与 { $name } 配对。
gui-devices-of = { $name } 共享的设备
gui-no-remote-devices = 本计算机未共享任何设备。
gui-connect = 连接
gui-disconnect = 断开连接
gui-in-use-elsewhere = 正被另一台计算机使用
gui-connected-here = 已连接到本计算机
gui-back = 返回
gui-connected-title = 已连接到本计算机的设备
gui-connected-subtitle = 远程设备会保持连接，网络断开后会自动重新连接。
gui-connected-empty = 没有已连接的远程设备。打开“网络”即可连接一个。
gui-on-computer = 位于 { $name }
gui-state-connecting = 正在连接……
gui-state-attached = 已连接
gui-state-retrying = 将在 { $seconds } 秒后重新连接
gui-state-stopped = 已断开
gui-state-failed = 失败
gui-reconnect = 重新连接
gui-remove = 移除
gui-paired-title = 已配对的计算机
gui-paired-subtitle = 计算机只需通过 PIN 码配对一次，之后会自动被识别。
gui-paired-servers = 可以使用其设备的计算机
gui-paired-clients = 可以使用本计算机设备的计算机
gui-paired-empty = 暂无。
gui-remove-confirm = 是否移除 { $name }？需要重新配对才能再次使用它。
gui-fingerprint = 证书指纹
gui-service-denied-title = 此用户无法控制 USB Nexus 服务
gui-service-denied-body = 只有管理员和 usbnexus 用户组成员可以控制。允许此用户（会要求输入管理员密码），或以管理员身份运行：
gui-service-denied-command = 命令：
gui-service-denied-user = 用户
gui-grant-access = 允许此用户
gui-grant-access-done = 已授予权限。
gui-setup-kernel-modules = 缺少 USB over IP 所需的内核模块：{ $modules }。请安装它们；服务会自行加载：
gui-setup-kernel-modules-nocmd = 大多数发行版的内核软件包中已包含这些模块（Ubuntu：linux-modules-extra；Fedora：kernel-modules-extra）。
gui-service-down-title = USB Nexus 服务未在运行
gui-service-down-body = 请启动该服务；此窗口会自动连接到它。
gui-service-down-linux = 在 Linux 上运行：
gui-service-down-windows = 在 Windows 上以管理员身份运行：
gui-service-down-macos = 在 macOS 上运行：
gui-retry = 重试
gui-details = 详情
err-service-unavailable = 无法连接到 USB Nexus 服务。

## Web interface

gui-web-login-title = 登录 { $name }
gui-web-password = 密码
gui-web-sign-in = 登录
gui-web-sign-out = 退出登录
gui-web-wrong-password = 密码错误。
gui-web-locked = 尝试次数过多，请在 { $seconds } 秒后重试。
cmd-web-about = 开启或关闭网页界面。
cmd-enable-about = 开启网页界面（首次使用会要求设置密码）。
cmd-disable-about = 关闭网页界面。
cmd-password-about = 更改网页界面的密码。
cmd-status-about = 显示网页界面是否已开启及其地址。
arg-lan = 允许网络中的其他计算机访问。
arg-port = 网页界面的 TCP 端口。
web-on = 网页界面已开启：
web-off = 网页界面已关闭。
web-local-only = 只能从本计算机打开。使用 --lan 可允许其他计算机访问。
web-fingerprint = 浏览器会提示证书警告；证书指纹应为：{ $fp }
web-trusted = 本计算机上的浏览器信任该证书；其他计算机会显示警告。证书指纹为：{ $fp }
web-not-running = 网页界面已启用但无法启动：{ $detail }
web-password-prompt = 新的网页界面密码：
web-password-repeat = 请再次输入密码：
web-password-mismatch = 两次输入的密码不一致。
web-password-set = 网页界面密码已更改。
err-weak-password = 密码长度必须至少为 8 个字符。
err-password-required = 请先设置网页界面密码：usbnexus web password
err-forbidden = 只能在该计算机本机上更改此设置。
err-not-logged-in = 请重新登录。

## Hotplug, access control and usage log

arg-device = 设备：标识或 bus id（参见：usbnexus list 服务器）。
cmd-policy-about = 显示或选择谁可以使用共享设备。
arg-policy = open：每台已配对计算机都可使用每个共享设备；restricted：仅允许每个设备单独授权的计算机使用。
arg-no-server = 不设置共享本计算机的 USB 设备。
arg-no-client = 不设置使用其他计算机的 USB 设备。
arg-web = 网页界面：off（关闭）、local（仅本计算机）或 network（整个网络）。
arg-web-port = 网页界面的端口（默认 3242）。
arg-web-password-file = 包含新网页界面密码的文件。
cmd-history-about = 显示本计算机共享设备的使用记录。
arg-csv = 以 CSV 格式打印所有记录（例如用于保存到文件）。
arg-limit = 要显示的记录数量。
col-device-id = 设备标识
col-time = 时间（UTC）
col-event = 事件
col-computer = 计算机
col-duration = 持续时间
state-unplugged = 未插入
state-no-permission = 无权限
serve-denied = “{ $client }”无权使用 { $busid }。
attach-waiting-device = 服务器上未插入该设备；正在等待……
attach-queued = 该设备正被另一台计算机使用；本机在队列中排第 { $position } 位。
policy-open = 每台已配对计算机都可使用每个共享设备（开放）。
policy-restricted = 已配对计算机只能使用被允许使用的设备（受限）。
history-header = 使用记录（记录保留 { $days } 天）：
history-empty = 使用记录为空。
history-paired = 已配对
history-pairing-failed = PIN 码错误
history-attached = 开始使用
history-detached = 停止使用
history-denied = 已拒绝（无权限）
err-access-denied = 本计算机无权使用该设备。

gui-nav-history = 历史记录
gui-nav-settings = 设置
gui-not-plugged-in = 未插入
gui-tracked-by-port = 按端口跟踪
gui-tracked-by-port-hint = 此设备没有序列号，因此系统会通过它所插入的 USB 端口来识别它。请将它插回同一个端口。
gui-no-permission = 无权限
gui-no-permission-hint = 该计算机的所有者尚未允许本计算机使用该设备。
gui-connect-when-plugged-in = 设备插入后会自动连接。
gui-state-waiting-device = 正在等待设备
gui-state-queued = 正被其他计算机使用；在队列中排第 { $position } 位
gui-save = 保存
gui-saved = 已保存。
gui-skip = 跳过
gui-access-title = 谁可以使用此设备
gui-access-everyone = 所有已配对计算机
gui-access-some = 选定的计算机（{ $count }）
gui-access-nobody = 暂无计算机
gui-access-mode-default = 遵循默认设置
gui-access-default-open = 当前：所有已配对计算机。
gui-access-default-restricted = 当前：仅下面选定的计算机。
gui-access-mode-open = 所有已配对计算机
gui-access-mode-open-body = 与本机配对的每台计算机都可以使用它。
gui-access-mode-selected = 仅选定的计算机
gui-access-mode-selected-body = 只有下面勾选的计算机可以使用它。
gui-access-computers = 允许使用它的计算机
gui-access-revoke-note = 一旦某计算机失去权限，会立即断开它与该设备的连接。
gui-client-devices = 设备
gui-client-devices-title = { $name } 可以使用的设备
gui-client-devices-body = 勾选允许本计算机使用的共享设备。
gui-client-devices-after-pairing = 只有获得允许的计算机才能使用共享设备。请勾选本计算机可以使用的设备；如果跳过此步骤，本机目前将无法使用任何设备。
gui-no-shared-devices = 本计算机尚未共享任何设备。
gui-roles-title = 本计算机的使用方式
gui-roles-body = 未选择的用途对应的界面会被隐藏。添加某种用途会安装其所需的组件。
gui-role-server = 用作服务器（共享本计算机的 USB 设备）
gui-role-client = 用作客户端（使用其他计算机的 USB 设备）
gui-roles-client-note = 如有需要，会安装 usbip-win2 驱动程序；USB 设备会中断几秒钟，Windows 可能需要重新启动。
gui-roles-applying = 正在应用……
gui-reboot-required = 请重新启动计算机以完成设置。
gui-used-by-waiting = { $name } 正在使用它 · { $count } 台计算机在等待
gui-col-permissions = 权限
gui-col-status = 状态
gui-in-use-title = 使用者
gui-nobody-using = 目前没有人在使用该设备。
gui-since = 自 { $time } 起
gui-disconnect-user = 断开连接
gui-disconnected-note = { $name } 已断开连接。如果它一直请求该设备，会在几秒内重新连接；要永久阻止它，请在左侧列表中取消勾选并保存。
gui-queue-title = 排队等待（{ $count }）
gui-queue-empty = 没有等待的计算机。
gui-badge-using = 使用中
gui-badge-queued = 排队中（第 { $position } 位）
gui-handover-title = 自动移交
gui-handover-default-on = 默认（开启，{ $seconds } 秒）
gui-handover-default-off = 默认（关闭）
gui-handover-on = 开启
gui-handover-off = 关闭
gui-handover-before = 当有其他计算机等待时，闲置
gui-handover-after = 秒后即移交给队列中的下一台。
gui-kind-storage = 存储设备
gui-kind-input = 键盘 / 鼠标
gui-kind-printer = 打印机
gui-kind-dongle = 加密狗
gui-kind-other = 其他设备
gui-web-title = 网页界面
gui-web-body = 通过浏览器管理本计算机，需要密码。
gui-web-confirm-off = 是否关闭网页界面？此页面将停止工作。可以在桌面应用中，或在本机上运行“usbnexus web enable”重新开启。
gui-web-confirm-local = 是否仅允许本计算机访问？此页面是从网络打开的，之后将无法继续使用。
gui-web-turned-off = 网页界面已关闭。可以在桌面应用中，或在本机上运行“usbnexus web enable”重新开启。
gui-web-enabled = 网页界面已开启
gui-web-access-local = 仅本计算机
gui-web-access-network = 整个网络
gui-web-port = 端口
gui-web-new-password = 新密码
gui-web-repeat-password = 新密码（再次输入）
gui-web-password-keep = 至少 8 个字符。留空则保留当前密码。
gui-web-password-required = 至少 8 个字符。
gui-web-mismatch = 两次输入的密码不一致。
gui-web-open-at = 访问地址：
gui-web-fingerprint = 浏览器会提示证书警告；证书指纹为 { $fp }。
gui-web-trusted = 本计算机上的浏览器信任该证书；其他计算机会显示警告。证书指纹为 { $fp }。
gui-policy-title = 谁可以使用共享设备
gui-policy-body = 计算机始终需要先完成配对。此设置是所有共享设备的默认值，每个设备也可单独覆盖。
gui-policy-first-title = 谁可以使用您共享的设备？
gui-policy-first-body = 计算机始终需要先通过 PIN 码配对。请选择已配对计算机的权限：
gui-policy-later = 您可以稍后在“设置”中更改此项。
gui-policy-open = 所有已配对计算机
gui-policy-open-body = 每台已配对计算机都可以使用每个共享设备。
gui-policy-restricted = 仅允许的计算机
gui-policy-restricted-body = 您可以为每个设备分别选择哪些已配对计算机可以使用它。
gui-retention-title = 使用记录
gui-retention-body = 配对、错误 PIN 码、设备使用及被拒绝的请求都会被记录。较旧的记录会自动删除。
gui-retention-days = 记录保留时长（天）
gui-settings-title = 设置
gui-startup-title = 启动
gui-startup-body = 关闭窗口后，USB Nexus 仍会保留在通知区域；无论如何，共享和连接都由服务负责维持。
gui-startup-enabled = 登录时自动启动
gui-tray-open = 打开 USB Nexus
gui-tray-quit = 退出
gui-history-title = 历史记录
gui-history-subtitle = 谁在何时使用了本计算机的设备。记录保留 { $days } 天。
gui-history-empty = 尚无任何记录。
gui-history-export = 导出为 CSV
gui-history-time = 时间
gui-history-event = 事件
gui-history-computer = 计算机
gui-history-device = 设备
gui-history-device-id = 设备标识
gui-history-duration = 持续时间
gui-history-paired = 已配对
gui-history-pairing-failed = PIN 码错误
gui-history-attached = 开始使用
gui-history-detached = 停止使用
gui-history-denied = 已拒绝：无权限

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = 您将如何使用 USB Nexus？
setup-roles-subtitle = 请选择本计算机的用途。
setup-role-server = 用作服务器
setup-role-client = 用作客户端
setup-role-web = 网页访问
setup-usbip-install-note = 同时会安装 usbip-win2 驱动程序。安装期间 USB 设备会中断几秒钟，之后需要重新启动 Windows。
setup-usbip-update-note = 已安装的 usbip-win2 驱动程序过旧，将进行更新。安装期间 USB 设备会中断几秒钟，之后需要重新启动 Windows。
setup-usbip-present-note = 本计算机已安装 usbip-win2 驱动程序。
setup-usbip-failed = usbip-win2 驱动程序安装失败。您可以稍后重新运行 USB Nexus 安装程序进行重试。
setup-service-failed = USB Nexus 服务设置失败。详情请查看安装日志。
setup-web-title = 网页界面
setup-web-subtitle = 用于从浏览器管理本计算机的设置。
setup-web-access = 访问范围：
setup-web-local = 仅本计算机
setup-web-network = 整个网络
setup-web-port = 端口：
setup-web-port-free = ✓ 该端口可用
setup-web-port-busy = ✗ 该端口已被另一个程序占用
setup-web-port-invalid = ✗ 请输入 1 到 65535 之间的数字
setup-web-password = 密码：
setup-web-password-repeat = 密码（再次输入）：
setup-web-password-hint = 至少 8 个字符。
setup-web-password-keep = 至少 8 个字符。留空则保留当前密码。
