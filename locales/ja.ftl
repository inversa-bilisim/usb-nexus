# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Japanese.
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = セキュアな USB over IP：暗号化、ペアリング、自動再接続機能を備え、ネットワーク経由で USB デバイスを共有します。
arg-lang = インターフェースの言語（例：en、tr）。既定値はシステムの言語です。
arg-state-dir = 鍵とペアリング済みコンピューターを保存するディレクトリです。
arg-name = 他のコンピューターに表示される、このコンピューターの名前です。
arg-verbose = 詳細なログメッセージを表示します。

cmd-daemon-about = USB Nexus サービスを実行します（デスクトップアプリから使用されます）。
arg-allow-all-users = このコンピューターのすべてのローカルユーザーにサービスの操作を許可します（既定：usbnexus グループのメンバーのみ）。
arg-socket = サービスのローカル制御ソケットのパスです。
cmd-service-about = USB Nexus の Windows サービスをインストールまたは削除します。
cmd-install-about = サービスをインストールし、今すぐ、および起動時に開始します（管理者として実行してください）。
cmd-uninstall-about = サービスを停止して削除します。
cmd-serve-about = このコンピューターの USB デバイスを共有します。
arg-export = 共有するデバイスの bus id（複数回指定可）。参照：usbnexus local
arg-listen = 待ち受けるアドレスとポートです。
arg-pair = 起動時にペアリングウィンドウを開き、PIN を表示します。
arg-no-mdns = このサーバーをローカルネットワークに通知しません。

cmd-pin-about = 実行中のサーバーに新しいコンピューターをペアリングするための PIN を表示します。
arg-seconds = PIN が有効な時間（秒）です。

cmd-local-about = このコンピューターに接続されている USB デバイスを一覧表示します。
cmd-discover-about = ローカルネットワーク上の USB Nexus サーバーを検索します。
arg-timeout = 検索する時間（秒）です。
cmd-pair-about = サーバーに表示された PIN を使ってペアリングします。
arg-server = サーバー：ペアリング済みの名前、フィンガープリント、または host[:port] です。
arg-pin = サーバーに表示される PIN（省略時は入力を求められます）。
cmd-list-about = サーバーが共有しているデバイスを一覧表示します。
cmd-allow-user-about = デスクトップアプリからサービスを操作できるユーザーを追加します（usbnexus グループに追加します）。
arg-user = ユーザー名です。
allow-user-invalid = 「{ $user }」は有効なユーザー名ではありません。
allow-user-failed = 「{ $user }」を usbnexus グループに追加できませんでした。
allow-user-done = 「{ $user }」は USB Nexus サービスを操作できるようになりました。
allow-user-relogin = 再ログイン後に適用されます。
cmd-attach-about = リモートデバイスをこのコンピューターで使用します。自動的に再接続します。
arg-busid = リモートデバイスの bus id です。
cmd-peers-about = ペアリング済みのコンピューターを一覧表示します。
cmd-forget-about = ペアリング済みのコンピューターを削除します。
arg-peer = ペアリング済みコンピューターの名前またはフィンガープリントです。

## General

error-prefix = エラー：{ $detail }
hint-root = この操作には管理者権限が必要です。sudo を付けて再実行してください。
unsupported-os = このコマンドはこの OS ではまだサポートされていません。
yes = はい
no = いいえ

service-installed = USB Nexus サービスがインストールされ、開始されました。
service-removed = USB Nexus サービスが削除されました。

## Server

serve-started = サーバー「{ $name }」が { $addr } で待ち受けています。
serve-fingerprint = フィンガープリント：{ $fp }
serve-exporting = 共有中のデバイス：
serve-no-exports = 共有中のデバイスはありません。--export BUSID を指定してください（デバイス一覧：usbnexus local）。
serve-pairing-pin = ペアリング PIN：{ $pin }（{ $seconds } 秒間有効）
serve-paired = 「{ $name }」とペアリングしました。
serve-pairing-failed = { $addr } からのペアリングが失敗しました。
serve-exported = { $busid } は現在「{ $client }」が使用しています。
serve-released = { $busid } は「{ $client }」によって解放されました。
serve-stopping = 停止中です。デバイスを通常のドライバーに戻しています…
serve-mdns-failed = ローカルネットワーク検出を利用できません：{ $detail }
serve-bind-failed = { $busid } を共有用に準備できませんでした：{ $detail }

## Pairing

