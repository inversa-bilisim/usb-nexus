# SPDX-License-Identifier: GPL-3.0-or-later
# USB Nexus user interface strings — Russian.

## Command line help

app-about = Безопасный USB-over-IP: делитесь USB-устройствами по сети с шифрованием, сопряжением и автоматическим переподключением.
arg-lang = Язык интерфейса (например, en, tr). По умолчанию — язык системы.
arg-state-dir = Папка для ключей и сопряжённых компьютеров.
arg-name = Имя этого компьютера, видимое другим.
arg-verbose = Показывать подробные сообщения журнала.

cmd-daemon-about = Запустить службу USB Nexus (используется настольным приложением).
arg-allow-all-users = Разрешить управлять службой всем локальным пользователям (по умолчанию — только членам группы usbnexus).
arg-socket = Путь к локальному управляющему сокету службы.
cmd-service-about = Установить или удалить службу USB Nexus для Windows.
cmd-install-about = Установить службу, запустить её сейчас и при каждой загрузке (запустите от имени администратора).
cmd-uninstall-about = Остановить и удалить службу.
cmd-serve-about = Предоставить общий доступ к USB-устройствам этого компьютера.
arg-export = Bus ID устройства для общего доступа (можно указывать несколько раз). См.: usbnexus local
arg-listen = Адрес и порт для прослушивания.
arg-pair = Открыть окно сопряжения при запуске и показать PIN-код.
arg-no-mdns = Не анонсировать этот сервер в локальной сети.

cmd-pin-about = Показать PIN-код для сопряжения нового компьютера с работающим сервером.
arg-seconds = Сколько секунд PIN-код остаётся действительным.

cmd-local-about = Показать список USB-устройств, подключённых к этому компьютеру.
cmd-discover-about = Найти серверы USB Nexus в локальной сети.
arg-timeout = Продолжительность поиска, в секундах.
cmd-pair-about = Выполнить сопряжение с сервером по PIN-коду, который он показывает.
arg-server = Сервер: сопряжённое имя, отпечаток или адрес[:порт].
arg-pin = PIN-код, показанный сервером (если не указан, будет запрошен).
cmd-list-about = Показать список устройств, к которым сервер предоставляет общий доступ.
cmd-allow-user-about = Разрешить пользователю управлять службой из настольного приложения (добавляет его в группу usbnexus).
arg-user = Имя пользователя.
allow-user-invalid = «{ $user }» не является допустимым именем пользователя.
allow-user-failed = Не удалось добавить «{ $user }» в группу usbnexus.
allow-user-done = «{ $user }» теперь может управлять службой USB Nexus.
allow-user-relogin = Изменение вступит в силу после повторного входа в систему.
cmd-attach-about = Использовать удалённое устройство на этом компьютере; переподключается автоматически.
arg-busid = Bus ID удалённого устройства.
cmd-peers-about = Показать список сопряжённых компьютеров.
cmd-forget-about = Удалить сопряжённый компьютер.
arg-peer = Имя или отпечаток сопряжённого компьютера.

## General

error-prefix = Ошибка: { $detail }
hint-root = Для этой операции требуются права администратора. Повторите попытку с sudo.
unsupported-os = Эта команда пока не поддерживается в данной операционной системе.
yes = да
no = нет

service-installed = Служба USB Nexus установлена и запущена.
service-removed = Служба USB Nexus удалена.

## Server

serve-started = Сервер «{ $name }» слушает на { $addr }.
serve-fingerprint = Отпечаток: { $fp }
serve-exporting = Устройства с общим доступом:
serve-no-exports = Нет устройств с общим доступом. Добавьте --export BUSID (список устройств: usbnexus local).
serve-pairing-pin = PIN-код сопряжения: { $pin } (действителен { $seconds } секунд)
serve-paired = Выполнено сопряжение с «{ $name }».
serve-pairing-failed = Неудачная попытка сопряжения с адреса { $addr }.
serve-exported = { $busid } теперь используется компьютером «{ $client }».
serve-released = { $busid } освобождён компьютером «{ $client }».
serve-stopping = Остановка; устройства возвращаются в обычные драйверы…
serve-mdns-failed = Обнаружение в локальной сети недоступно: { $detail }
serve-bind-failed = Не удалось подготовить { $busid } для общего доступа: { $detail }

## Pairing

