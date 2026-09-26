# Windows

## Bileşenler

| Bileşen | Görev |
|---|---|
| `usbnexus.exe` | Arka plan hizmeti (`USB Nexus`, LocalSystem) ve komut satırı aracı |
| `usbnexus-desktop.exe` | Masaüstü uygulaması; hizmetle `\\.\pipe\usbnexus` üzerinden konuşur |
| [usbip-win2](https://github.com/vadimgrn/usbip-win2) | Uzak cihazları Windows'a takan, Microsoft imzalı sürücü (BSD-2-Clause, [lisans](usbip-win2/LICENSE.txt)); kurulum paketinin içinde gelir |
| `drivers\` (VBoxUSB, VBoxUSBMon) | Bu bilgisayarın cihazlarını paylaşmak için VirtualBox USB sürücüleri (Oracle + Microsoft imzalı, GPL-3.0; [ayrıntılar](drivers/README.md)) |

## Kurulum paketi (Windows üzerinde)

```powershell
cargo install tauri-cli --version "^2" --locked
cargo build --release -p usbnexus-cli
New-Item -ItemType Directory -Force apps\desktop\src-tauri\binaries
Copy-Item target\release\usbnexus.exe apps\desktop\src-tauri\binaries\usbnexus-x86_64-pc-windows-msvc.exe
packaging\windows\fetch-usbip-win2.ps1   # usbip-win2 kurulumunu indirir ve SHA-256 ile doğrular
# packaging\windows\drivers is bundled automatically (tauri.bundle.json -> resources)
cd apps\desktop\src-tauri
cargo tauri build --config ..\..\..\packaging\windows\tauri.bundle.json
```

Çıktı: `target\release\bundle\nsis\USB Nexus_*_x64-setup.exe`. Kurulum yönetici yetkisiyle çalışır,
hizmeti kaydedip başlatır ve güvenlik duvarı kuralını ekler. Kaldırma hizmeti de kaldırır.

### usbip-win2

Kurulum, pakete eklenmiş usbip-win2 kurulumunu (`fetch-usbip-win2.ps1` içindeki sürüm) şu durumlarda
önerir; kullanıcı onaylarsa sessizce kurar:

| Bilgisayardaki durum | Ne olur |
|---|---|
| usbip-win2 yok | "Şimdi kurulsun mu?" sorulur |
| 0.9.7.6'dan eski sürüm var (`attach --once` yok) | "Güncellensin mi?" sorulur |
| 0.9.7.6 veya daha yeni sürüm var | Hiçbir şey sorulmaz, mevcut kurulum kullanılır |

- Sürüm, usbip-win2'nin kaldırma kaydından (`DisplayVersion`), yoksa `usbip.exe` dosya sürümünden okunur.
- Kurulum sırasında USB 3 hub'ları yeniden başlar (takılı cihazlar birkaç saniye kesilir); sonrasında
  Windows'un yeniden başlatılması gerekir. Son sayfa bunu önerir.
- Sessiz kurulumda (`/S`) usbip-win2 kurulmaz.
- USB Nexus kaldırılınca usbip-win2 kalır (başka programlar da kullanıyor olabilir; "Yüklü uygulamalar"dan
  ayrıca kaldırılabilir).
- Eski bir usbip-win2 ile bağlanma denenirse arayüz "usbip-win2 çok eski" hatası gösterir.

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
