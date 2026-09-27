# Linux

## Paketler

| Paket | İçerik |
|---|---|
| `usbnexus` | `/usr/bin/usbnexus`, systemd hizmeti, `usbnexus` grubu. Ekransız sunucular için tek başına yeterli. |
| `usb-nexus` | Masaüstü uygulaması (`usbnexus-desktop`); `usbnexus` paketine bağımlıdır. |

Kurulum hizmeti etkinleştirip başlatır:

```sh
sudo apt install ./usbnexus_*.deb ./usb-nexus_*.deb      # veya: sudo dnf install ./usbnexus-*.rpm ./USB*.rpm
```

- **Kullanıcı izni:** Masaüstü uygulaması hizmeti yalnızca `usbnexus` grubundaki kullanıcılar için
  yönetebilir. Uygulama ilk açılışta izin yoksa **“Bu kullanıcıya izin ver”** düğmesi gösterir (yönetici
  parolası sorar, hemen geçerli olur). Elle: `sudo usbnexus allow-user KULLANICI`.
- **Çekirdek modülleri:** `usbip-host` ve `vhci-hcd` çoğu dağıtımda çekirdekle gelir; hizmet onları
  kendisi yükler. Eksiklerse (Ubuntu bulut çekirdekleri: `linux-modules-extra-$(uname -r)`; Fedora/RHEL:
  `kernel-modules-extra`, RPM paketi bunu önerilen bağımlılık olarak kurar) uygulama ve web arayüzü
  kurulacak paketi gösterir.

## Paket üretme

```sh
cargo install cargo-deb cargo-generate-rpm --locked
cargo install tauri-cli --version "^2" --locked

cargo build --release -p usbnexus-cli
cargo deb -p usbnexus-cli --no-build                 # target/debian/usbnexus_*.deb
cargo generate-rpm -p crates/usbnexus-cli            # target/generate-rpm/usbnexus-*.rpm
(cd apps/desktop/src-tauri && cargo tauri build --config ../../../packaging/linux/tauri.bundle.json)
                                                     # target/release/bundle/{deb,rpm}/
```

## Elle kurulum (paketsiz)

```sh
sudo install -m 755 target/release/usbnexus /usr/bin/usbnexus
sudo install -m 644 packaging/linux/usbnexus.service /usr/lib/systemd/system/usbnexus.service
sudo install -m 644 packaging/linux/usbnexus.sysusers /usr/lib/sysusers.d/usbnexus.conf
sudo systemd-sysusers && sudo systemctl daemon-reload && sudo systemctl enable --now usbnexus
```

- Durum: `systemctl status usbnexus`, günlük: `journalctl -u usbnexus -f`
- Ayarlar ve anahtarlar: `/var/lib/usbnexus`
- Denetim soketi: `/run/usbnexus/daemon.sock` (yalnızca `root` ve `usbnexus` grubu)
- Paket kaldırılırken (`purge`) ayarlar da silinir.