pin-show = ペアリング PIN：{ $pin }
pin-hint = { $seconds } 秒以内に、もう一方のコンピューターで「usbnexus pair { $name }」を実行してください。
pin-no-server = 実行中の USB Nexus サーバーが見つかりませんでした。次のコマンドで開始してください：usbnexus serve
pair-enter-pin = サーバーに表示された PIN を入力してください：
pair-ok = 「{ $name }」とペアリングしました（{ $fp }）。
pair-already = 「{ $name }」とはすでにペアリング済みです。

## Devices

local-header = このコンピューターの USB デバイス：
local-empty = USB デバイスが見つかりませんでした。
list-header = 「{ $name }」が共有しているデバイス：
list-empty = このサーバーはデバイスを共有していません。
list-in-use = 使用中
col-busid = BUS ID
col-id = VID:PID
col-speed = 速度
col-product = 製品名
col-driver = ドライバー
col-state = 状態
col-name = 名前
col-fingerprint = フィンガープリント
col-address = アドレス

## Discovery and peers

discover-searching = ローカルネットワークを検索しています…
discover-none = USB Nexus サーバーが見つかりませんでした。
discover-paired = ペアリング済み
peers-empty = ペアリング済みのコンピューターはまだありません。
forget-ok = 「{ $name }」を削除しました。
forget-unknown = 「{ $peer }」に一致するペアリング済みコンピューターはありません。

## Attaching

attach-connecting = { $addr } に接続しています…
attach-attached = { $busid } を接続しました（仮想ポート { $port }）。
attach-disconnected = 接続が切断されました：{ $reason }
attach-retrying = { $seconds } 秒後に再接続します…
attach-detached = デバイスを切断しました。
attach-stop-hint = Ctrl+C を押すと切断します。

## Errors

err-pairing-required = 「{ $name }」はまだこのコンピューターとペアリングされていません。次を実行してください：usbnexus pair { $target }
err-not-found = ネットワーク上でサーバーが見つかりませんでした。
err-version = サーバーは USB Nexus の互換性のないバージョンを実行しています。
err-not-trusted = このコンピューターはサーバーとペアリングされていません。
err-pairing-closed = サーバーでペアリングが開始されていません。サーバーで次を実行してください：usbnexus pin
err-pairing-failed = ペアリングに失敗しました。PIN を確認して再試行してください。
err-no-such-device = サーバーはこのデバイスを共有していません。
err-device-busy = デバイスは別のコンピューターが使用しています。
err-internal = サーバーが内部エラーを報告しました。
err-protocol = サーバーから予期しない応答がありました。
err-connection-lost = コンピューターへの接続が失われました。
err-unsupported = この機能はこの OS ではまだ利用できません。
err-driver-missing = usbip-win2 ドライバーがインストールされていません。USB Nexus のセットアップを再実行し、usbip-win2 のインストールを承諾してください。
err-driver-outdated = インストールされている usbip-win2 ドライバーが古すぎます。USB Nexus のセットアップを再実行し、usbip-win2 の更新を承諾してください。
err-vboxusb-missing = 共有に必要な VirtualBox USB ドライバーがインストールされていません。USB Nexus を再インストールしてください。
err-device-in-use-by-os = OS がこのデバイスを独自のドライバーで使用しているため、このコンピューターから共有できません。
err-unreachable = コンピューターに到達できませんでした。電源が入っており、ネットワークに接続されていることを確認してください。
err-cancelled = 操作はキャンセルされました。
err-permission-denied = USB Nexus に必要な権限がありません。
err-invalid = リクエストを認識できませんでした。
err-other = エラーが発生しました：{ $detail }

## Help layout

help-usage = 使用方法：
help-arguments = 引数
help-options = オプション
help-commands = コマンド
arg-help = ヘルプを表示します。
arg-version = バージョンを表示します。

## Peers

peers-servers = ペアリング済みサーバー（このコンピューターがそのデバイスを使用できます）：
peers-clients = ペアリング済みクライアント（このコンピューターのデバイスを使用できます）：

## Desktop app

