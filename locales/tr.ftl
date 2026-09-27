# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Turkish.

## Command line help

app-about = Güvenli USB-over-IP: USB cihazlarını ağ üzerinden şifreli, eşleştirmeli ve otomatik yeniden bağlanarak paylaşın.
arg-lang = Arayüz dili (ör. en, tr). Varsayılan: sistem dili.
arg-state-dir = Anahtarların ve eşleştirilmiş bilgisayarların tutulduğu klasör.
arg-name = Bu bilgisayarın diğerlerine görünen adı.
arg-verbose = Ayrıntılı günlük mesajlarını göster.

cmd-daemon-about = USB Nexus hizmetini çalıştır (masaüstü uygulaması bunu kullanır).
arg-allow-all-users = Hizmeti bu bilgisayardaki tüm kullanıcılar yönetebilsin (varsayılan: usbnexus grubunun üyeleri).
arg-socket = Hizmetin yerel denetim soketinin yolu.
cmd-service-about = USB Nexus Windows hizmetini kur veya kaldır.
cmd-install-about = Hizmeti kur; şimdi ve her açılışta başlat (yönetici olarak çalıştırın).
cmd-uninstall-about = Hizmeti durdur ve kaldır.
cmd-serve-about = Bu bilgisayarın USB cihazlarını paylaş.
arg-export = Paylaşılacak cihazın veri yolu kimliği (birden çok kez verilebilir). Bkz: usbnexus local
arg-listen = Dinlenecek adres ve bağlantı noktası.
arg-pair = Açılışta eşleştirmeyi başlat ve PIN kodunu göster.
arg-no-mdns = Bu sunucuyu yerel ağda duyurma.

cmd-pin-about = Çalışan sunucuyla yeni bir bilgisayarı eşleştirmek için PIN göster.
arg-seconds = PIN kodunun geçerli kalacağı süre (saniye).

cmd-local-about = Bu bilgisayara takılı USB cihazlarını listele.
cmd-discover-about = Yerel ağdaki USB Nexus sunucularını bul.
arg-timeout = Arama süresi (saniye).
cmd-pair-about = Sunucunun gösterdiği PIN ile eşleştir.
arg-server = Sunucu: eşleştirilmiş ad, parmak izi veya adres[:port].
arg-pin = Sunucunun gösterdiği PIN (verilmezse sorulur).
cmd-list-about = Bir sunucunun paylaştığı cihazları listele.
cmd-allow-user-about = Bir kullanıcının masaüstü uygulamasından hizmeti yönetmesine izin ver (usbnexus grubuna ekler).
arg-user = Kullanıcı adı.
allow-user-invalid = “{ $user }” geçerli bir kullanıcı adı değil.
allow-user-failed = “{ $user }” usbnexus grubuna eklenemedi.
allow-user-done = “{ $user }” artık USB Nexus hizmetini yönetebilir.
allow-user-relogin = Oturumu yeniden açınca geçerli olur.
cmd-attach-about = Uzaktaki bir cihazı bu bilgisayarda kullan; bağlantı koparsa kendiliğinden yeniden bağlanır.
arg-busid = Uzaktaki cihazın veri yolu kimliği.
cmd-peers-about = Eşleştirilmiş bilgisayarları listele.
cmd-forget-about = Eşleştirilmiş bir bilgisayarı kaldır.
arg-peer = Eşleştirilmiş bilgisayarın adı veya parmak izi.

## General

error-prefix = Hata: { $detail }
hint-root = Bu işlem yönetici yetkisi gerektiriyor. sudo ile yeniden deneyin.
unsupported-os = Bu komut bu işletim sisteminde henüz desteklenmiyor.
yes = evet
no = hayır

service-installed = USB Nexus hizmeti kuruldu ve başlatıldı.
service-removed = USB Nexus hizmeti kaldırıldı.

## Server

