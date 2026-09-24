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
err-driver-missing = usbip-win2 sürücüsü kurulu değil. github.com/vadimgrn/usbip-win2/releases adresinden kurup yeniden deneyin.
err-vboxusb-missing = Paylaşım için gereken VirtualBox USB sürücüleri kurulu değil. USB Nexus'u yeniden kurun.
err-device-in-use-by-os = İşletim sistemi bu cihazı kendi sürücüsüyle kullanıyor; bu yüzden bu bilgisayardan paylaşılamıyor.
err-unreachable = Bilgisayara ulaşılamadı. Açık ve ağa bağlı olduğundan emin olun.
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
gui-nav-network = Ağ
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
gui-service-down-title = USB Nexus hizmeti çalışmıyor
gui-service-down-body = Hizmeti başlatın; bu pencere ona kendiliğinden bağlanır.
gui-service-down-linux = Linux'ta şunu çalıştırın:
gui-service-down-windows = Windows'ta yönetici olarak şunu çalıştırın:
gui-service-down-macos = macOS'ta şunu çalıştırın:
gui-retry = Yeniden dene
gui-details = Ayrıntılar
err-service-unavailable = USB Nexus hizmetine ulaşılamadı.