pin-show = PIN-код сопряжения: { $pin }
pin-hint = На другом компьютере выполните команду «usbnexus pair { $name }» в течение { $seconds } секунд.
pin-no-server = Работающий сервер USB Nexus не найден. Запустите его командой: usbnexus serve
pair-enter-pin = Введите PIN-код, показанный на сервере:
pair-ok = Выполнено сопряжение с «{ $name }» ({ $fp }).
pair-already = Уже сопряжено с «{ $name }».

## Devices

local-header = USB-устройства на этом компьютере:
local-empty = USB-устройства не найдены.
list-header = Устройства с общим доступом от «{ $name }»:
list-empty = Этот сервер не предоставляет общий доступ ни к одному устройству.
list-in-use = используется
col-busid = BUS ID
col-id = VID:PID
col-speed = СКОРОСТЬ
col-product = УСТРОЙСТВО
col-driver = ДРАЙВЕР
col-state = СОСТОЯНИЕ
col-name = ИМЯ
col-fingerprint = ОТПЕЧАТОК
col-address = АДРЕС

## Discovery and peers

discover-searching = Поиск в локальной сети…
discover-none = Серверы USB Nexus не найдены.
discover-paired = сопряжён
peers-empty = Пока нет сопряжённых компьютеров.
forget-ok = «{ $name }» удалён.
forget-unknown = Нет сопряжённого компьютера, соответствующего «{ $peer }».

## Attaching

attach-connecting = Подключение к { $addr }…
attach-attached = { $busid } подключён (виртуальный порт { $port }).
attach-disconnected = Соединение потеряно: { $reason }
attach-retrying = Повторное подключение через { $seconds } секунд…
attach-detached = Устройство отключено.
attach-stop-hint = Нажмите Ctrl+C, чтобы отключить устройство.

## Errors

err-pairing-required = «{ $name }» ещё не сопряжён с этим компьютером. Выполните: usbnexus pair { $target }
err-not-found = Сервер не найден в сети.
err-version = На сервере работает несовместимая версия USB Nexus.
err-not-trusted = Этот компьютер не сопряжён с сервером.
err-pairing-closed = Сопряжение на сервере не открыто. На сервере выполните: usbnexus pin
err-pairing-failed = Сопряжение не выполнено. Проверьте PIN-код и повторите попытку.
err-no-such-device = Сервер не предоставляет общий доступ к этому устройству.
err-device-busy = Устройство используется другим компьютером.
err-internal = Сервер сообщил о внутренней ошибке.
err-protocol = Неожиданный ответ от сервера.
err-connection-lost = Соединение с компьютером потеряно.
err-unsupported = Это пока не поддерживается в данной операционной системе.
err-driver-missing = Драйвер usbip-win2 не установлен. Запустите установку USB Nexus ещё раз и разрешите установку usbip-win2.
err-driver-outdated = Установленный драйвер usbip-win2 слишком старый. Запустите установку USB Nexus ещё раз и разрешите обновление usbip-win2.
err-vboxusb-missing = Драйверы VirtualBox USB, необходимые для общего доступа, не установлены. Переустановите USB Nexus.
err-device-in-use-by-os = Операционная система использует это устройство собственным драйвером, поэтому предоставить к нему общий доступ с этого компьютера нельзя.
err-unreachable = Не удалось связаться с компьютером. Убедитесь, что он включён и подключён к сети.
err-cancelled = Действие отменено.
err-permission-denied = У USB Nexus недостаточно прав.
err-invalid = Запрос не распознан.
err-other = Что-то пошло не так: { $detail }

## Help layout

help-usage = Использование:
help-arguments = Аргументы
help-options = Параметры
help-commands = Команды
arg-help = Показать справку.
arg-version = Показать версию.

## Peers

peers-servers = Сопряжённые серверы (устройства этих компьютеров может использовать этот компьютер):
peers-clients = Сопряжённые клиенты (могут использовать устройства этого компьютера):

## Desktop app

