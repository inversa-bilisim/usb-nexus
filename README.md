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
cargo test --workspace
```

## Lisans

USB Nexus özgür yazılımdır: [GNU Genel Kamu Lisansı sürüm 3](LICENSE) veya (tercihinize göre) daha sonraki
bir sürümün koşulları altında yeniden dağıtabilir ve/veya değiştirebilirsiniz (`GPL-3.0-or-later`).

USB Nexus is free software, licensed under the GNU General Public License v3.0 or later.
See [LICENSE](LICENSE).