gui-nav-this-computer = このコンピューター
gui-nav-network = ネットワーク上のコンピューター
gui-nav-connected = 接続中のデバイス
gui-nav-paired = ペアリング済みコンピューター
gui-language = 言語
gui-this-title = このコンピューターのデバイス
gui-this-subtitle = 他のコンピューターが使用できるデバイスを選択してください。
gui-share = 共有
gui-shared = 共有中
gui-not-shared = 未共有
gui-used-by = { $name } が使用中
gui-no-local-devices = このコンピューターに USB デバイスが見つかりませんでした。
gui-unnamed-device = USB デバイス
gui-pair-new = 新しいコンピューターをペアリング
gui-pin-title = ペアリング PIN
gui-pin-body = もう一方のコンピューターでこの PIN を入力してください。
gui-pin-remaining = 残り { $seconds } 秒有効
gui-pin-expired = PIN の有効期限が切れました。
gui-pin-new = 新しい PIN
gui-pin-stop = ペアリングを停止
gui-close = 閉じる
gui-cancel = キャンセル
gui-network-title = ネットワーク上のコンピューター
gui-network-subtitle = ローカルネットワーク上で見つかった USB Nexus のコンピューターです。
gui-refresh = 更新
gui-searching = ネットワークを検索しています…
gui-none-found = コンピューターが見つかりませんでした。もう一方のコンピューターで USB Nexus が実行されていることを確認するか、アドレスで追加してください。
gui-add-by-address = アドレスで追加
gui-address = アドレス
gui-address-hint = 例：192.168.1.20
gui-pair = ペアリング
gui-paired = ペアリング済み
gui-pair-title = { $name } とペアリング
gui-pair-body = { $name } 側で「新しいコンピューターをペアリング」を選び、表示された PIN を入力してください。
gui-pin = PIN
gui-pair-done = { $name } とペアリングしました。
gui-devices-of = { $name } が共有しているデバイス
gui-no-remote-devices = このコンピューターはデバイスを共有していません。
gui-connect = 接続
gui-disconnect = 切断
gui-in-use-elsewhere = 別のコンピューターが使用中
gui-connected-here = このコンピューターに接続中
gui-back = 戻る
gui-connected-title = このコンピューターに接続中のデバイス
gui-connected-subtitle = リモートデバイスは接続を維持し、ネットワークが切れても自動的に再接続します。
gui-connected-empty = 接続中のリモートデバイスはありません。デバイスを接続するには「ネットワーク」を開いてください。
gui-on-computer = { $name } 上
gui-state-connecting = 接続しています…
gui-state-attached = 接続済み
gui-state-retrying = { $seconds } 秒後に再接続
gui-state-stopped = 切断済み
gui-state-failed = 失敗
gui-reconnect = 再接続
gui-remove = 削除
gui-paired-title = ペアリング済みコンピューター
gui-paired-subtitle = コンピューターは一度 PIN でペアリングすれば、以後は自動的に認識されます。
gui-paired-servers = デバイスを使用できるコンピューター
gui-paired-clients = このコンピューターのデバイスを使用できるコンピューター
gui-paired-empty = まだありません。
gui-remove-confirm = { $name } を削除しますか？ 再度使用するにはペアリングし直す必要があります。
gui-fingerprint = フィンガープリント
gui-service-denied-title = このユーザーは USB Nexus サービスを操作できません
gui-service-denied-body = 操作できるのは管理者と usbnexus グループのメンバーのみです。このユーザーを許可する（管理者パスワードの入力が必要です）か、管理者として次を実行してください：
gui-service-denied-command = コマンド：
gui-service-denied-user = ユーザー
gui-grant-access = このユーザーを許可
gui-grant-access-done = アクセスを許可しました。
gui-setup-kernel-modules = USB over IP に必要なカーネルモジュールが見つかりません：{ $modules }。インストールしてください。サービスが自動的に読み込みます：
gui-setup-kernel-modules-nocmd = ほとんどのディストリビューションではカーネルパッケージに含まれています（Ubuntu：linux-modules-extra、Fedora：kernel-modules-extra）。
gui-service-down-title = USB Nexus サービスが実行されていません
gui-service-down-body = サービスを開始してください。このウィンドウは自動的に接続します。
gui-service-down-linux = Linux では次を実行してください：
gui-service-down-windows = Windows では管理者として次を実行してください：
gui-service-down-macos = macOS では次を実行してください：
gui-retry = 再試行
gui-details = 詳細
err-service-unavailable = USB Nexus サービスに接続できませんでした。

## Web interface