serve-started = “{ $name }” sunucusu { $addr } adresinde dinliyor.
serve-fingerprint = Parmak izi: { $fp }
serve-exporting = Paylaşılan cihazlar:
serve-no-exports = Paylaşılan cihaz yok. --export BUSID ekleyin (cihazları görmek için: usbnexus local).
serve-pairing-pin = Eşleştirme PIN kodu: { $pin } ({ $seconds } saniye geçerli)
serve-paired = “{ $name }” ile eşleştirildi.
serve-pairing-failed = { $addr } adresinden başarısız eşleştirme denemesi.
serve-exported = { $busid } artık “{ $client }” tarafından kullanılıyor.
serve-released = { $busid }, “{ $client }” tarafından bırakıldı.
serve-stopping = Durduruluyor; cihazlar normal sürücülerine geri veriliyor…
serve-mdns-failed = Yerel ağda bulunabilirlik kullanılamıyor: { $detail }
serve-bind-failed = { $busid } paylaşıma hazırlanamadı: { $detail }

## Pairing

pin-show = Eşleştirme PIN kodu: { $pin }
pin-hint = Diğer bilgisayarda { $seconds } saniye içinde “usbnexus pair { $name }” komutunu çalıştırın.
pin-no-server = Çalışan bir USB Nexus sunucusu bulunamadı. Başlatmak için: usbnexus serve
pair-enter-pin = Sunucuda görünen PIN kodunu girin:
pair-ok = “{ $name }” ile eşleştirildi ({ $fp }).
pair-already = “{ $name }” ile zaten eşleştirilmiş.

## Devices

local-header = Bu bilgisayardaki USB cihazları:
local-empty = USB cihazı bulunamadı.
list-header = “{ $name }” tarafından paylaşılan cihazlar:
list-empty = Bu sunucu hiçbir cihaz paylaşmıyor.
list-in-use = kullanımda
col-busid = VERİ YOLU
col-id = VID:PID
col-speed = HIZ
col-product = ÜRÜN
col-driver = SÜRÜCÜ
col-state = DURUM
col-name = AD
col-fingerprint = PARMAK İZİ
col-address = ADRES

## Discovery and peers

discover-searching = Yerel ağda aranıyor…
discover-none = Hiç USB Nexus sunucusu bulunamadı.
discover-paired = eşleştirilmiş
peers-empty = Henüz eşleştirilmiş bilgisayar yok.
forget-ok = “{ $name }” kaldırıldı.
forget-unknown = “{ $peer }” ile eşleşen bir bilgisayar yok.

## Attaching

attach-connecting = { $addr } adresine bağlanılıyor…
attach-attached = { $busid } bağlandı (sanal bağlantı noktası { $port }).
attach-disconnected = Bağlantı koptu: { $reason }
attach-retrying = { $seconds } saniye sonra yeniden bağlanılacak…
attach-detached = Cihaz ayrıldı.
attach-stop-hint = Ayırmak için Ctrl+C tuşlarına basın.

## Errors

err-pairing-required = “{ $name }” henüz bu bilgisayarla eşleştirilmemiş. Çalıştırın: usbnexus pair { $target }
err-not-found = Sunucu ağda bulunamadı.
err-version = Sunucu, USB Nexus'un uyumsuz bir sürümünü çalıştırıyor.
err-not-trusted = Bu bilgisayar sunucuyla eşleştirilmemiş.
err-pairing-closed = Sunucuda eşleştirme açık değil. Sunucuda şunu çalıştırın: usbnexus pin
err-pairing-failed = Eşleştirme başarısız. PIN kodunu kontrol edip yeniden deneyin.
err-no-such-device = Sunucu bu cihazı paylaşmıyor.
err-device-busy = Cihaz başka bir bilgisayar tarafından kullanılıyor.
err-internal = Sunucu bir iç hata bildirdi.
err-protocol = Sunucudan beklenmeyen yanıt alındı.
err-connection-lost = Bilgisayarla bağlantı koptu.
err-unsupported = Bu özellik bu işletim sisteminde henüz kullanılamıyor.
err-driver-missing = usbip-win2 sürücüsü kurulu değil. USB Nexus kurulumunu yeniden çalıştırıp usbip-win2 kurulumunu onaylayın.
err-driver-outdated = Kurulu usbip-win2 sürücüsü çok eski. USB Nexus kurulumunu yeniden çalıştırıp usbip-win2 güncellemesini onaylayın.
err-vboxusb-missing = Paylaşım için gereken VirtualBox USB sürücüleri kurulu değil. USB Nexus'u yeniden kurun.
err-device-in-use-by-os = İşletim sistemi bu cihazı kendi sürücüsüyle kullanıyor; bu yüzden bu bilgisayardan paylaşılamıyor.
err-unreachable = Bilgisayara ulaşılamadı. Açık ve ağa bağlı olduğundan emin olun.
err-cancelled = İşlem iptal edildi.
err-permission-denied = USB Nexus gerekli izinlere sahip değil.
err-invalid = İstek anlaşılamadı.
err-other = Bir şeyler ters gitti: { $detail }

