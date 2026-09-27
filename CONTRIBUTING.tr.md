# Katkıda bulunma

[English](CONTRIBUTING.md) · **Türkçe**

Katkılarınızı bekliyoruz!

1. Bir issue açarak ne yapmak istediğinizi anlatın (küçük düzeltmeler için gerekmez).
2. Değişikliğinizi ayrı bir dalda yapın; `cargo fmt`, `cargo clippy --workspace` ve
   `cargo test --workspace` komutlarının temiz geçtiğinden emin olun (denetimlerin tam listesi
   `CLAUDE.md` içinde).
3. Pull request açın.

## Lisans ve kaynak kod kuralları

- Tüm katkılar projenin lisansı olan **GPL-3.0-or-later** altında kabul edilir.
- Yeni kaynak dosyalarının başına SPDX başlığını ekleyin:
  ```rust
  // SPDX-License-Identifier: GPL-3.0-or-later
  // Copyright (C) 2026 USB Nexus contributors
  ```
- Kaynak dosyalarındaki yorumlar ve belgeler İngilizcedir. Kullanıcıya görünen her metin, her dil için
  `locales/*.ftl` dosyalarına yazılır; aksi halde testler başarısız olur.
- Başka projelerden kod alıyorsanız, lisansının GPL-3.0 ile uyumlu olduğundan emin olun ve kaynağını,
  özgün telif satırlarını koruyarak belirtin. **GPL-2.0-only** lisanslı kod (ör. Linux çekirdeği)
  GPL-3.0 ile birleştirilemez; bu tür kodlar yalnızca referans olarak okunabilir, kopyalanamaz.
- Üçüncü taraf ikili dosyalar (ör. Windows sürücüleri) `packaging/` altında, lisans metinleri ve
  kaynak kod erişim bilgisiyle birlikte tutulur.
