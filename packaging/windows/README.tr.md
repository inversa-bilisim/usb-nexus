# Windows

[English](README.md) · **Türkçe**

## Bileşenler

| Bileşen | Görev |
|---|---|
| `usbnexus.exe` | Arka plan hizmeti (`USB Nexus`, LocalSystem) ve komut satırı aracı |
| `usbnexus-desktop.exe` | Masaüstü uygulaması; hizmetle `\\.\pipe\usbnexus` üzerinden konuşur |
| [usbip-win2](https://github.com/vadimgrn/usbip-win2) | Uzak cihazları Windows'a takan, Microsoft imzalı sürücü (BSD-2-Clause, [lisans](usbip-win2/LICENSE.txt)); kurulum paketinin içinde gelir |
| `drivers\` (VBoxUSB, VBoxUSBMon) | Bu bilgisayarın cihazlarını paylaşmak için VirtualBox USB sürücüleri (Oracle + Microsoft imzalı, GPL-3.0; [ayrıntılar](drivers/README.md)) |

## Kurulum paketi

`USB Nexus_*_x64-setup.exe` yönetici yetkisiyle, Windows'un dilinde çalışır (o dil yoksa İngilizce).
Kurulum dosyası henüz kod imzalı olmadığından Windows SmartScreen "Windows bilgisayarınızı korudu"
uyarısı gösterebilir: **Daha fazla bilgi → Yine de çalıştır** deyin. Kurulum klasöründen sonra
bilgisayarın nasıl kullanılacağını sorar; seçeneklerin hepsi işaretli gelir:

| Seçenek | Ne yapar |
|---|---|
| **Sunucu olarak kullanacağım** | VBoxUSB sürücülerini kurar; uygulamada "Bu bilgisayar" ekranı görünür. |
| **İstemci olarak kullanacağım** | usbip-win2 yoksa ya da 0.9.7.6'dan eskiyse pakettekini kurar (sayfadaki not bunu söyler; USB cihazları birkaç saniye kesilir, ardından Windows'un yeniden başlatılması gerekir). Uygulamada "Ağdaki bilgisayarlar" ve "Bağlı cihazlar" ekranları görünür. |
| **Web'den erişim olacak** | Sonraki sayfa sorar: yalnızca bu bilgisayardan (varsayılan) ya da tüm ağdan erişim, port (3242; yazarken boş olup olmadığı denetlenir) ve parola (en az 8 karakter). Kurulum bitince web arayüzü tarayıcıda açılır (yeniden başlatma gerekiyorsa ondan sonra). |

Sunucu ve istemciden en az biri seçili olmalıdır. Hizmet her durumda kurulup başlatılır, güvenlik
duvarı kuralı eklenir. Eski sürümün üzerine kurulumda önce eski hizmet durdurulur, önceki seçimler
işaretli gelir, parola boş bırakılırsa mevcut parola korunur. Sessiz kurulum (`/S`) sunucu ve istemciyi
web arayüzü olmadan kurar.

Roller sonradan uygulamanın **Ayarlar** bölümünden değiştirilebilir; eklenen rol için gerekenler kurulur.
Kaldırma hizmeti de kaldırır; usbip-win2 kalır (başka programlar da kullanıyor olabilir; "Yüklü
uygulamalar"da kendi kaydı vardır).

## Kurulum paketini üretme (Windows üzerinde)

```powershell
cargo install tauri-cli --version "=2.12.0" --locked   # installer.nsi bu sürümün NSIS şablonunun kopyası
cargo build --release -p usbnexus-cli
New-Item -ItemType Directory -Force apps\desktop\src-tauri\binaries
Copy-Item target\release\usbnexus.exe apps\desktop\src-tauri\binaries\usbnexus-x86_64-pc-windows-msvc.exe
packaging\windows\fetch-usbip-win2.ps1   # usbip-win2 kurulumunu indirir ve SHA-256 ile doğrular
# packaging\windows\drivers kendiliğinden pakete eklenir (tauri.bundle.json -> resources)
cd apps\desktop\src-tauri
cargo tauri build --config ..\..\..\packaging\windows\tauri.bundle.json
```

Çıktı: `target\release\bundle\nsis\USB Nexus_*_x64-setup.exe`.

- `installer.nsi`, Tauri'nin NSIS şablonunun iki ek sayfa (`usbnexus-pages.nsh`) içeren kopyasıdır.
- Kurulum metinleri `locales/*.ftl` içindeki `setup-*` mesajlarından gelir; `usbnexus-strings.nsh`
  onlardan üretilir (`UPDATE_NSIS_STRINGS=1 cargo test -p usbnexus-i18n`). Yeni bir dil için
  `tauri.bundle.json` içindeki `languages` listesine de kayıt gerekir.

## Elle kurulum

```powershell
# Yönetici PowerShell
usbnexus.exe service install                       # hizmeti kurar ve başlatır (iki rol de)
usbnexus.exe service install --no-client --web local --web-port 3242 --web-password-file pw.txt
usbnexus.exe service uninstall
```

- Ayarlar, anahtarlar ve günlük: `C:\ProgramData\USB Nexus\` (`service.log`)
- Denetim kanalı `\\.\pipe\usbnexus`: SYSTEM, yöneticiler ve oturum açmış yerel kullanıcılar
- `usbip.exe` şuralarda aranır: `C:\Program Files\USBip\usbip.exe`, `PATH` ya da `USBNEXUS_USBIP`
  ortam değişkeni

## Durum

İki Windows 11 bilgisayarda denendi: USB bellek paylaşımı (VBoxUSB), belleğin diğer bilgisayarda
kullanılması (usbip-win2), masaüstü uygulaması, kurulum sayfaları ve ağdan web arayüzü. Paylaşım
sırasında cihaz Windows'tan alınıp VBoxUSB'ye verilir, oturum bitince normal sürücüsüne geri döner.
`service install` sürücü paketini ekler ve `VBoxUSBMon` çekirdek hizmetini kaydeder (usbipd-win
kuruluysa onun kaydını kullanır).
