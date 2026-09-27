# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Azerbaijani.
#
# Every message here must also exist in every other locale file;
# `cargo test -p usbnexus-i18n` enforces this.

## Command line help

app-about = Təhlükəsiz USB over IP: USB cihazlarını şəbəkə üzərindən şifrələmə, cütləmə və avtomatik yenidən qoşulma ilə paylaşın.
arg-lang = İnterfeys dili (məs., en, tr). Defolt olaraq sistem dili istifadə olunur.
arg-state-dir = Açarların və cütlənmiş kompüterlərin saxlandığı qovluq.
arg-name = Bu kompüterin digərlərinə göstərilən adı.
arg-verbose = Ətraflı jurnal mesajlarını göstərir.

cmd-daemon-about = USB Nexus xidmətini işə salır (masaüstü tətbiqi bundan istifadə edir).
arg-allow-all-users = Xidməti bu kompüterdəki bütün istifadəçilər idarə edə bilsin (defolt: usbnexus qrupunun üzvləri).
arg-socket = Xidmətin yerli idarəetmə soketinin yolu.
cmd-service-about = USB Nexus Windows xidmətini quraşdırır və ya silir.
cmd-install-about = Xidməti quraşdırır, indi və hər açılışda başladır (administrator kimi işə salın).
cmd-uninstall-about = Xidməti dayandırır və silir.
cmd-serve-about = Bu kompüterin USB cihazlarını paylaşır.
arg-export = Paylaşılacaq cihazın bus ID-si (bir neçə dəfə göstərilə bilər). Bax: usbnexus local
arg-listen = Dinləniləcək ünvan və port.
arg-pair = Başlanğıcda cütləmə pəncərəsini açır və PIN-i göstərir.
arg-no-mdns = Bu serveri yerli şəbəkədə elan etmə.

cmd-pin-about = İşləyən serverlə yeni kompüteri cütləmək üçün PIN göstərir.
arg-seconds = PIN-in etibarlı olacağı müddət (saniyə).

cmd-local-about = Bu kompüterə qoşulmuş USB cihazlarını siyahılayır.
cmd-discover-about = Yerli şəbəkədəki USB Nexus serverlərini axtarır.
arg-timeout = Axtarış müddəti (saniyə).
cmd-pair-about = Serverin göstərdiyi PIN ilə cütləyir.
arg-server = Server: cütlənmiş ad, barmaq izi və ya host[:port].
arg-pin = Serverin göstərdiyi PIN (verilməzsə soruşulur).
cmd-list-about = Bir serverin paylaşdığı cihazları siyahılayır.
cmd-allow-user-about = İstifadəçiyə masaüstü tətbiqindən xidməti idarə etməyə icazə verir (usbnexus qrupuna əlavə edir).
arg-user = İstifadəçi adı.
allow-user-invalid = “{ $user }” düzgün istifadəçi adı deyil.
allow-user-failed = “{ $user }” usbnexus qrupuna əlavə edilə bilmədi.
allow-user-done = “{ $user }” indi USB Nexus xidmətini idarə edə bilər.
allow-user-relogin = Yenidən daxil olduqda qüvvəyə minir.
cmd-attach-about = Uzaq cihazı bu kompüterdə istifadə edir; avtomatik yenidən qoşulur.
arg-busid = Uzaq cihazın bus ID-si.
cmd-peers-about = Cütlənmiş kompüterləri siyahılayır.
cmd-forget-about = Cütlənmiş kompüteri silir.
arg-peer = Cütlənmiş kompüterin adı və ya barmaq izi.

## General

error-prefix = Xəta: { $detail }
hint-root = Bu əməliyyat administrator hüquqları tələb edir. sudo ilə yenidən cəhd edin.
unsupported-os = Bu əmr bu əməliyyat sistemində hələ dəstəklənmir.
yes = bəli
no = xeyr

service-installed = USB Nexus xidməti quraşdırıldı və başladıldı.
service-removed = USB Nexus xidməti silindi.

## Server