## Help layout

help-usage = Kullanım:
help-arguments = Argümanlar
help-options = Seçenekler
help-commands = Komutlar
arg-help = Yardımı göster.
arg-version = Sürümü göster.

## Peers

peers-servers = Eşleştirilmiş sunucular (bu bilgisayar onların cihazlarını kullanabilir):
peers-clients = Eşleştirilmiş istemciler (bu bilgisayarın cihazlarını kullanabilir):

## Desktop app

gui-nav-this-computer = Bu bilgisayar
gui-nav-network = Ağdaki bilgisayarlar
gui-nav-connected = Bağlı cihazlar
gui-nav-paired = Eşleştirilmiş bilgisayarlar
gui-language = Dil
gui-this-title = Bu bilgisayardaki cihazlar
gui-this-subtitle = Diğer bilgisayarların hangi cihazları kullanabileceğini seçin.
gui-share = Paylaş
gui-shared = Paylaşılıyor
gui-not-shared = Paylaşılmıyor
gui-used-by = { $name } kullanıyor
gui-no-local-devices = Bu bilgisayarda USB cihazı bulunamadı.
gui-unnamed-device = USB cihazı
gui-pair-new = Yeni bilgisayar eşleştir
gui-pin-title = Eşleştirme PIN kodu
gui-pin-body = Bu PIN kodunu diğer bilgisayarda girin.
gui-pin-remaining = { $seconds } saniye daha geçerli
gui-pin-expired = PIN kodunun süresi doldu.
gui-pin-new = Yeni PIN
gui-pin-stop = Eşleştirmeyi durdur
gui-close = Kapat
gui-cancel = Vazgeç
gui-network-title = Ağdaki bilgisayarlar
gui-network-subtitle = Yerel ağınızda bulunan USB Nexus bilgisayarları.
gui-refresh = Yenile
gui-searching = Ağda aranıyor…
gui-none-found = Bilgisayar bulunamadı. Diğer bilgisayarda USB Nexus'un çalıştığından emin olun ya da adresle ekleyin.
gui-add-by-address = Adresle ekle
gui-address = Adres
gui-address-hint = ör. 192.168.1.20
gui-pair = Eşleştir
gui-paired = Eşleştirilmiş
gui-pair-title = { $name } ile eşleştir
gui-pair-body = { $name } üzerinde “Yeni bilgisayar eşleştir” düğmesine basın ve orada görünen PIN kodunu yazın.
gui-pin = PIN kodu
gui-pair-done = { $name } ile eşleştirildi.
gui-devices-of = { $name } tarafından paylaşılan cihazlar
gui-no-remote-devices = Bu bilgisayar hiçbir cihaz paylaşmıyor.
gui-connect = Bağlan
gui-disconnect = Bağlantıyı kes
gui-in-use-elsewhere = Başka bir bilgisayar kullanıyor
gui-connected-here = Bu bilgisayara bağlı
gui-back = Geri
gui-connected-title = Bu bilgisayara bağlı cihazlar
gui-connected-subtitle = Uzak cihazlar bağlı kalır; ağ koparsa kendiliğinden yeniden bağlanır.
gui-connected-empty = Bağlı uzak cihaz yok. Bir cihaz bağlamak için “Ağ” bölümünü açın.
gui-on-computer = { $name } üzerinde
gui-state-connecting = Bağlanıyor…
gui-state-attached = Bağlı
gui-state-retrying = { $seconds } sn içinde yeniden bağlanacak
gui-state-stopped = Bağlantı kesildi
gui-state-failed = Başarısız
gui-reconnect = Yeniden bağlan
gui-remove = Kaldır
gui-paired-title = Eşleştirilmiş bilgisayarlar
gui-paired-subtitle = Bilgisayarlar bir kez PIN koduyla eşleştirilir, sonrasında kendiliğinden tanınır.
gui-paired-servers = Cihazlarını kullanabildiğiniz bilgisayarlar
gui-paired-clients = Cihazlarınızı kullanabilen bilgisayarlar
gui-paired-empty = Henüz yok.
gui-remove-confirm = { $name } kaldırılsın mı? Yeniden kullanmak için tekrar eşleştirmeniz gerekecek.
gui-fingerprint = Parmak izi
gui-service-denied-title = Bu kullanıcı USB Nexus hizmetini yönetemiyor
gui-service-denied-body = Yalnızca yöneticiler ve usbnexus grubundakiler yönetebilir. Bu kullanıcıya izin verin (yönetici parolası sorulur) ya da yönetici olarak şunu çalıştırın:
gui-service-denied-command = Komut:
gui-service-denied-user = KULLANICI
gui-grant-access = Bu kullanıcıya izin ver
gui-grant-access-done = İzin verildi.
gui-setup-kernel-modules = USB paylaşımı için gereken çekirdek modülleri eksik: { $modules }. Kurun; hizmet onları kendisi yükler:
gui-setup-kernel-modules-nocmd = Çoğu dağıtımda çekirdek paketiyle gelirler (Ubuntu: linux-modules-extra, Fedora: kernel-modules-extra).
gui-service-down-title = USB Nexus hizmeti çalışmıyor
gui-service-down-body = Hizmeti başlatın; bu pencere ona kendiliğinden bağlanır.
gui-service-down-linux = Linux'ta şunu çalıştırın:
gui-service-down-windows = Windows'ta yönetici olarak şunu çalıştırın:
gui-service-down-macos = macOS'ta şunu çalıştırın:
gui-retry = Yeniden dene
gui-details = Ayrıntılar
err-service-unavailable = USB Nexus hizmetine ulaşılamadı.

