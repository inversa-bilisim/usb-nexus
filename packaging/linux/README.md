# Linux kurulumu (elle)

Paketler hazır olana kadar elle kurulum:

```sh
cargo build --release -p usbnexus-cli -p usbnexus-desktop
sudo install -m 755 target/release/usbnexus /usr/bin/usbnexus
sudo install -m 755 target/release/usbnexus-desktop /usr/bin/usbnexus-desktop
sudo install -m 644 packaging/linux/usbnexus.service /usr/lib/systemd/system/usbnexus.service
sudo install -m 644 packaging/linux/usbnexus.sysusers /usr/lib/sysusers.d/usbnexus.conf
sudo systemd-sysusers
sudo usermod -aG usbnexus "$USER"     # masaüstü uygulamasının hizmeti yönetebilmesi için
sudo systemctl daemon-reload
sudo systemctl enable --now usbnexus
```

Grup üyeliğinin geçerli olması için oturumu kapatıp yeniden açın, ardından `usbnexus-desktop` uygulamasını başlatın.

- Durum: `systemctl status usbnexus`, günlük: `journalctl -u usbnexus -f`
- Ayarlar ve anahtarlar: `/var/lib/usbnexus`
- Denetim soketi: `/run/usbnexus/daemon.sock` (yalnızca `root` ve `usbnexus` grubu)
