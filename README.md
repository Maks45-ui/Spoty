# Spoty

Неофициальный оконный клиент Spotify для Windows на Rust. Интерфейс построен на egui с OpenGL, звук воспроизводит librespot. В клиенте есть поиск, сохранённая музыка, плейлисты, очередь и медиаклавиши. Интерфейс приложения на английском.

## Тестовая сборка для Windows 10 и 11 x64

1. Откройте [Releases](https://github.com/Maks45-ui/Spoty/releases) и скачайте `Spoty-Setup-x64.exe`. Для доступа к релизу нужен доступ к приватному репозиторию.
2. Запустите установщик и откройте Spoty из меню «Пуск». Сборка не подписана. Для установки в профиль пользователя права администратора не нужны.
3. Войдите в [Spotify Developer Dashboard](https://developer.spotify.com/dashboard) с аккаунтом Spotify Premium. Создайте приложение, добавьте Redirect URI `http://127.0.0.1:8989/login` и выберите Web API. Скопируйте Client ID в Spoty, затем подтвердите вход на странице Spotify. Client Secret не нужен.

Подробнее о настройке и типичных ошибках можно прочитать в [руководстве исходного проекта](docs/_guide/make-it-even-faster.md).

## Сборка

На Windows установите Rust через rustup и Visual Studio Build Tools с компонентами C++ и Windows SDK.

```powershell
cargo build --locked --release --target x86_64-pc-windows-msvc --bin spoty
```

Программа появится в `target/x86_64-pc-windows-msvc/release/spoty.exe`. Для создания установщика используйте Inno Setup 6 и передайте абсолютные пути к бинарнику и папке назначения.

```powershell
ISCC.exe "/DBinary=$PWD\target\x86_64-pc-windows-msvc\release\spoty.exe" "/DOutputDir=$PWD\dist" packaging\windows\spoty.iss
```

GitHub Actions собирает установщик, проверяет установку и удаление и добавляет установщик и SHA-256 в предварительный релиз для тегов `spoty-v*`. Новые версии устанавливаются вручную из Releases.

## Исходный проект

Spoty основан на [Spotsie v0.1.8](https://github.com/ahaan-shah/spotsie/tree/v0.1.8), производном от Spotifast. Авторы и условия использования указаны в [лицензии MIT](LICENSE). [Исходная документация](README.upstream.md) сохранена.