gui-web-login-title = { $name } にサインイン
gui-web-password = パスワード
gui-web-sign-in = サインイン
gui-web-sign-out = サインアウト
gui-web-wrong-password = パスワードが正しくありません。
gui-web-locked = 試行回数が多すぎます。{ $seconds } 秒後に再試行してください。
cmd-web-about = Web インターフェースをオンまたはオフにします。
cmd-enable-about = Web インターフェースをオンにします（初回はパスワードの入力を求められます）。
cmd-disable-about = Web インターフェースをオフにします。
cmd-password-about = Web インターフェースのパスワードを変更します。
cmd-status-about = Web インターフェースの状態とアドレスを表示します。
arg-lan = ネットワーク上の他のコンピューターからのアクセスを許可します。
arg-port = Web インターフェースの TCP ポートです。
web-on = Web インターフェースはオンです：
web-off = Web インターフェースはオフです。
web-local-only = このコンピューターからのみ開けます。他のコンピューターから開くには --lan を使用してください。
web-fingerprint = ブラウザーは証明書の警告を表示します。フィンガープリントは次の値と一致するはずです：{ $fp }
web-trusted = このコンピューターのブラウザーは証明書を信頼します。他のコンピューターでは警告が表示されます。フィンガープリント：{ $fp }
web-not-running = Web インターフェースは有効ですが開始できませんでした：{ $detail }
web-password-prompt = 新しい Web インターフェースのパスワード：
web-password-repeat = パスワードを再入力してください：
web-password-mismatch = パスワードが一致しません。
web-password-set = Web インターフェースのパスワードを変更しました。
err-weak-password = パスワードは 8 文字以上にしてください。
err-port-in-use = このポートは別のプログラムが使用しています。別のポートを選択してください。
err-password-required = 先に Web インターフェースのパスワードを設定してください：usbnexus web password
err-forbidden = これはコンピューター自身からのみ変更できます。
err-not-logged-in = もう一度サインインしてください。

## Hotplug, access control and usage log

arg-device = デバイス：識別子または bus id（参照：usbnexus list SERVER）。
cmd-policy-about = 共有デバイスを使用できる相手を表示または選択します。
arg-policy = open：ペアリング済みのすべてのコンピューターがすべての共有デバイスを使用できます。restricted：デバイスごとに許可されたコンピューターのみ使用できます。
arg-no-server = このコンピューターの USB デバイスの共有を設定しません。
arg-no-client = 他のコンピューターの USB デバイスの使用を設定しません。
arg-web = Web インターフェース：off（オフ）、local（このコンピューターのみ）、network（ネットワーク全体）。
arg-web-port = Web インターフェースのポート（既定 3242）です。
arg-web-password-file = 新しい Web インターフェースのパスワードを含むファイルです。
cmd-history-about = このコンピューターの共有デバイスの使用ログを表示します。
arg-csv = すべての項目を CSV として出力します（例：ファイルに保存するため）。
arg-limit = 表示する項目数です。
cmd-log-about = サービスのログの詳しさを表示または変更します。
arg-level = info（既定）、debug、trace のいずれか。すぐに反映されます。info より上は問題調査用です。
col-device-id = デバイス ID
col-time = 時刻（UTC）
col-event = イベント
col-computer = コンピューター
col-duration = 使用時間
state-unplugged = 未接続
state-no-permission = 権限なし
serve-denied = 「{ $client }」には { $busid } を使用する権限がありません。
attach-waiting-device = デバイスがサーバーに接続されていません。接続を待っています…
attach-queued = 別のコンピューターがデバイスを使用しています。このコンピューターは待機列の { $position } 番目です。
policy-open = ペアリング済みのすべてのコンピューターがすべての共有デバイスを使用できます（オープン）。
policy-restricted = ペアリング済みコンピューターは許可されたデバイスのみ使用できます（制限あり）。
log-level = ログレベル: { $level }
history-header = 使用ログ（項目は { $days } 日間保存されます）：
history-empty = 使用ログは空です。
history-paired = ペアリング
history-pairing-failed = PIN が正しくありません
history-attached = 使用開始
history-detached = 使用終了
history-denied = 拒否（権限なし）
err-access-denied = このコンピューターにはこのデバイスを使用する権限がありません。

