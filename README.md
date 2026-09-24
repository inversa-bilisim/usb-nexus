# Demli

Güvenli, kolay kurulan USB-over-IP çözümü (geliştirme aşamasında).

Hedefler:
- TLS şifreleme ve sertifika tabanlı kimlik doğrulama, PIN ile eşleştirme
- Ağdaki sunucuları otomatik bulma (mDNS)
- Bağlantı koptuğunda otomatik yeniden bağlanma
- Grafik arayüz (Tauri) ve paketli kurulum

## Yapı

| Crate | Açıklama |
|---|---|
| `crates/demli-proto` | USB/IP kablo protokolü (spesifikasyondan sıfırdan yazıldı) |
| `crates/demli-core` | Güvenli taşıma, eşleştirme, bulma, yeniden bağlanma (yapım aşamasında) |
| `crates/demli-cli` | `demli` komut satırı aracı ve servis (yapım aşamasında) |

## Derleme

```sh
cargo test --workspace
```

## Lisans

Demli özgür yazılımdır: [GNU Genel Kamu Lisansı sürüm 3](LICENSE) veya (tercihinize göre) daha sonraki
bir sürümün koşulları altında yeniden dağıtabilir ve/veya değiştirebilirsiniz (`GPL-3.0-or-later`).

Demli is free software, licensed under the GNU General Public License v3.0 or later.
See [LICENSE](LICENSE).