serve-started = “{ $name }” serveri { $addr } ünvanında dinləyir.
serve-fingerprint = Barmaq izi: { $fp }
serve-exporting = Paylaşılan cihazlar:
serve-no-exports = Heç bir cihaz paylaşılmır. --export BUSID əlavə edin (cihazları görmək üçün: usbnexus local).
serve-pairing-pin = Cütləmə PIN-i: { $pin } ({ $seconds } saniyə etibarlıdır)
serve-paired = “{ $name }” ilə cütləndi.
serve-pairing-failed = { $addr } ünvanından uğursuz cütləmə cəhdi.
serve-exported = { $busid } indi “{ $client }” tərəfindən istifadə olunur.
serve-released = { $busid }, “{ $client }” tərəfindən buraxıldı.
serve-stopping = Dayandırılır; cihazlar öz normal drayverlərinə qaytarılır…
serve-mdns-failed = Yerli şəbəkədə aşkarlama əlçatan deyil: { $detail }
serve-bind-failed = { $busid } paylaşım üçün hazırlana bilmədi: { $detail }

## Pairing

pin-show = Cütləmə PIN-i: { $pin }
pin-hint = Digər kompüterdə { $seconds } saniyə ərzində “usbnexus pair { $name }” əmrini işə salın.
pin-no-server = İşləyən USB Nexus serveri tapılmadı. Başlatmaq üçün: usbnexus serve
pair-enter-pin = Serverdə göstərilən PIN-i daxil edin:
pair-ok = “{ $name }” ilə cütləndi ({ $fp }).
pair-already = “{ $name }” ilə artıq cütlənib.

## Devices

local-header = Bu kompüterdəki USB cihazları:
local-empty = Heç bir USB cihazı tapılmadı.
list-header = “{ $name }” tərəfindən paylaşılan cihazlar:
list-empty = Bu server heç bir cihaz paylaşmır.
list-in-use = istifadədədir
col-busid = BUS ID
col-id = VID:PID
col-speed = SÜRƏT
col-product = MƏHSUL
col-driver = DRAYVER
col-state = VƏZİYYƏT
col-name = AD
col-fingerprint = BARMAQ İZİ
col-address = ÜNVAN

## Discovery and peers

discover-searching = Yerli şəbəkədə axtarılır…
discover-none = Heç bir USB Nexus serveri tapılmadı.
discover-paired = cütlənib
peers-empty = Hələ cütlənmiş kompüter yoxdur.
forget-ok = “{ $name }” silindi.
forget-unknown = “{ $peer }” ilə uyğun gələn kompüter yoxdur.

## Attaching

attach-connecting = { $addr } ünvanına qoşulur…
attach-attached = { $busid } qoşuldu (virtual port { $port }).
attach-disconnected = Bağlantı kəsildi: { $reason }
attach-retrying = { $seconds } saniyə sonra yenidən qoşulacaq…
attach-detached = Cihaz ayrıldı.
attach-stop-hint = Ayırmaq üçün Ctrl+C düymələrini basın.

## Errors

err-pairing-required = “{ $name }” hələ bu kompüterlə cütlənməyib. İşə salın: usbnexus pair { $target }
err-not-found = Server şəbəkədə tapıla bilmədi.
err-version = Server USB Nexus-un uyğun olmayan versiyasını işlədir.
err-not-trusted = Bu kompüter serverlə cütlənməyib.
err-pairing-closed = Serverdə cütləmə açıq deyil. Serverdə işə salın: usbnexus pin
err-pairing-failed = Cütləmə uğursuz oldu. PIN-i yoxlayıb yenidən cəhd edin.
err-no-such-device = Server bu cihazı paylaşmır.
err-device-busy = Cihaz başqa bir kompüter tərəfindən istifadə olunur.
err-internal = Server daxili xəta bildirdi.
err-protocol = Serverdən gözlənilməz cavab alındı.
err-connection-lost = Kompüterlə bağlantı kəsildi.
err-unsupported = Bu, bu əməliyyat sistemində hələ əlçatan deyil.
err-driver-missing = usbip-win2 drayveri quraşdırılmayıb. USB Nexus qurulumunu yenidən işə salıb usbip-win2 quraşdırılmasını təsdiqləyin.
err-driver-outdated = Quraşdırılmış usbip-win2 drayveri çox köhnədir. USB Nexus qurulumunu yenidən işə salıb usbip-win2 yenilənməsini təsdiqləyin.
err-vboxusb-missing = Paylaşım üçün lazım olan VirtualBox USB drayverləri quraşdırılmayıb. USB Nexus-u yenidən quraşdırın.
err-device-in-use-by-os = Əməliyyat sistemi bu cihazı öz drayveri ilə istifadə edir, ona görə bu kompüterdən paylaşıla bilmir.
err-unreachable = Kompüterlə əlaqə saxlanıla bilmədi. Açıq və şəbəkəyə qoşulu olduğunu yoxlayın.
err-cancelled = Əməliyyat ləğv edildi.
err-permission-denied = USB Nexus-un lazımi icazələri yoxdur.
err-invalid = Sorğu başa düşülmədi.
err-other = Nəsə səhv getdi: { $detail }

