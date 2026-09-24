# Windows

## Bileşenler

| Bileşen | Görev |
|---|---|
| `usbnexus.exe` | Arka plan hizmeti (`USB Nexus`, LocalSystem) ve komut satırı aracı |
| `usbnexus-desktop.exe` | Masaüstü uygulaması; hizmetle `\\.\pipe\usbnexus` üzerinden konuşur |
| [usbip-win2](https://github.com/vadimgrn/usbip-win2) | Uzak cihazları Windows'a takan, Microsoft imzalı sürücü (ayrı kurulur) |
| `drivers\` (VBoxUSB, VBoxUSBMon) | Bu bilgisayarın cihazlarını paylaşmak için VirtualBox USB sürücüleri (Oracle + Microsoft imzalı, GPL-3.0; [ayrıntılar](drivers/README.md)) |

## Kurulum paketi (Windows üzerinde)

```powershell
cargo install tauri-cli --version "^2" --locked
cargo build --release -p usbnexus-cli
New-Item -ItemType Directory -Force apps\desktop\src-tauri\binaries
Copy-Item target\release\usbnexus.exe apps\desktop\src-tauri\binaries\usbnexus-x86_64-pc-windows-msvc.exe
# packaging\windows\drivers is bundled automatically (tauri.bundle.json -> resources)
cd apps\desktop\src-tauri
cargo tauri build --config ..\..\..\packaging\windows\tauri.bundle.json
```

Çıktı: `target\release\bundle\nsis\USB Nexus_*_x64-setup.exe`. Kurulum yönetici yetkisiyle çalışır,
hizmeti kaydedip başlatır, güvenlik duvarı kuralını ekler ve usbip-win2 kurulu değilse indirme
sayfasını önerir. Kaldırma hizmeti de kaldırır.

## Elle kurulum

```powershell
# Yönetici PowerShell
usbnexus.exe service install     # hizmeti kurar ve başlatır
usbnexus.exe service uninstall   # kaldırır
```

- Ayarlar, anahtarlar ve günlük: `C:\ProgramData\USB Nexus\` (`service.log`)
- Denetim kanalı `\\.\pipe\usbnexus`: SYSTEM, yöneticiler ve oturum açmış yerel kullanıcılar
- `usbip.exe` konumu: `C:\Program Files\USBip\usbip.exe`, `PATH` veya `USBNEXUS_USBIP` ortam değişkeni

## Durum

- Windows **istemci** (uzak cihazı Windows'ta kullanma): usbip-win2 ile. Gerçek Windows'ta henüz denenmedi.
- Windows **sunucu** (Windows'taki cihazı paylaşma): VBoxUSB ile yazıldı; gerçek Windows'ta test bekliyor.
  Paylaşım sırasında cihaz Windows'tan alınıp VBoxUSB'ye verilir, oturum bitince normal sürücüsüne geri döner.
  `service install` sürücü paketini ekler ve `VBoxUSBMon` çekirdek hizmetini kaydeder
  (usbipd-win kuruluysa onun kaydını kullanır).
