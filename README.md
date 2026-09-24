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

Telif hakkı © Demli geliştiricileri. Tüm hakları saklıdır. Lisans modeli henüz belirlenmedi.