## Help layout

help-usage = İstifadə qaydası:
help-arguments = Arqumentlər
help-options = Seçimlər
help-commands = Əmrlər
arg-help = Kömək göstərir.
arg-version = Versiyanı göstərir.

## Peers

peers-servers = Cütlənmiş serverlər (bu kompüter onların cihazlarını istifadə edə bilər):
peers-clients = Cütlənmiş müştərilər (bu kompüterin cihazlarını istifadə etməyə icazəli):

## Desktop app

gui-nav-this-computer = Bu kompüter
gui-nav-network = Şəbəkədəki kompüterlər
gui-nav-connected = Qoşulmuş cihazlar
gui-nav-paired = Cütlənmiş kompüterlər
gui-language = Dil
gui-this-title = Bu kompüterdəki cihazlar
gui-this-subtitle = Digər kompüterlərin hansı cihazları istifadə edə biləcəyini seçin.
gui-share = Paylaş
gui-shared = Paylaşılır
gui-not-shared = Paylaşılmır
gui-used-by = { $name } istifadə edir
gui-no-local-devices = Bu kompüterdə USB cihazı tapılmadı.
gui-unnamed-device = USB cihazı
gui-pair-new = Yeni kompüter cütlə
gui-pin-title = Cütləmə PIN-i
gui-pin-body = Bu PIN-i digər kompüterdə daxil edin.
gui-pin-remaining = { $seconds } saniyə daha etibarlıdır
gui-pin-expired = PIN-in müddəti bitdi.
gui-pin-new = Yeni PIN
gui-pin-stop = Cütləməni dayandır
gui-close = Bağla
gui-cancel = Ləğv et
gui-network-title = Şəbəkədəki kompüterlər
gui-network-subtitle = Yerli şəbəkənizdə tapılan USB Nexus kompüterləri.
gui-refresh = Yenilə
gui-searching = Şəbəkədə axtarılır…
gui-none-found = Heç bir kompüter tapılmadı. Digər kompüterdə USB Nexus-un işlədiyinə əmin olun, yaxud ünvanla əlavə edin.
gui-add-by-address = Ünvanla əlavə et
gui-address = Ünvan
gui-address-hint = məs., 192.168.1.20
gui-pair = Cütlə
gui-paired = Cütlənib
gui-pair-title = { $name } ilə cütlə
gui-pair-body = { $name } üzərində “Yeni kompüter cütlə” seçin və orada göstərilən PIN-i yazın.
gui-pin = PIN
gui-pair-done = { $name } ilə cütləndi.
gui-devices-of = { $name } tərəfindən paylaşılan cihazlar
gui-no-remote-devices = Bu kompüter heç bir cihaz paylaşmır.
gui-connect = Qoşul
gui-disconnect = Bağlantını kəs
gui-in-use-elsewhere = Başqa kompüter istifadə edir
gui-connected-here = Bu kompüterə qoşulub
gui-back = Geri
gui-connected-title = Bu kompüterə qoşulmuş cihazlar
gui-connected-subtitle = Uzaq cihazlar qoşulu qalır; şəbəkə kəsilsə özü yenidən qoşulur.
gui-connected-empty = Heç bir uzaq cihaz qoşulmayıb. Bir cihaz qoşmaq üçün “Şəbəkə” bölməsini açın.
gui-on-computer = { $name } üzərində
gui-state-connecting = Qoşulur…
gui-state-attached = Qoşulub
gui-state-retrying = { $seconds } saniyə ərzində yenidən qoşulacaq
gui-state-stopped = Bağlantı kəsilib
gui-state-failed = Uğursuz oldu
gui-reconnect = Yenidən qoşul
gui-remove = Sil
gui-paired-title = Cütlənmiş kompüterlər
gui-paired-subtitle = Kompüterlər bir dəfə PIN ilə cütlənir, sonra avtomatik tanınır.
gui-paired-servers = Cihazlarını istifadə edə biləcəyiniz kompüterlər
gui-paired-clients = Sizin cihazlarınızı istifadə edə bilən kompüterlər
gui-paired-empty = Hələ yoxdur.
gui-remove-confirm = { $name } silinsin? Onu yenidən istifadə etmək üçün təkrar cütləməli olacaqsınız.
gui-fingerprint = Barmaq izi
gui-service-denied-title = Bu istifadəçi USB Nexus xidmətini idarə edə bilməz
gui-service-denied-body = Bunu yalnız administratorlar və usbnexus qrupunun üzvləri edə bilər. Bu istifadəçiyə icazə verin (administrator parolu soruşulacaq), yaxud administrator kimi işə salın:
gui-service-denied-command = Əmr:
gui-service-denied-user = İSTİFADƏÇİ
gui-grant-access = Bu istifadəçiyə icazə ver
gui-grant-access-done = İcazə verildi.
gui-setup-kernel-modules = USB over IP üçün lazım olan nüvə modulları çatışmır: { $modules }. Onları quraşdırın; xidmət özü yükləyəcək:
gui-setup-kernel-modules-nocmd = Onlar əksər distribusiyaların nüvəsi ilə birlikdə gəlir (Ubuntu: linux-modules-extra, Fedora: kernel-modules-extra).
gui-service-down-title = USB Nexus xidməti işləmir
gui-service-down-body = Xidməti başladın; bu pəncərə ona avtomatik qoşulacaq.
gui-service-down-linux = Linux-da işə salın:
gui-service-down-windows = Windows-da administrator kimi işə salın:
gui-service-down-macos = macOS-da işə salın:
gui-retry = Yenidən cəhd et
gui-details = Ətraflı
err-service-unavailable = USB Nexus xidməti ilə əlaqə saxlanıla bilmədi.