gui-nav-history = 履歴
gui-nav-settings = 設定
gui-not-plugged-in = 未接続
gui-tracked-by-port = ポートで識別
gui-tracked-by-port-hint = このデバイスにはシリアル番号がないため、接続されている USB ポートで識別されます。同じポートに接続し直してください。
gui-no-permission = 権限なし
gui-no-permission-hint = そのコンピューターの所有者は、このコンピューターにこのデバイスの使用を許可していません。
gui-connect-when-plugged-in = デバイスが接続されると自動的に接続します。
gui-state-waiting-device = デバイスを待っています
gui-state-queued = 他のコンピューターが使用中：待機列の { $position } 番目
gui-save = 保存
gui-saved = 保存しました。
gui-skip = スキップ
gui-access-title = このデバイスを使用できる相手
gui-access-everyone = ペアリング済みのすべてのコンピューター
gui-access-some = 選択したコンピューター（{ $count }）
gui-access-nobody = まだコンピューターがありません
gui-access-mode-default = 既定の設定に従う
gui-access-default-open = 現在の設定：ペアリング済みのすべてのコンピューター。
gui-access-default-restricted = 現在の設定：以下で選択したコンピューターのみ。
gui-access-mode-open = ペアリング済みのすべてのコンピューター
gui-access-mode-open-body = このコンピューターとペアリング済みのすべてのコンピューターが使用できます。
gui-access-mode-selected = 選択したコンピューターのみ
gui-access-mode-selected-body = 以下でチェックしたコンピューターのみ使用できます。
gui-access-computers = 使用を許可するコンピューター
gui-access-revoke-note = 権限を失ったコンピューターは、そのデバイスから即座に切断されます。
gui-client-devices = デバイス
gui-client-devices-title = { $name } が使用できるデバイス
gui-client-devices-body = このコンピューターが使用できる共有デバイスにチェックを入れてください。
gui-client-devices-after-pairing = 共有デバイスは許可されたコンピューターのみ使用できます。このコンピューターが使用できるデバイスにチェックを入れてください。ここでスキップすると、今のところどのデバイスも使用できません。
gui-no-shared-devices = このコンピューターはまだデバイスを共有していません。
gui-roles-title = このコンピューターの使用方法
gui-roles-body = 選択されていない用途の画面は非表示になります。用途を追加すると、必要なものが自動的にインストールされます。
gui-role-server = サーバーとして使用（このコンピューターの USB デバイスを共有）
gui-role-client = クライアントとして使用（他のコンピューターの USB デバイスを使用）
gui-roles-client-note = 必要に応じて usbip-win2 ドライバーがインストールされます。USB デバイスが数秒間停止し、Windows の再起動が必要になる場合があります。
gui-roles-applying = 適用しています…
gui-reboot-required = セットアップを完了するには、コンピューターを再起動してください。
gui-used-by-waiting = { $name } が使用中・{ $count } 台が待機中
gui-col-permissions = 権限
gui-col-status = 状態
gui-in-use-title = 使用中
gui-nobody-using = 現在このデバイスを使用しているコンピューターはありません。
gui-since = { $time } から
gui-disconnect-user = 切断
gui-disconnected-note = { $name } を切断しました。デバイスを要求し続ける場合は数秒以内に再接続します。完全に切断したままにするには、左側の一覧でチェックを外して保存してください。
gui-queue-title = 待機列（{ $count } 台）
gui-queue-empty = 待機しているコンピューターはありません。
gui-badge-using = 使用中
gui-badge-queued = 待機列（{ $position } 番目）
gui-handover-title = 自動引き継ぎ
gui-handover-default-on = 既定（オン、{ $seconds } 秒）
gui-handover-default-off = 既定（オフ）
gui-handover-on = オン
gui-handover-off = オフ
gui-handover-before = 他のコンピューターが待機している間、
gui-handover-after = 秒間使用がなければ次のコンピューターに引き継がれます。
gui-kind-storage = ストレージ
gui-kind-input = キーボード / マウス
gui-kind-printer = プリンター
gui-kind-dongle = ライセンスドングル
gui-kind-other = その他のデバイス
gui-web-title = Web インターフェース
gui-web-body = パスワードを使って、ブラウザーからこのコンピューターを管理します。
gui-web-confirm-off = Web インターフェースをオフにしますか？ このページは動作を停止します。デスクトップアプリから、またはコンピューター自身で「usbnexus web enable」を実行すると再度オンにできます。
gui-web-confirm-local = アクセスをこのコンピューターのみに限定しますか？ このページはネットワークから開かれているため、動作を停止します。
gui-web-turned-off = Web インターフェースはオフです。デスクトップアプリから、またはコンピューター自身で「usbnexus web enable」を実行すると再度オンにできます。
gui-web-enabled = Web インターフェースがオンです
gui-web-access-local = このコンピューターのみ
gui-web-access-network = ネットワーク全体
gui-web-port = ポート
gui-web-new-password = 新しいパスワード
gui-web-repeat-password = 新しいパスワード（再入力）
gui-web-password-keep = 8 文字以上。空欄のままにすると現在のパスワードが維持されます。
gui-web-password-required = 8 文字以上にしてください。
gui-web-mismatch = パスワードが一致しません。
gui-web-open-at = 接続先：
gui-web-fingerprint = ブラウザーは証明書の警告を表示します。フィンガープリントは { $fp } です。
gui-web-trusted = このコンピューターのブラウザーは証明書を信頼します。他のコンピューターでは警告が表示されます。フィンガープリントは { $fp } です。
gui-policy-title = 共有デバイスを使用できる相手
gui-policy-body = コンピューターは常に先にペアリングが必要です。これはすべての共有デバイスの既定値で、デバイスごとに変更できます。
gui-policy-first-title = 共有デバイスを使用できる相手は？
gui-policy-first-body = コンピューターは常に先に PIN でペアリングが必要です。ペアリング済みコンピューターに許可する操作を選択してください：
gui-policy-later = これは後で設定から変更できます。
gui-policy-open = ペアリング済みのすべてのコンピューター
gui-policy-open-body = ペアリング済みのすべてのコンピューターがすべての共有デバイスを使用できます。
gui-policy-restricted = 許可されたコンピューターのみ
gui-policy-restricted-body = デバイスごとに、使用を許可するペアリング済みコンピューターを選択します。
gui-retention-title = 使用ログ
gui-retention-body = ペアリング、誤った PIN の入力、デバイスの使用、拒否されたリクエストが記録されます。古い項目は自動的に削除されます。
gui-retention-days = 項目の保存期間（日数）
gui-settings-title = 設定
gui-startup-title = スタートアップ
gui-startup-body = ウィンドウを閉じても USB Nexus は通知領域に残ります。共有と接続はサービスによって継続されます。
gui-startup-enabled = サインイン時に自動的に起動する
gui-tray-open = USB Nexus を開く
gui-tray-quit = 終了
gui-history-title = 履歴
gui-history-subtitle = このコンピューターのデバイスを誰がいつ使用したかを表示します。項目は { $days } 日間保存されます。
gui-history-empty = まだ何も記録されていません。
gui-history-export = CSV として書き出す
gui-history-time = 時刻
gui-history-event = イベント
gui-history-computer = コンピューター
gui-history-device = デバイス
gui-history-device-id = デバイス ID
gui-history-duration = 使用時間
gui-history-paired = ペアリング
gui-history-pairing-failed = PIN が正しくありません
gui-history-attached = 使用開始
gui-history-detached = 使用終了
gui-history-denied = 拒否：権限なし

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = USB Nexus をどのように使用しますか？
setup-roles-subtitle = このコンピューターの用途を選択してください。
setup-role-server = サーバーとして使用する
setup-role-client = クライアントとして使用する
setup-role-web = Web アクセスを使用する
setup-usbip-install-note = usbip-win2 ドライバーも同時にインストールされます。インストール中は USB デバイスが数秒間停止し、その後 Windows の再起動が必要です。
setup-usbip-update-note = インストール済みの usbip-win2 ドライバーが古すぎるため更新されます。インストール中は USB デバイスが数秒間停止し、その後 Windows の再起動が必要です。
setup-usbip-present-note = usbip-win2 ドライバーはこのコンピューターに既にインストールされています。
setup-usbip-failed = usbip-win2 ドライバーをインストールできませんでした。後で USB Nexus のセットアップを再実行すると再試行できます。
setup-service-failed = USB Nexus サービスを設定できませんでした。詳細はインストールログを確認してください。
setup-web-title = Web インターフェース
setup-web-subtitle = ブラウザーからこのコンピューターを管理するための設定です。
setup-web-access = アクセス：
setup-web-local = このコンピューターのみ
setup-web-network = ネットワーク全体
setup-web-port = ポート：
setup-web-port-free = ✓ このポートは使用可能です
setup-web-port-busy = ✗ このポートは他のプログラムが使用しています
setup-web-port-invalid = ✗ 1 から 65535 までの数値を入力してください
setup-web-password = パスワード：
setup-web-password-repeat = パスワード（再入力）：
setup-web-password-hint = 8 文字以上。
setup-web-password-keep = 8 文字以上。空欄のままにすると現在のパスワードが維持されます。
