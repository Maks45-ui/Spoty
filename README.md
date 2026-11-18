<p align="center">
  <img src="packaging/icons/spotifast.svg" width="72" alt="">
</p>

<h1 align="center">Spoty</h1>

<p align="center">Клиент Spotify для Windows на Rust</p>

<p align="center">
  <a href="https://github.com/Maks45-ui/Spoty/releases/latest/download/Spoty-Setup-x64.exe"><strong>Скачать для Windows x64</strong></a>
  · <a href="https://github.com/Maks45-ui/Spoty/releases">Все версии</a>
  · <a href="https://github.com/Maks45-ui/Spoty/issues">Сообщить об ошибке</a>
</p>

Spoty работает в собственном окне и воспроизводит звук на компьютере через официальный компонент [Spotify Soloist](https://developer.spotify.com/documentation/soloist). Обычный Spotify нужен для первого сопряжения. Затем треки можно запускать из Spoty.

## Возможности

- Поиск треков, альбомов и исполнителей, доступ к библиотеке, плейлистам и очереди.
- Правая панель **Now playing** с обложкой, исполнителем и текстом песни. При наличии таймкодов текущая строка подсвечивается, а нажатие на строку перематывает трек.
- Изменяемая ширина панелей и переключатель **Settings → Appearance → Separate panels**.
- Медиаклавиши и запуск в области уведомлений. Автозапуск можно изменить в **Settings → Start with your computer**.

## Установка

Нужны **Windows 10 или 11 x64**, **Spotify Premium**, личный **Client ID** и личный **Soloist API Key**. Установщик без цифровой подписи.

1. Скачай [Spoty-Setup-x64.exe](https://github.com/Maks45-ui/Spoty/releases/latest/download/Spoty-Setup-x64.exe) и запусти его.
2. При первом входе создай приложение с **Web API** в [Spotify Developer Dashboard](https://developer.spotify.com/dashboard). Укажи Redirect URI `http://127.0.0.1:8989/login` и введи его **Client ID** в Spoty.
3. В Spoty выбери **Set up Soloist** либо открой **Settings → Spotify Soloist**. Создай [Soloist API Key](https://developer.spotify.com/dashboard/soloist) и введи его в Spoty. Приложение скачает компонент напрямую с серверов Spotify.
4. Один раз запусти трек в обычном Spotify с тем же аккаунтом. В списке устройств выбери **Spoty Soloist (…)**. Когда в Spoty появится кнопка **Start listening**, нажми её и закрой обычный Spotify.

### Обновление

Полностью выйди из Spoty через **Quit** в меню значка рядом с часами и запусти новый установщик. При ошибке **Access is denied** заверши процесс **Spoty** в Диспетчере задач и выбери **Try again** в установщике. Файл `spoty.exe` нужно заменить, чтобы обновление вступило в силу.

Тексты доступны для тех треков, для которых их нашёл источник. Для воспроизведения на этом ПК используй **Spoty Soloist**, если встроенный движок сообщает [об ошибке аудиоключа](https://github.com/librespot-org/librespot/issues/1649).

### Ссылки Spotify из Discord

Начиная со Spoty 0.2.6, ссылки `open.spotify.com` можно открывать через расширение **Open in Spoty**. Установи его один раз в Edge или Chrome, который Windows использует для открытия ссылок:

1. Открой страницу расширений (`edge://extensions` или `chrome://extensions`).
2. Включи **Режим разработчика**, нажми **Загрузить распакованное расширение** и выбери папку `%LOCALAPPDATA%\Programs\Spoty\browser-extension`. Если Spoty установлен в другую папку, выбери её подпапку `browser-extension`.
3. Нажми ссылку Spotify в Discord. Трек начнёт играть в Spoty, если вход и воспроизведение уже настроены. Ссылки на альбомы, плейлисты и исполнителей откроют их страницы.

Расширение передаёт тип ссылки и идентификатор Spotify напрямую приложению на этом ПК. На промежуточной странице можно открыть ссылку в веб-плеере.

## Сборка из исходников

На Windows нужны Rust через rustup и Visual Studio Build Tools с компонентами C++ и Windows SDK.

```powershell
cargo build --locked --release --no-default-features --target x86_64-pc-windows-msvc --bins
```

Для сборки установщика используй Inno Setup 6 и [скрипт упаковки](packaging/windows/spoty.iss), передав пути к `spoty.exe` и `spoty-browser.exe` как `/DBinary` и `/DBrowserBinary`. [GitHub Actions](https://github.com/Maks45-ui/Spoty/actions/workflows/spoty-windows.yml) проверяет установку, автозапуск, регистрацию расширения и удаление программы.

## Происхождение проекта

Spoty основан на [Spotifast](https://github.com/crmne/spotifast/tree/5286363). Смотри [лицензию MIT](LICENSE) и [документацию исходного проекта](README.upstream.md).