## Web interface

gui-web-login-title = { $name } oturumunu aç
gui-web-password = Parola
gui-web-sign-in = Giriş yap
gui-web-sign-out = Çıkış yap
gui-web-wrong-password = Parola yanlış.
gui-web-locked = Çok fazla deneme. { $seconds } saniye sonra yeniden deneyin.
cmd-web-about = Web arayüzünü aç veya kapat.
cmd-enable-about = Web arayüzünü aç (ilk seferde parola sorar).
cmd-disable-about = Web arayüzünü kapat.
cmd-password-about = Web arayüzü parolasını değiştir.
cmd-status-about = Web arayüzünün açık olup olmadığını ve adresini göster.
arg-lan = Ağdaki diğer bilgisayarlardan erişime izin ver.
arg-port = Web arayüzünün TCP bağlantı noktası.
web-on = Web arayüzü açık:
web-off = Web arayüzü kapalı.
web-local-only = Yalnızca bu bilgisayardan açılabilir. Diğer bilgisayarlar için --lan kullanın.
web-fingerprint = Tarayıcı sertifika uyarısı verecek; sertifikanın parmak izi şu olmalı: { $fp }
web-not-running = Web arayüzü etkin ama başlatılamadı: { $detail }
web-password-prompt = Yeni web arayüzü parolası:
web-password-repeat = Parolayı tekrar girin:
web-password-mismatch = Parolalar eşleşmiyor.
web-password-set = Web arayüzü parolası değiştirildi.
err-weak-password = Parola en az 8 karakter olmalıdır.
err-password-required = Önce bir web arayüzü parolası belirleyin: usbnexus web password
err-forbidden = Bu yalnızca bilgisayarın kendisinden değiştirilebilir.
err-not-logged-in = Lütfen yeniden giriş yapın.

