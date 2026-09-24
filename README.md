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
| `crates/usbnexus-cli` | `usbnexus` komut satırı aracı ve servis (yapım aşamasında) |
| `locales/` | Çeviri dosyaları: `en.ftl`, `tr.ftl` |

## Diller

Arayüz şu an **Türkçe** ve **İngilizce** destekliyor. Dil, sistem ayarından (`LANG`) otomatik seçilir;
`--lang tr` ile değiştirilebilir. Yeni bir dil eklemek için `locales/en.ftl` dosyasını kopyalayıp çevirin
ve `crates/usbnexus-i18n/src/lib.rs` içindeki `LOCALES` listesine ekleyin. Eksik çeviri olursa testler
başarısız olur.

## Derleme

```sh
cargo build --release      # çıktı: target/release/usbnexus
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
| Windows sunucu | VBoxUSB (Oracle, Microsoft imzalı, GPL-3.0) | Planlandı |
| Windows istemci | usbip-win2 (attestation imzalı, BSD-2) | Planlandı |
| Grafik arayüz | Tauri, aynı `locales/` çevirileri | Planlandı |

## Lisans

USB Nexus özgür yazılımdır: [GNU Genel Kamu Lisansı sürüm 3](LICENSE) veya (tercihinize göre) daha sonraki
bir sürümün koşulları altında yeniden dağıtabilir ve/veya değiştirebilirsiniz (`GPL-3.0-or-later`).

USB Nexus is free software, licensed under the GNU General Public License v3.0 or later.
See [LICENSE](LICENSE).