## Web interface

gui-web-login-title = { $name }-a daxil olun
gui-web-password = Parol
gui-web-sign-in = Daxil ol
gui-web-sign-out = Çıxış et
gui-web-wrong-password = Parol yanlışdır.
gui-web-locked = Həddindən çox cəhd. { $seconds } saniyə sonra yenidən cəhd edin.
cmd-web-about = Veb interfeysi aç və ya bağla.
cmd-enable-about = Veb interfeysi aç (ilk dəfə parol soruşulur).
cmd-disable-about = Veb interfeysi bağla.
cmd-password-about = Veb interfeysinin parolunu dəyişdir.
cmd-status-about = Veb interfeysinin açıq olub-olmadığını və ünvanını göstər.
arg-lan = Şəbəkədəki digər kompüterlərdən girişə icazə ver.
arg-port = Veb interfeysinin TCP portu.
web-on = Veb interfeysi açıqdır:
web-off = Veb interfeysi bağlıdır.
web-local-only = Onu yalnız bu kompüter aça bilər. Digər kompüterlər üçün --lan istifadə edin.
web-fingerprint = Brauzer sertifikat xəbərdarlığı verəcək; sertifikatın barmaq izi bu olmalıdır: { $fp }
web-trusted = Bu kompüterdəki brauzerlər sertifikata etibar edir; digər kompüterlər xəbərdarlıq verəcək. Sertifikatın barmaq izi: { $fp }
web-not-running = Veb interfeysi aktivdir, amma başladıla bilmədi: { $detail }
web-password-prompt = Yeni veb interfeys parolu:
web-password-repeat = Parolu təkrar daxil edin:
web-password-mismatch = Parollar uyğun gəlmir.
web-password-set = Veb interfeysinin parolu dəyişdirildi.
err-weak-password = Parol ən azı 8 simvoldan ibarət olmalıdır.
err-port-in-use = Bu port başqa proqram tərəfindən istifadə olunur. Başqa port seçin.
err-password-required = Əvvəlcə veb interfeys parolu təyin edin: usbnexus web password
err-forbidden = Bu yalnız kompüterin özündən dəyişdirilə bilər.
err-not-logged-in = Zəhmət olmasa yenidən daxil olun.

## Hotplug, access control and usage log

