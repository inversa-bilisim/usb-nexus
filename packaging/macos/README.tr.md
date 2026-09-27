# macOS

[English](README.md) · **Türkçe**

## Neler destekleniyor

| Senaryo | Durum |
|---|---|
| **Bu Mac'e takılı cihazı paylaşmak** (Mac sunucu) | libusb ile. macOS'un kendi sürücüsü olmayan cihazlar (programlayıcılar, geliştirme kartları, bazı yazıcı/tarayıcılar, özel donanım) paylaşılabilir. |
| USB bellek, klavye, fare gibi macOS'un kendi sürücüsüyle kullandığı cihazlar | Apple'ın `com.apple.vm.device-access` yetkisi gerekir; bu yetki olmadan "işletim sistemi kullanıyor" hatası verilir. |
| **Uzaktaki cihazı Mac'te kullanmak** (Mac istemci) | Desteklenmiyor: macOS, üçüncü taraflara sanal USB denetleyici yazma imkânı vermiyor. |
| İzokron (ses/görüntü) aktarımları | Henüz yok. |

## Paket oluşturma (Mac üzerinde)

```sh
cargo install tauri-cli --version "^2" --locked
packaging/macos/build-pkg.sh
```

Çıktı: `target/USB Nexus-<sürüm>.pkg`. Kurulum:

- `/usr/local/bin/usbnexus` (komut satırı + hizmet)
- `/Library/LaunchDaemons/org.usbnexus.daemon.plist` (açılışta root olarak başlar)
- `/Applications/USB Nexus.app`

Ayarlar ve anahtarlar `/Library/Application Support/USB Nexus`, günlük `/Library/Logs/USB Nexus/daemon.log`.
Masaüstü uygulamasını `admin` grubundaki kullanıcılar kullanabilir. Kaldırmak için:
`sudo packaging/macos/uninstall.sh`.

## İmzalama ve noter onayı

İmzasız paket ve uygulama çalışır, ancak Gatekeeper ilk açılışta uyarı verir (Finder'da sağ tık → Aç).
Uyarısız dağıtım için bir Apple Developer hesabı (yıllık ücretli) ile `SIGN_APP` / `SIGN_PKG`
değişkenleri verilerek imzalanmalı ve `xcrun notarytool` ile noter onayı alınmalıdır.