gui-nav-this-computer = Этот компьютер
gui-nav-network = Компьютеры в сети
gui-nav-connected = Подключённые устройства
gui-nav-paired = Сопряжённые компьютеры
gui-language = Язык
gui-this-title = Устройства на этом компьютере
gui-this-subtitle = Выберите, какие устройства могут использовать другие компьютеры.
gui-share = Предоставить доступ
gui-shared = Доступ предоставлен
gui-not-shared = Доступ не предоставлен
gui-used-by = Используется компьютером { $name }
gui-no-local-devices = На этом компьютере USB-устройства не найдены.
gui-unnamed-device = USB-устройство
gui-pair-new = Сопрячь новый компьютер
gui-pin-title = PIN-код сопряжения
gui-pin-body = Введите этот PIN-код на другом компьютере.
gui-pin-remaining = Действителен ещё { $seconds } секунд
gui-pin-expired = Срок действия PIN-кода истёк.
gui-pin-new = Новый PIN-код
gui-pin-stop = Остановить сопряжение
gui-close = Закрыть
gui-cancel = Отмена
gui-network-title = Компьютеры в сети
gui-network-subtitle = Компьютеры с USB Nexus, найденные в вашей локальной сети.
gui-refresh = Обновить
gui-searching = Поиск в сети…
gui-none-found = Компьютеры не найдены. Убедитесь, что USB Nexus запущен на другом компьютере, либо добавьте его по адресу.
gui-add-by-address = Добавить по адресу
gui-address = Адрес
gui-address-hint = например, 192.168.1.20
gui-pair = Сопрячь
gui-paired = Сопряжён
gui-pair-title = Сопряжение с { $name }
gui-pair-body = На компьютере { $name } выберите «Сопрячь новый компьютер» и введите показанный там PIN-код.
gui-pin = PIN-код
gui-pair-done = Выполнено сопряжение с { $name }.
gui-devices-of = Устройства с общим доступом от { $name }
gui-no-remote-devices = Этот компьютер не предоставляет общий доступ ни к одному устройству.
gui-connect = Подключить
gui-disconnect = Отключить
gui-in-use-elsewhere = Используется другим компьютером
gui-connected-here = Подключено к этому компьютеру
gui-back = Назад
gui-connected-title = Устройства, подключённые к этому компьютеру
gui-connected-subtitle = Удалённые устройства остаются подключёнными и переподключаются самостоятельно при сбое сети.
gui-connected-empty = Нет подключённых удалённых устройств. Откройте раздел «Сеть», чтобы подключить устройство.
gui-on-computer = на { $name }
gui-state-connecting = Подключение…
gui-state-attached = Подключено
gui-state-retrying = Переподключение через { $seconds } с
gui-state-stopped = Отключено
gui-state-failed = Ошибка
gui-reconnect = Переподключить
gui-remove = Удалить
gui-paired-title = Сопряжённые компьютеры
gui-paired-subtitle = Компьютеры сопрягаются один раз с помощью PIN-кода, после чего распознаются автоматически.
gui-paired-servers = Компьютеры, устройства которых вы можете использовать
gui-paired-clients = Компьютеры, которые могут использовать ваши устройства
gui-paired-empty = Пока нет.
gui-remove-confirm = Удалить { $name }? Чтобы снова использовать его, потребуется повторное сопряжение.
gui-fingerprint = Отпечаток
gui-service-denied-title = Этому пользователю не разрешено управлять службой USB Nexus
gui-service-denied-body = Управлять могут только администраторы и члены группы usbnexus. Разрешите доступ этому пользователю (потребуется пароль администратора) либо выполните от имени администратора:
gui-service-denied-command = Команда:
gui-service-denied-user = ПОЛЬЗОВАТЕЛЬ
gui-grant-access = Разрешить этому пользователю
gui-grant-access-done = Доступ предоставлен.
gui-setup-kernel-modules = Отсутствуют модули ядра, необходимые для USB over IP: { $modules }. Установите их; служба загрузит их самостоятельно:
gui-setup-kernel-modules-nocmd = Они входят в состав ядра большинства дистрибутивов (Ubuntu: linux-modules-extra, Fedora: kernel-modules-extra).
gui-service-down-title = Служба USB Nexus не запущена
gui-service-down-body = Запустите службу; это окно подключится к ней автоматически.
gui-service-down-linux = В Linux выполните:
gui-service-down-windows = В Windows выполните от имени администратора:
gui-service-down-macos = В macOS выполните:
gui-retry = Повторить
gui-details = Подробности
err-service-unavailable = Не удалось связаться со службой USB Nexus.

## Web interface