## Hotplug, access control and usage log

arg-device = Cihaz: kimlik veya veri yolu kimliği (bkz. usbnexus list SUNUCU).
cmd-policy-about = Paylaşılan cihazları kimlerin kullanabileceğini göster veya seç.
arg-policy = open: eşleştirilmiş her bilgisayar paylaşılan her cihazı kullanabilir; restricted: yalnızca cihaz bazında izin verilen bilgisayarlar.
arg-no-server = Bu bilgisayarın USB cihazlarını paylaşmayı kurma.
arg-no-client = Diğer bilgisayarların USB cihazlarını kullanmayı kurma.
arg-web = Web arayüzü: off (kapalı), local (yalnızca bu bilgisayar) veya network (tüm ağ).
arg-web-port = Web arayüzünün portu (varsayılan 3242).
arg-web-password-file = Yeni web arayüzü parolasını içeren dosya.
cmd-history-about = Bu bilgisayarın paylaşılan cihazlarının kullanım kaydını göster.
arg-csv = Tüm kayıtları CSV olarak yazdır (ör. bir dosyaya kaydetmek için).
arg-limit = Gösterilecek kayıt sayısı.
col-device-id = CİHAZ KİMLİĞİ
col-time = ZAMAN (UTC)
col-event = OLAY
col-computer = BİLGİSAYAR
col-duration = SÜRE
state-unplugged = takılı değil
state-no-permission = izin yok
serve-denied = “{ $client }” bilgisayarının { $busid } cihazını kullanma izni yok.
attach-waiting-device = Cihaz sunucuya takılı değil; takılması bekleniyor…
attach-queued = Cihazı başka bir bilgisayar kullanıyor; bu bilgisayar sırada { $position }.
policy-open = Eşleştirilmiş her bilgisayar paylaşılan her cihazı kullanabilir (açık).
policy-restricted = Eşleştirilmiş bilgisayarlar yalnızca izin verilen cihazları kullanabilir (kısıtlı).
history-header = Kullanım kaydı (kayıtlar { $days } gün saklanır):
history-empty = Kullanım kaydı boş.
history-paired = eşleştirildi
history-pairing-failed = yanlış PIN
history-attached = kullanmaya başladı
history-detached = kullanmayı bıraktı
history-denied = reddedildi (izin yok)
err-access-denied = Bu bilgisayarın bu cihazı kullanma izni yok.

