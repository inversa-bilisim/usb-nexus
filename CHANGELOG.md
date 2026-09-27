# Changelog

The section for a version tag becomes the text of its GitHub release
(`release.yml` copies it into the draft). Keep the English part first and
the Turkish part after the `---` line of each version.

## 0.1.1

Second pre-release. Tested on two Windows 11 computers, including moving a
shared USB stick between ports while another computer uses it. Linux and
macOS packages are built but not yet tried on real hardware.

- **Hotplug:** a device that is unplugged and plugged in again (also on
  another port) comes back to the computer using it by itself, without
  errors. The server waits for the operating system to finish setting up a
  freshly plugged-in device, ends the session of an unplugged device at
  once, and the client waits briefly before attaching again.
- **Languages:** German, Spanish, French, Italian, Portuguese (Brazil),
  Russian, Japanese and Simplified Chinese, in the app, the web interface,
  the command line and the installer (English and Turkish as before).
- **Web interface:** its own settings (on/off, this computer or the whole
  network, port, password) can now be changed from the web interface too.
  It asks before a change that would cut the current page off, follows a
  port change, and keeps you signed in. A port used by another program is
  refused before anything is saved.
- **Desktop app:** the history CSV export opens a save dialog.
- **Diagnostics:** `usbnexus log debug` makes the service log in detail;
  `usbnexus log info` returns to normal.

Known limitations: the installer is not code-signed (Windows SmartScreen
shows "More info → Run anyway"); the installer itself is in English for
languages it does not have.

---

İkinci ön sürüm. İki Windows 11 bilgisayarda denendi; paylaşılan bir USB
bellek başka bir bilgisayar kullanırken porttan porta taşındı. Linux ve
macOS paketleri derleniyor ama gerçek donanımda henüz denenmedi.

- **Çıkar-tak:** Çıkarılıp yeniden takılan (başka porta da olsa) cihaz, onu
  kullanan bilgisayara hatasız geri geliyor. Sunucu yeni takılan cihazın
  işletim sistemince kurulmasını bekliyor, çıkarılan cihazın oturumunu
  hemen kapatıyor, istemci yeniden bağlanmadan önce kısa süre bekliyor.
- **Diller:** Almanca, İspanyolca, Fransızca, İtalyanca, Portekizce
  (Brezilya), Rusça, Japonca ve Basitleştirilmiş Çince; uygulamada, web
  arayüzünde, komut satırında ve kurulum sihirbazında (İngilizce ve Türkçe
  eskisi gibi).
- **Web arayüzü:** Kendi ayarları (açık/kapalı, yalnızca bu bilgisayar ya
  da tüm ağ, port, parola) artık web arayüzünden de değiştirilebiliyor.
  Sayfayı erişilemez kılacak değişiklikten önce soruyor, port değişince
  yeni adrese gidiyor, oturum açık kalıyor. Başka programın kullandığı
  port kaydedilmeden reddediliyor.
- **Masaüstü uygulaması:** Geçmiş CSV dışa aktarma kaydetme penceresi açıyor.
- **Sorun giderme:** `usbnexus log debug` hizmetin ayrıntılı günlük
  tutmasını sağlar; `usbnexus log info` normale döndürür.

Bilinen sınırlar: kurulum dosyası kod imzalı değil (Windows SmartScreen'de
"Daha fazla bilgi → Yine de çalıştır"); kurulum sihirbazının bilmediği
dillerde sihirbaz İngilizce.

## 0.1.0

First pre-release: USB over IP inside mutually authenticated TLS with PIN
pairing, automatic discovery and reconnection; server and client roles;
device details, waiting queue and automatic handover; access control and
usage log; web interface; Windows installer with role selection and the
bundled usbip-win2 driver; Linux deb/rpm and macOS pkg packages; English
and Turkish interface.

---

İlk ön sürüm: karşılıklı doğrulamalı TLS içinde USB over IP, PIN ile
eşleştirme, otomatik bulma ve yeniden bağlanma; sunucu ve istemci rolleri;
cihaz ayrıntıları, bekleme sırası ve otomatik devir; erişim denetimi ve
kullanım kaydı; web arayüzü; rol seçimli ve usbip-win2 sürücüsü gömülü
Windows kurulumu; Linux deb/rpm ve macOS pkg paketleri; Türkçe ve
İngilizce arayüz.
