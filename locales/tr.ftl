# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Turkish.

## Command line help

app-about = Güvenli USB-over-IP: USB cihazlarını ağ üzerinden şifreli, eşleştirmeli ve otomatik yeniden bağlanarak paylaşın.
arg-lang = Arayüz dili (ör. en, tr). Varsayılan: sistem dili.
arg-state-dir = Anahtarların ve eşleştirilmiş bilgisayarların tutulduğu klasör.
arg-name = Bu bilgisayarın diğerlerine görünen adı.
arg-verbose = Ayrıntılı günlük mesajlarını göster.

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