gui-web-login-title = Вход в { $name }
gui-web-password = Пароль
gui-web-sign-in = Войти
gui-web-sign-out = Выйти
gui-web-wrong-password = Неверный пароль.
gui-web-locked = Слишком много попыток. Повторите через { $seconds } секунд.
cmd-web-about = Включить или отключить веб-интерфейс.
cmd-enable-about = Включить веб-интерфейс (при первом запуске запрашивает пароль).
cmd-disable-about = Отключить веб-интерфейс.
cmd-password-about = Изменить пароль веб-интерфейса.
cmd-status-about = Показать, включён ли веб-интерфейс и по какому адресу доступен.
arg-lan = Разрешить доступ с других компьютеров сети.
arg-port = TCP-порт веб-интерфейса.
web-on = Веб-интерфейс включён:
web-off = Веб-интерфейс отключён.
web-local-only = Открыть его может только этот компьютер. Используйте --lan, чтобы разрешить другим компьютерам.
web-fingerprint = Браузер предупредит о сертификате; его отпечаток должен быть таким: { $fp }
web-trusted = Браузеры на этом компьютере доверяют сертификату; другие компьютеры будут предупреждены. Отпечаток сертификата: { $fp }
web-not-running = Веб-интерфейс включён, но не запустился: { $detail }
web-password-prompt = Новый пароль веб-интерфейса:
web-password-repeat = Повторите пароль:
web-password-mismatch = Пароли не совпадают.
web-password-set = Пароль веб-интерфейса изменён.
err-weak-password = Пароль должен содержать не менее 8 символов.
err-port-in-use = Этот порт используется другой программой. Выберите другой порт.
err-password-required = Сначала задайте пароль веб-интерфейса: usbnexus web password
err-forbidden = Это можно изменить только на самом компьютере.
err-not-logged-in = Пожалуйста, войдите ещё раз.

## Hotplug, access control and usage log

arg-device = Устройство: идентификатор или Bus ID (см.: usbnexus list СЕРВЕР).
cmd-policy-about = Показать или выбрать, кто может использовать устройства с общим доступом.
arg-policy = open: любой сопряжённый компьютер может использовать любое устройство с общим доступом; restricted: только компьютеры, разрешённые для конкретного устройства.
arg-no-server = Не настраивать общий доступ к USB-устройствам этого компьютера.
arg-no-client = Не настраивать использование USB-устройств других компьютеров.
arg-web = Веб-интерфейс: off (отключён), local (только этот компьютер) или network (вся сеть).
arg-web-port = Порт веб-интерфейса (по умолчанию 3242).
arg-web-password-file = Файл с новым паролем веб-интерфейса.
cmd-history-about = Показать журнал использования устройств этого компьютера с общим доступом.
arg-csv = Вывести все записи в формате CSV (например, для сохранения в файл).
arg-limit = Количество записей для вывода.
col-device-id = ИДЕНТИФИКАТОР УСТРОЙСТВА
col-time = ВРЕМЯ (UTC)
col-event = СОБЫТИЕ
col-computer = КОМПЬЮТЕР
col-duration = ДЛИТЕЛЬНОСТЬ
state-unplugged = не подключено
state-no-permission = нет разрешения
serve-denied = «{ $client }» не разрешено использовать { $busid }.
attach-waiting-device = Устройство не подключено к серверу; ожидание…
attach-queued = Устройство используется другим компьютером; этот компьютер — { $position }-й в очереди.
policy-open = Любой сопряжённый компьютер может использовать любое устройство с общим доступом (открытый режим).
policy-restricted = Сопряжённые компьютеры могут использовать только те устройства, к которым им разрешён доступ (ограниченный режим).
history-header = Журнал использования (записи хранятся { $days } дней):
history-empty = Журнал использования пуст.
history-paired = сопряжение
history-pairing-failed = неверный PIN-код
history-attached = начал использование
history-detached = завершил использование
history-denied = отказано (нет разрешения)
err-access-denied = Этому компьютеру не разрешено использовать устройство.