arg-device = Cihaz: identifikator və ya bus ID (bax: usbnexus list SERVER).
cmd-policy-about = Paylaşılan cihazları kimin istifadə edə biləcəyini göstər və ya seç.
arg-policy = open: cütlənmiş hər kompüter paylaşılan hər cihazı istifadə edə bilər; restricted: yalnız cihaz üzrə icazəli kompüterlər.
arg-no-server = Bu kompüterin USB cihazlarını paylaşmağı quraşdırma.
arg-no-client = Digər kompüterlərin USB cihazlarını istifadə etməyi quraşdırma.
arg-web = Veb interfeys: off (bağlı), local (yalnız bu kompüter) və ya network (bütün şəbəkə).
arg-web-port = Veb interfeysinin portu (defolt 3242).
arg-web-password-file = Yeni veb interfeys parolunu ehtiva edən fayl.
cmd-history-about = Bu kompüterin paylaşılan cihazlarının istifadə jurnalını göstər.
arg-csv = Bütün qeydləri CSV formatında çap et (məs., fayla saxlamaq üçün).
arg-limit = Göstəriləcək qeyd sayı.
cmd-log-about = Xidmətin nə qədər ətraflı jurnal tutduğunu göstər və ya dəyişdir.
arg-level = info (defolt), debug və ya trace. Dərhal qüvvəyə minir; info-dan yuxarısı problem axtararkən istifadə olunur.
col-device-id = CİHAZ ID
col-time = VAXT (UTC)
col-event = HADİSƏ
col-computer = KOMPÜTER
col-duration = MÜDDƏT
state-unplugged = qoşulmayıb
state-no-permission = icazə yoxdur
serve-denied = “{ $client }”-in { $busid } cihazını istifadə etməyə icazəsi yoxdur.
attach-waiting-device = Cihaz serverə qoşulmayıb; qoşulması gözlənilir…
attach-queued = Cihazı başqa kompüter istifadə edir; bu kompüter növbədə { $position }-cidir.
policy-open = Cütlənmiş hər kompüter paylaşılan hər cihazı istifadə edə bilər (açıq).
policy-restricted = Cütlənmiş kompüterlər yalnız istifadə etməyə icazəli olduqları cihazları istifadə edə bilər (məhdud).
log-level = Jurnal səviyyəsi: { $level }
history-header = İstifadə jurnalı (qeydlər { $days } gün saxlanılır):
history-empty = İstifadə jurnalı boşdur.
history-paired = cütləndi
history-pairing-failed = yanlış PIN
history-attached = istifadəyə başladı
history-detached = istifadəni dayandırdı
history-denied = rədd edildi (icazə yoxdur)
err-access-denied = Bu kompüterin cihazı istifadə etməyə icazəsi yoxdur.