gui-nav-history = Geçmiş
gui-nav-settings = Ayarlar
gui-not-plugged-in = Takılı değil
gui-tracked-by-port = bağlantı noktasıyla izleniyor
gui-tracked-by-port-hint = Bu cihazın seri numarası yok; bu yüzden takılı olduğu USB bağlantı noktasından tanınır. Yeniden aynı bağlantı noktasına takın.
gui-no-permission = İzin yok
gui-no-permission-hint = O bilgisayarın sahibi bu bilgisayarın cihazı kullanmasına izin vermedi.
gui-connect-when-plugged-in = Cihaz takılır takılmaz kendiliğinden bağlanır.
gui-state-waiting-device = Cihaz bekleniyor
gui-state-queued = Başka bilgisayar kullanıyor; sırada { $position }.
gui-save = Kaydet
gui-saved = Kaydedildi.
gui-skip = Atla
gui-access-title = Bu cihazı kimler kullanabilir
gui-access-everyone = Eşleştirilmiş tüm bilgisayarlar
gui-access-some = Seçili bilgisayarlar ({ $count })
gui-access-nobody = Henüz hiçbir bilgisayar
gui-access-mode-default = Varsayılan ayarı kullan
gui-access-default-open = Şu an: eşleştirilmiş tüm bilgisayarlar.
gui-access-default-restricted = Şu an: yalnızca aşağıda seçilen bilgisayarlar.
gui-access-mode-open = Eşleştirilmiş tüm bilgisayarlar
gui-access-mode-open-body = Bu bilgisayarla eşleştirilmiş her bilgisayar kullanabilir.
gui-access-mode-selected = Yalnızca seçili bilgisayarlar
gui-access-mode-selected-body = Yalnızca aşağıda işaretlenen bilgisayarlar kullanabilir.
gui-access-computers = Kullanmasına izin verilen bilgisayarlar
gui-access-revoke-note = İzni kaldırılan bilgisayarın cihazla bağlantısı hemen kesilir.
gui-client-devices = Cihazlar
gui-client-devices-title = { $name } bilgisayarının kullanabileceği cihazlar
gui-client-devices-body = Bu bilgisayarın kullanabileceği paylaşılan cihazları işaretleyin.
gui-client-devices-after-pairing = Paylaşılan cihazları yalnızca izin verilen bilgisayarlar kullanabilir. Bu bilgisayarın kullanabileceği cihazları işaretleyin; atlarsanız şimdilik hiçbirini kullanamaz.
gui-no-shared-devices = Bu bilgisayar henüz hiçbir cihaz paylaşmıyor.
gui-roles-title = Kullanım şekli
gui-roles-body = Seçilmeyen kullanımın ekranları gizlenir. Eklenen kullanım için gerekenler kurulur.
gui-role-server = Sunucu olarak kullan (bu bilgisayarın USB cihazlarını paylaş)
gui-role-client = İstemci olarak kullan (diğer bilgisayarların USB cihazlarını kullan)
gui-roles-client-note = Gerekirse usbip-win2 sürücüsü kurulur; USB cihazları birkaç saniye kesilir ve Windows'un yeniden başlatılması gerekebilir.
gui-roles-applying = Uygulanıyor…
gui-reboot-required = Kurulumun tamamlanması için bilgisayarı yeniden başlatın.
gui-used-by-waiting = { $name } kullanıyor · { $count } bilgisayar bekliyor
gui-col-permissions = İzinler
gui-col-status = Durum
gui-in-use-title = Şu an kullanan
gui-nobody-using = Şu an cihazı kimse kullanmıyor.
gui-since = { $time }'den beri
gui-disconnect-user = Bağlantıyı kes
gui-disconnected-user = { $name } bağlantısı kesildi.
gui-queue-title = Sırada bekleyenler ({ $count })
gui-queue-empty = Bekleyen yok.
gui-badge-using = kullanıyor
gui-badge-queued = sırada ({ $position }.)
gui-handover-title = Otomatik devir
gui-handover-default-on = Varsayılan (açık, { $seconds } sn)
gui-handover-default-off = Varsayılan (kapalı)
gui-handover-on = Açık
gui-handover-off = Kapalı
gui-handover-before = Başka bir bilgisayar beklerken
gui-handover-after = saniye kullanılmazsa sıradakine geçer.
gui-kind-storage = Depolama
gui-kind-input = Klavye / fare
gui-kind-printer = Yazıcı
gui-kind-dongle = Lisans dongle'ı
gui-kind-other = Diğer cihaz
gui-web-title = Web arayüzü
gui-web-body = Bu bilgisayarı tarayıcıdan, parolayla yönetin.
gui-web-local-only-note = Web arayüzü, masaüstü uygulamasından ya da bilgisayarın kendisinde “usbnexus web” komutuyla ayarlanır.
gui-web-enabled = Web arayüzü açık
gui-web-access-local = Yalnızca bu bilgisayar
gui-web-access-network = Tüm ağ
gui-web-port = Port
gui-web-new-password = Yeni parola
gui-web-repeat-password = Yeni parola (tekrar)
gui-web-password-keep = En az 8 karakter. Boş bırakırsanız mevcut parola korunur.
gui-web-password-required = En az 8 karakter.
gui-web-mismatch = Parolalar uyuşmuyor.
gui-web-open-at = Adres:
gui-web-fingerprint = Tarayıcı sertifika uyarısı verir; sertifikanın parmak izi { $fp }.
gui-policy-title = Paylaşılan cihazları kimler kullanabilir
gui-policy-body = Bilgisayarların her zaman önce eşleştirilmesi gerekir. Bu ayar her paylaşılan cihaz için varsayılandır; her cihaz için ayrıca değiştirilebilir.
gui-policy-first-title = Paylaşılan cihazlarınızı kimler kullanabilsin?
gui-policy-first-body = Bilgisayarların her zaman önce PIN koduyla eşleştirilmesi gerekir. Eşleştirilmiş bilgisayarların neler yapabileceğini seçin:
gui-policy-later = Bunu daha sonra Ayarlar'dan değiştirebilirsiniz.
gui-policy-open = Eşleştirilmiş her bilgisayar
gui-policy-open-body = Eşleştirilmiş her bilgisayar paylaşılan her cihazı kullanabilir.
gui-policy-restricted = Yalnızca izin verilen bilgisayarlar
gui-policy-restricted-body = Her cihaz için hangi eşleştirilmiş bilgisayarların kullanabileceğini siz seçersiniz.
gui-retention-title = Kullanım kaydı
gui-retention-body = Eşleştirmeler, yanlış PIN girişleri, cihaz kullanımı ve reddedilen istekler kaydedilir. Eski kayıtlar kendiliğinden silinir.
gui-retention-days = Kayıtların saklanacağı süre (gün)
gui-settings-title = Ayarlar
gui-history-title = Geçmiş
gui-history-subtitle = Bu bilgisayarın cihazlarını kimin, ne zaman kullandığı. Kayıtlar { $days } gün saklanır.
gui-history-empty = Henüz hiçbir şey kaydedilmedi.
gui-history-export = CSV olarak dışa aktar
gui-history-time = Zaman
gui-history-event = Olay
gui-history-computer = Bilgisayar
gui-history-device = Cihaz
gui-history-device-id = Cihaz kimliği
gui-history-duration = Süre
gui-history-paired = Eşleştirildi
gui-history-pairing-failed = Yanlış PIN
gui-history-attached = Kullanmaya başladı
gui-history-detached = Kullanmayı bıraktı
gui-history-denied = Reddedildi: izin yok

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = Kullanım şekli
setup-roles-subtitle = USB Nexus'u bu bilgisayarda nasıl kullanacağınızı seçin.
setup-role-server = Sunucu olarak kullanacağım
setup-role-client = İstemci olarak kullanacağım
setup-role-web = Web'den erişim olacak
setup-usbip-install-note = usbip-win2 sürücüsü de kurulacak. Kurulum sırasında USB cihazları birkaç saniye kesilir; ardından Windows'un yeniden başlatılması gerekir.
setup-usbip-update-note = Kurulu usbip-win2 sürücüsü çok eski, güncellenecek. Kurulum sırasında USB cihazları birkaç saniye kesilir; ardından Windows'un yeniden başlatılması gerekir.
setup-usbip-present-note = usbip-win2 sürücüsü bu bilgisayarda zaten kurulu.
setup-usbip-failed = usbip-win2 sürücüsü kurulamadı. Daha sonra USB Nexus kurulumunu yeniden çalıştırarak tekrar deneyebilirsiniz.
setup-service-failed = USB Nexus hizmeti kurulamadı. Ayrıntılar kurulum günlüğünde.
setup-web-title = Web arayüzü
setup-web-subtitle = Bu bilgisayarı tarayıcıdan yönetmek için ayarlar.
setup-web-access = Erişim:
setup-web-local = Yalnızca bu bilgisayar
setup-web-network = Tüm ağ
setup-web-port = Port:
setup-web-port-free = ✓ Port kullanılabilir
setup-web-port-busy = ✗ Bu port başka bir program tarafından kullanılıyor
setup-web-port-invalid = ✗ 1 ile 65535 arasında bir sayı girin
setup-web-password = Parola:
setup-web-password-repeat = Parola (tekrar):
setup-web-password-hint = En az 8 karakter.
setup-web-password-keep = En az 8 karakter. Boş bırakırsanız mevcut parola korunur.