gui-nav-history = История
gui-nav-settings = Настройки
gui-not-plugged-in = Не подключено
gui-tracked-by-port = отслеживается по порту
gui-tracked-by-port-hint = У этого устройства нет серийного номера, поэтому оно определяется по USB-порту, в который оно подключено. Подключите его в тот же порт снова.
gui-no-permission = Нет разрешения
gui-no-permission-hint = Владелец того компьютера не разрешил этому компьютеру использовать устройство.
gui-connect-when-plugged-in = Подключается автоматически, как только устройство подключено.
gui-state-waiting-device = Ожидание устройства
gui-state-queued = Используется в другом месте; { $position }-й в очереди
gui-save = Сохранить
gui-saved = Сохранено.
gui-skip = Пропустить
gui-access-title = Кто может использовать это устройство
gui-access-everyone = Все сопряжённые компьютеры
gui-access-some = Выбранные компьютеры ({ $count })
gui-access-nobody = Пока ни одного компьютера
gui-access-mode-default = Использовать настройку по умолчанию
gui-access-default-open = Сейчас: все сопряжённые компьютеры.
gui-access-default-restricted = Сейчас: только компьютеры, выбранные ниже.
gui-access-mode-open = Все сопряжённые компьютеры
gui-access-mode-open-body = Использовать может любой компьютер, сопряжённый с этим.
gui-access-mode-selected = Только выбранные компьютеры
gui-access-mode-selected-body = Использовать могут только отмеченные ниже компьютеры.
gui-access-computers = Компьютеры, которым разрешено использовать устройство
gui-access-revoke-note = Компьютер, потерявший разрешение, немедленно отключается от устройства.
gui-client-devices = Устройства
gui-client-devices-title = Устройства, которые может использовать { $name }
gui-client-devices-body = Отметьте устройства с общим доступом, которые может использовать этот компьютер.
gui-client-devices-after-pairing = Устройства с общим доступом могут использовать только разрешённые компьютеры. Отметьте устройства, которые может использовать этот компьютер; если пропустить этот шаг, он пока не сможет использовать ни одного.
gui-no-shared-devices = Этот компьютер пока не предоставляет общий доступ ни к одному устройству.
gui-roles-title = Как используется этот компьютер
gui-roles-body = Экраны не выбранного варианта использования скрыты. При добавлении варианта устанавливается всё необходимое.
gui-role-server = Использовать как сервер (предоставлять общий доступ к USB-устройствам этого компьютера)
gui-role-client = Использовать как клиент (использовать USB-устройства других компьютеров)
gui-roles-client-note = При необходимости будет установлен драйвер usbip-win2; USB-устройства на несколько секунд перестанут работать, и может потребоваться перезагрузка Windows.
gui-roles-applying = Применение…
gui-reboot-required = Перезагрузите компьютер, чтобы завершить настройку.
gui-used-by-waiting = { $name } использует · ожидают: { $count }
gui-col-permissions = Разрешения
gui-col-status = Состояние
gui-in-use-title = Использует
gui-nobody-using = Сейчас устройство никто не использует.
gui-since = с { $time }
gui-disconnect-user = Отключить
gui-disconnected-note = { $name } отключён. Если он продолжает запрашивать устройство, он переподключится в течение нескольких секунд; чтобы отключить его навсегда, снимите отметку в списке слева и сохраните.
gui-queue-title = Очередь ({ $count })
gui-queue-empty = Никто не ждёт.
gui-badge-using = использует
gui-badge-queued = в очереди ({ $position })
gui-handover-title = Автоматическая передача
gui-handover-default-on = По умолчанию (включено, { $seconds } с)
gui-handover-default-off = По умолчанию (отключено)
gui-handover-on = Включено
gui-handover-off = Отключено
gui-handover-before = Если другой компьютер ждёт, через
gui-handover-after = секунд без использования устройство переходит к следующему.
gui-kind-storage = Накопитель
gui-kind-input = Клавиатура / мышь
gui-kind-printer = Принтер
gui-kind-dongle = Аппаратный ключ лицензии
gui-kind-other = Другое устройство
gui-web-title = Веб-интерфейс
gui-web-body = Управляйте этим компьютером из браузера, с паролем.
gui-web-confirm-off = Отключить веб-интерфейс? Эта страница перестанет работать. Его можно снова включить в настольном приложении или командой «usbnexus web enable» на самом компьютере.
gui-web-confirm-local = Разрешить доступ только с этого компьютера? Эта страница была открыта из сети и перестанет работать.
gui-web-turned-off = Веб-интерфейс отключён. Его можно снова включить в настольном приложении или командой «usbnexus web enable» на самом компьютере.
gui-web-enabled = Веб-интерфейс включён
gui-web-access-local = Только этот компьютер
gui-web-access-network = Вся сеть
gui-web-port = Порт
gui-web-new-password = Новый пароль
gui-web-repeat-password = Новый пароль (ещё раз)
gui-web-password-keep = Не менее 8 символов. Оставьте пустым, чтобы сохранить текущий пароль.
gui-web-password-required = Не менее 8 символов.
gui-web-mismatch = Пароли не совпадают.
gui-web-open-at = Адрес:
gui-web-fingerprint = Браузер предупреждает о сертификате; его отпечаток: { $fp }.
gui-web-trusted = Браузеры на этом компьютере доверяют сертификату; другие компьютеры будут предупреждены. Отпечаток сертификата: { $fp }.
gui-policy-title = Кто может использовать устройства с общим доступом
gui-policy-body = Компьютеры всегда должны быть сначала сопряжены. Это настройка по умолчанию для каждого устройства с общим доступом; для каждого устройства её можно изменить отдельно.
gui-policy-first-title = Кто может использовать ваши устройства с общим доступом?
gui-policy-first-body = Компьютеры всегда должны сначала пройти сопряжение по PIN-коду. Выберите, что могут делать сопряжённые компьютеры:
gui-policy-later = Это можно изменить позже в настройках.
gui-policy-open = Любой сопряжённый компьютер
gui-policy-open-body = Любой сопряжённый компьютер может использовать любое устройство с общим доступом.
gui-policy-restricted = Только разрешённые компьютеры
gui-policy-restricted-body = Вы сами выбираете для каждого устройства, каким сопряжённым компьютерам разрешено его использовать.
gui-retention-title = Журнал использования
gui-retention-body = Записываются сопряжения, неверные PIN-коды, использование устройств и отклонённые запросы. Старые записи удаляются автоматически.
gui-retention-days = Хранить записи (дней)
gui-settings-title = Настройки
gui-startup-title = Автозапуск
gui-startup-body = При закрытии окна USB Nexus остаётся в области уведомлений; общий доступ и подключения всё равно поддерживает служба.
gui-startup-enabled = Запускать автоматически при входе в систему
gui-tray-open = Открыть USB Nexus
gui-tray-quit = Выход
gui-history-title = История
gui-history-subtitle = Кто и когда использовал устройства этого компьютера. Записи хранятся { $days } дней.
gui-history-empty = Пока ничего не записано.
gui-history-export = Экспорт в CSV
gui-history-time = Время
gui-history-event = Событие
gui-history-computer = Компьютер
gui-history-device = Устройство
gui-history-device-id = Идентификатор устройства
gui-history-duration = Длительность
gui-history-paired = Сопряжение
gui-history-pairing-failed = Неверный PIN-код
gui-history-attached = Начал использование
gui-history-detached = Завершил использование
gui-history-denied = Отказано: нет разрешения