gui-nav-history = Tarixçə
gui-nav-settings = Ayarlar
gui-not-plugged-in = Qoşulmayıb
gui-tracked-by-port = port üzrə izlənir
gui-tracked-by-port-hint = Bu cihazın seriya nömrəsi yoxdur, buna görə qoşulduğu USB portundan tanınır. Onu yenidən eyni porta qoşun.
gui-no-permission = İcazə yoxdur
gui-no-permission-hint = Həmin kompüterin sahibi bu kompüterə cihazı istifadə etməyə icazə verməyib.
gui-connect-when-plugged-in = Cihaz qoşulan kimi avtomatik qoşulur.
gui-state-waiting-device = Cihaz gözlənilir
gui-state-queued = Başqa yerdə istifadə olunur; növbədə { $position }-cidir
gui-save = Saxla
gui-saved = Saxlanıldı.
gui-skip = Keç
gui-access-title = Bu cihazı kim istifadə edə bilər
gui-access-everyone = Bütün cütlənmiş kompüterlər
gui-access-some = Seçilmiş kompüterlər ({ $count })
gui-access-nobody = Hələ heç bir kompüter
gui-access-mode-default = Defolt ayarı izlə
gui-access-default-open = Hazırda: bütün cütlənmiş kompüterlər.
gui-access-default-restricted = Hazırda: yalnız aşağıda seçilmiş kompüterlər.
gui-access-mode-open = Bütün cütlənmiş kompüterlər
gui-access-mode-open-body = Bununla cütlənmiş hər kompüter bunu istifadə edə bilər.
gui-access-mode-selected = Yalnız seçilmiş kompüterlər
gui-access-mode-selected-body = Yalnız aşağıda işarələnmiş kompüterlər bunu istifadə edə bilər.
gui-access-computers = İstifadəyə icazəli kompüterlər
gui-access-revoke-note = İcazəsi ləğv edilən kompüter cihazdan dərhal ayrılır.
gui-client-devices = Cihazlar
gui-client-devices-title = { $name }-in istifadə edə biləcəyi cihazlar
gui-client-devices-body = Bu kompüterin istifadə edə biləcəyi paylaşılan cihazları işarələyin.
gui-client-devices-after-pairing = Paylaşılan cihazları yalnız icazəli kompüterlər istifadə edə bilər. Bu kompüterin istifadə edə biləcəyi cihazları işarələyin; bunu keçsəniz, hələlik heç birini istifadə edə bilməz.
gui-no-shared-devices = Bu kompüter hələ heç bir cihaz paylaşmır.
gui-roles-title = Bu kompüter necə istifadə olunur
gui-roles-body = Seçilməyən istifadə üsulunun ekranları gizlədilir. Əlavə edilən üsul üçün lazım olanlar quraşdırılır.
gui-role-server = Server kimi istifadə et (bu kompüterin USB cihazlarını paylaş)
gui-role-client = Müştəri kimi istifadə et (digər kompüterlərin USB cihazlarını istifadə et)
gui-roles-client-note = Lazım gələrsə usbip-win2 drayveri quraşdırılır; USB cihazları bir neçə saniyə dayanır və Windows-un yenidən başladılması tələb oluna bilər.
gui-roles-applying = Tətbiq edilir…
gui-reboot-required = Qurulumu tamamlamaq üçün kompüteri yenidən başladın.
gui-used-by-waiting = { $name } istifadə edir · { $count } kompüter gözləyir
gui-col-permissions = İcazələr
gui-col-status = Vəziyyət
gui-in-use-title = İstifadə edən
gui-nobody-using = Hazırda cihazı heç kim istifadə etmir.
gui-since = { $time }-dən bəri
gui-disconnect-user = Bağlantını kəs
gui-disconnected-note = { $name }-in bağlantısı kəsildi. Cihazı istəməyə davam edərsə bir neçə saniyə ərzində yenidən qoşulacaq; onu həmişəlik uzaq tutmaq üçün soldakı siyahıda işarəsini götürüb saxlayın.
gui-queue-title = Növbədə gözləyənlər ({ $count })
gui-queue-empty = Gözləyən yoxdur.
gui-badge-using = istifadə edir
gui-badge-queued = növbədə ({ $position })
gui-handover-title = Avtomatik təhvil
gui-handover-default-on = Defolt (açıq, { $seconds } san)
gui-handover-default-off = Defolt (bağlı)
gui-handover-on = Açıq
gui-handover-off = Bağlı
gui-handover-before = Başqa kompüter gözlədiyi zaman,
gui-handover-after = saniyə istifadə olunmadıqda növbədəkinə keçir.
gui-kind-storage = Yaddaş
gui-kind-input = Klaviatura / siçan
gui-kind-printer = Printer
gui-kind-dongle = Lisenziya donqulu
gui-kind-other = Digər cihaz
gui-web-title = Veb interfeysi
gui-web-body = Bu kompüteri brauzerdən, parolla idarə edin.
gui-web-confirm-off = Veb interfeysi bağlansın? Bu səhifə işləməyi dayandıracaq. Onu masaüstü tətbiqindən, yaxud kompüterin özündə “usbnexus web enable” əmri ilə yenidən aça bilərsiniz.
gui-web-confirm-local = Giriş yalnız bu kompüterdən olsun? Bu səhifə şəbəkədən açılıb və işləməyi dayandıracaq.
gui-web-turned-off = Veb interfeysi bağlıdır. Onu masaüstü tətbiqindən, yaxud kompüterin özündə “usbnexus web enable” əmri ilə yenidən aça bilərsiniz.
gui-web-enabled = Veb interfeysi açıqdır
gui-web-access-local = Yalnız bu kompüter
gui-web-access-network = Bütün şəbəkə
gui-web-port = Port
gui-web-new-password = Yeni parol
gui-web-repeat-password = Yeni parol (təkrar)
gui-web-password-keep = Ən azı 8 simvol. Cari parolu saxlamaq üçün boş qoyun.
gui-web-password-required = Ən azı 8 simvol.
gui-web-mismatch = Parollar uyğun gəlmir.
gui-web-open-at = Ünvan:
gui-web-fingerprint = Brauzer sertifikat xəbərdarlığı verir; sertifikatın barmaq izi { $fp }.
gui-web-trusted = Bu kompüterdəki brauzerlər sertifikata etibar edir; digər kompüterlər xəbərdarlıq verir. Sertifikatın barmaq izi { $fp }.
gui-policy-title = Paylaşılan cihazları kim istifadə edə bilər
gui-policy-body = Kompüterlər həmişə əvvəlcə cütlənməlidir. Bu, hər paylaşılan cihaz üçün defolt ayardır; hər cihaz üçün ayrıca dəyişdirilə bilər.
gui-policy-first-title = Paylaşılan cihazlarınızı kim istifadə edə bilsin?
gui-policy-first-body = Kompüterlər həmişə əvvəlcə PIN ilə cütlənməlidir. Cütlənmiş kompüterlərin nə edə biləcəyini seçin:
gui-policy-later = Bunu sonra Ayarlar bölməsindən dəyişə bilərsiniz.
gui-policy-open = Cütlənmiş hər kompüter
gui-policy-open-body = Cütlənmiş hər kompüter paylaşılan hər cihazı istifadə edə bilər.
gui-policy-restricted = Yalnız icazəli kompüterlər
gui-policy-restricted-body = Hər cihaz üçün hansı cütlənmiş kompüterlərin istifadə edə biləcəyini siz seçirsiniz.
gui-retention-title = İstifadə jurnalı
gui-retention-body = Cütləmələr, yanlış PIN-lər, cihaz istifadəsi və rədd edilən sorğular qeydə alınır. Köhnə qeydlər avtomatik silinir.
gui-retention-days = Qeydlərin saxlanılma müddəti (gün)
gui-settings-title = Ayarlar
gui-startup-title = Başlanğıc
gui-startup-body = Pəncərəni bağlamaq USB Nexus-u bildiriş sahəsində saxlayır; xidmət paylaşımı və bağlantıları hər halda davam etdirir.
gui-startup-enabled = Daxil olduqda avtomatik başlat
gui-tray-open = USB Nexus-u aç
gui-tray-quit = Çıxış
gui-history-title = Tarixçə
gui-history-subtitle = Bu kompüterin cihazlarını kim, nə vaxt istifadə edib. Qeydlər { $days } gün saxlanılır.
gui-history-empty = Hələ heç nə qeydə alınmayıb.
gui-history-export = CSV kimi ixrac et
gui-history-time = Vaxt
gui-history-event = Hadisə
gui-history-computer = Kompüter
gui-history-device = Cihaz
gui-history-device-id = Cihaz ID
gui-history-duration = Müddət
gui-history-paired = Cütləndi
gui-history-pairing-failed = Yanlış PIN
gui-history-attached = İstifadəyə başladı
gui-history-detached = İstifadəni dayandırdı
gui-history-denied = Rədd edildi: icazə yoxdur

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = USB Nexus-u necə istifadə edəcəksiniz?
setup-roles-subtitle = Bu kompüterin nə edəcəyini seçin.
setup-role-server = Server kimi istifadə et
setup-role-client = Müştəri kimi istifadə et
setup-role-web = Veb girişi
setup-usbip-install-note = usbip-win2 drayveri də quraşdırılacaq. Quraşdırma zamanı USB cihazları bir neçə saniyə dayanacaq və sonra Windows-un yenidən başladılması lazımdır.
setup-usbip-update-note = Quraşdırılmış usbip-win2 drayveri çox köhnədir və yenilənəcək. Quraşdırma zamanı USB cihazları bir neçə saniyə dayanacaq və sonra Windows-un yenidən başladılması lazımdır.
setup-usbip-present-note = usbip-win2 drayveri bu kompüterdə artıq quraşdırılıb.
setup-usbip-failed = usbip-win2 drayveri quraşdırıla bilmədi. USB Nexus qurulumunu sonra yenidən işə salaraq təkrar cəhd edə bilərsiniz.
setup-service-failed = USB Nexus xidməti quraşdırıla bilmədi. Ətraflı məlumat qurulum jurnalındadır.
setup-web-title = Veb interfeysi
setup-web-subtitle = Bu kompüteri brauzerdən idarə etmək üçün ayarlar.
setup-web-access = Giriş:
setup-web-local = Yalnız bu kompüter
setup-web-network = Bütün şəbəkə
setup-web-port = Port:
setup-web-port-free = ✓ Port əlçatandır
setup-web-port-busy = ✗ Bu port başqa proqram tərəfindən istifadə olunur
setup-web-port-invalid = ✗ 1 ilə 65535 arasında bir ədəd daxil edin
setup-web-password = Parol:
setup-web-password-repeat = Parol (təkrar):
setup-web-password-hint = Ən azı 8 simvol.
setup-web-password-keep = Ən azı 8 simvol. Cari parolu saxlamaq üçün boş qoyun.
