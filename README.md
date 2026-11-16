# Spoty

Оконный клиент Spotify для Windows на Rust. Локальное воспроизведение работает через Spotify Soloist, официальный фоновый компонент Spotify. После первого сопряжения обычное приложение Spotify можно закрыть.

Правая панель **Now playing** показывает обложку, название трека, исполнителя и альбом. Открывай и закрывай её круглой кнопкой с треугольником в нижнем плеере. Чтобы изменить ширину, перетащи левую границу панели.

Под данными трека находится текст песни. Если у текста есть таймкоды, Spoty подсвечивает текущую строку. Нажатие на строку перематывает трек, а кнопка в углу карточки разворачивает текст на весь экран.

Установщик включает запуск Spoty в области уведомлений при входе в Windows. Изменить это можно в **Settings → Start with your computer**. Следующие обновления сохраняют выбранный режим.

## Установка и первый запуск

1. Скачай `Spoty-Setup-x64.exe` из [Releases](https://github.com/Maks45-ui/Spoty/releases). Для загрузки нужен доступ к приватному репозиторию. Тестовый установщик без цифровой подписи.
2. Полностью закрой предыдущий Spoty через меню в области уведомлений. Запусти установщик и открой Spoty из меню «Пуск».
3. Если Spoty попросит войти, введи Client ID приложения из [Spotify Developer Dashboard](https://developer.spotify.com/dashboard). Подойдёт прежний Client ID с Redirect URI `http://127.0.0.1:8989/login`.
4. Нажми **Set up Soloist** или открой **Settings → Spotify Soloist**. Получи личный [Soloist API Key](https://developer.spotify.com/dashboard/soloist) и введи его в Spoty. Этот ключ отличается от Client ID. Spoty скачает Soloist напрямую с серверов Spotify.
5. Для первого сопряжения запусти трек в обычном Spotify под тем же аккаунтом и выбери в списке устройств **Spoty Soloist (…)**. Когда Spoty подтвердит подключение, нажми **Start listening**. После этого можно закрыть обычный Spotify и выбирать треки в Spoty.

Используй Soloist для локального воспроизведения. Встроенный альтернативный движок librespot может выдавать [ошибку аудиоключа](https://github.com/librespot-org/librespot/issues/1649) на некоторых аккаунтах Premium.

## Сборка

На Windows нужны Rust через rustup и Visual Studio Build Tools с компонентами C++ и Windows SDK.

```powershell
cargo build --locked --release --no-default-features --target x86_64-pc-windows-msvc --bin spoty
```

Установщик собирается с Inno Setup 6. GitHub Actions проверяет установку, запуск с `--version` и удаление, затем добавляет установщик в предварительный релиз. Бинарник Soloist в установщик не входит.

## Исходный проект

Эта версия основана на [Spotifast](https://github.com/crmne/spotifast/tree/5286363), интеграция Soloist которого сохранена. [Лицензия MIT](LICENSE) и [документация исходного проекта](README.upstream.md) сохранены.