## Windows installer (setup-*): generated into packaging/windows/strings.nsh;
## plain text only (no { $variables }).
setup-roles-title = Как вы будете использовать USB Nexus?
setup-roles-subtitle = Выберите, для чего будет служить этот компьютер.
setup-role-server = Использовать как сервер
setup-role-client = Использовать как клиент
setup-role-web = Веб-доступ
setup-usbip-install-note = Также будет установлен драйвер usbip-win2. Во время установки USB-устройства на несколько секунд перестанут работать, после чего потребуется перезагрузка Windows.
setup-usbip-update-note = Установленный драйвер usbip-win2 слишком старый и будет обновлён. Во время установки USB-устройства на несколько секунд перестанут работать, после чего потребуется перезагрузка Windows.
setup-usbip-present-note = Драйвер usbip-win2 уже установлен на этом компьютере.
setup-usbip-failed = Не удалось установить драйвер usbip-win2. Вы можете повторить попытку позже, запустив установку USB Nexus снова.
setup-service-failed = Не удалось настроить службу USB Nexus. Подробности — в журнале установки.
setup-web-title = Веб-интерфейс
setup-web-subtitle = Настройки для управления этим компьютером из браузера.
setup-web-access = Доступ:
setup-web-local = Только этот компьютер
setup-web-network = Вся сеть
setup-web-port = Порт:
setup-web-port-free = ✓ Порт свободен
setup-web-port-busy = ✗ Этот порт занят другой программой
setup-web-port-invalid = ✗ Введите число от 1 до 65535
setup-web-password = Пароль:
setup-web-password-repeat = Пароль (ещё раз):
setup-web-password-hint = Не менее 8 символов.
setup-web-password-keep = Не менее 8 символов. Оставьте пустым, чтобы сохранить текущий пароль.
