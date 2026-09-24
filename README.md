# USB Nexus

Güvenli, kolay kurulan USB-over-IP çözümü (geliştirme aşamasında).

Hedefler:
- TLS şifreleme ve sertifika tabanlı kimlik doğrulama, PIN ile eşleştirme
- Ağdaki sunucuları otomatik bulma (mDNS)
- Bağlantı koptuğunda otomatik yeniden bağlanma
- Grafik arayüz (Tauri) ve paketli kurulum

## Yapı

| Crate | Açıklama |
|---|---|
| `crates/usbnexus-proto` | USB/IP kablo protokolü (spesifikasyondan sıfırdan yazıldı) |
| `crates/usbnexus-core` | TLS tüneli, PIN ile eşleştirme, mDNS ile bulma, otomatik yeniden bağlanma, Linux arka uçları |
| `crates/usbnexus-i18n` | Arayüz çevirileri ([Fluent](https://projectfluent.org/)) |
| `crates/usbnexus-cli` | `usbnexus` komut satırı aracı ve servis |
| `apps/desktop` | Masaüstü uygulaması (Tauri 2; arayüz `apps/desktop/ui`) |
| `packaging/linux` | systemd servisi ve elle kurulum notları |
| `packaging/windows` | Windows hizmeti, NSIS kurulum paketi ayarları, VBoxUSB sürücüleri |
| `packaging/macos` | launchd hizmeti, `.pkg` üretme betiği |
| `locales/` | Çeviri dosyaları: `en.ftl`, `tr.ftl` |

## Diller

Arayüz şu an **Türkçe** ve **İngilizce** destekliyor. Dil, sistem ayarından (`LANG`) otomatik seçilir;
`--lang tr` ile değiştirilebilir. Yeni bir dil eklemek için `locales/en.ftl` dosyasını kopyalayıp çevirin
ve `crates/usbnexus-i18n/src/lib.rs` içindeki `LOCALES` listesine ekleyin. Eksik çeviri olursa testler
başarısız olur.

## Masaüstü uygulaması

<p align="center"><img src="docs/screenshots/this.png" width="720" alt="Bu bilgisayardaki cihazlar"></p>

| | |
|---|---|
| <img src="docs/screenshots/remote.png" width="400" alt="Uzak cihazlar"> | <img src="docs/screenshots/pin.png" width="400" alt="Eşleştirme PIN kodu"> |

Uygulama yetkisiz bir kullanıcı olarak çalışır ve tüm işleri arka plandaki **USB Nexus hizmetine**
(`usbnexus daemon`) yaptırır. Pencere kapatılsa bile paylaşımlar ve bağlantılar sürer; hizmet yeniden
başladığında kayıtlı bağlantılar kendiliğinden geri kurulur. Kurulum: [packaging/linux](packaging/linux/README.md).

Donanım olmadan denemek için (demo cihazlarla):

```sh
cargo build -p usbnexus-cli -p usbnexus-desktop
export USBNEXUS_SOCKET=/tmp/usbnexus-demo.sock
target/debug/usbnexus daemon --demo --state-dir /tmp/usbnexus-demo --socket $USBNEXUS_SOCKET &
target/debug/usbnexus-desktop
```

## Kurulum paketleri

| Platform | Paketler | Nasıl üretilir |
|---|---|---|
| Debian/Ubuntu | `usbnexus_*.deb` (hizmet + komut satırı), `usb-nexus_*.deb` (masaüstü) | [packaging/linux](packaging/linux/README.md) |
| Fedora/RHEL/openSUSE | `usbnexus-*.rpm`, `USB Nexus-*.rpm` | [packaging/linux](packaging/linux/README.md) |
| Windows | `USB Nexus_*_x64-setup.exe` | [packaging/windows](packaging/windows/README.md) |
| macOS | `USB Nexus-*.pkg` | [packaging/macos](packaging/macos/README.md) |

Bir `v*` sürüm etiketi pushlandığında `.github/workflows/release.yml` hepsini üretir ve taslak bir
GitHub sürümüne ekler.

## Derleme

Linux'ta masaüstü uygulaması için önce sistem kütüphaneleri gerekir:

```sh
sudo apt install libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev
```

```sh
cargo build --release      # çıktılar: target/release/usbnexus, target/release/usbnexus-desktop
cargo test --workspace
```

## Kullanım (Linux, ilk sürüm)

Her iki bilgisayarda çekirdek modüllerini yükleyin ve komutları `sudo` ile çalıştırın:

```sh
# Sunucu (USB cihazının takılı olduğu bilgisayar)
sudo modprobe usbip-host
sudo usbnexus local                         # paylaşılabilecek cihazları listeler
sudo usbnexus serve --export 1-2 --pair     # 1-2 cihazını paylaşır, PIN gösterir
sudo usbnexus pin                           # (çalışan sunucu için) yeni PIN üretir

# İstemci (cihazı kullanacak bilgisayar)
sudo modprobe vhci-hcd
sudo usbnexus discover                      # ağdaki sunucuları bulur
sudo usbnexus pair ofis-pc                  # PIN sorar ve eşleştirir (bir kez)
sudo usbnexus list ofis-pc                  # paylaşılan cihazları listeler
sudo usbnexus attach ofis-pc 1-2            # cihazı bağlar; bağlantı koparsa kendisi yeniden bağlanır
```

Bağlantı TLS 1.3 ile şifrelenir. İki taraf da birbirini PIN ile eşleştirme sırasında kaydedilen sertifika
parmak iziyle doğrular. Sunucunun IP adresi değişse bile istemci onu yerel ağda (mDNS) parmak izinden
yeniden bulur.

## Platform yol haritası

| Senaryo | Sürücü | Durum |
|---|---|---|
| Linux sunucu ↔ Linux istemci | Çekirdekteki `usbip-host` / `vhci-hcd` | Çalışıyor (donanım testi bekliyor) |
| Windows istemci | usbip-win2 (attestation imzalı, BSD-2) | Yazıldı; gerçek Windows'ta test bekliyor ([ayrıntılar](packaging/windows/README.md)) |
| Windows sunucu | VBoxUSB (Oracle + Microsoft imzalı, GPL-3.0) | Yazıldı; gerçek Windows'ta test bekliyor |
| Grafik arayüz (Linux) | Tauri, aynı `locales/` çevirileri | Çalışıyor (demo ile test edildi) |
| Grafik arayüz (Windows) | Hizmetle adlandırılmış kanal (`\\.\pipe\usbnexus`) | Yazıldı; test bekliyor |
| macOS sunucu (Mac'teki cihazı paylaşma) | libusb (programa gömülü) | Yazıldı; macOS'un kendi sürücüsünü kullandığı cihazlar hariç ([ayrıntılar](packaging/macos/README.md)) |
| macOS istemci | — | Mümkün değil (Apple sanal USB denetleyiciye izin vermiyor) |
| Grafik arayüz (macOS) | Hizmetle Unix soketi | Yazıldı; test bekliyor |

## Lisans

USB Nexus özgür yazılımdır: [GNU Genel Kamu Lisansı sürüm 3](LICENSE) veya (tercihinize göre) daha sonraki
bir sürümün koşulları altında yeniden dağıtabilir ve/veya değiştirebilirsiniz (`GPL-3.0-or-later`).

USB Nexus is free software, licensed under the GNU General Public License v3.0 or later.
See [LICENSE](LICENSE).
