# Web Extension Calculator

Калькулятор состоит из Rust-бекенда, PostgreSQL и React-интерфейса, который можно запускать как обычную веб-страницу или Chromium-расширение.

## Требования

- Rust и Cargo;
- Node.js и npm;
- PostgreSQL.

## 1. Запуск базы данных

Создайте базу `calculator` в локальном PostgreSQL. Например, PostgreSQL можно запустить в Docker:

```bash
docker run --name calculator-postgres \
  --restart unless-stopped \
  -e POSTGRES_USER=calculator \
  -e POSTGRES_PASSWORD=calculator \
  -e POSTGRES_DB=calculator \
  -p 127.0.0.1:5432:5432 \
  -v calculator-postgres-data:/var/lib/postgresql/data \
  -d postgres:17-alpine
```

Если контейнер уже создан, достаточно выполнить `docker start calculator-postgres`.

Скопируйте пример конфигурации бекенда и при необходимости исправьте строку подключения:

```bash
cp backend/.env.example .env
```

## 2. Запуск бекенда

Из корня репозитория выполните:

```bash
cargo run -p calculator-backend
```

Бекенд применит миграции при старте и будет доступен по адресу `http://127.0.0.1:3000`. Фронтенд использует:

- `POST /api/calculate`;
- `GET /api/history`.

## 3. Запуск фронтенда в браузере

В другом терминале выполните:

```bash
cd frontend/calculator
npm ci
npm run dev:server
```

Адрес страницы Vite появится в терминале. Для разработки без бекенда используйте `npm run dev:mock`.

Адрес API и режим можно изменить в локальном файле `frontend/calculator/.env.local`:

```dotenv
VITE_API_MODE=server
VITE_API_URL=http://127.0.0.1:3000
```

## 4. Сборка Chromium-расширения

```bash
cd frontend/calculator
npm ci
npm run build:server
```

После сборки откройте `chrome://extensions` (или `chromium://extensions`), включите режим разработчика, нажмите **Load unpacked / Загрузить распакованное расширение** и выберите каталог `frontend/calculator/dist`.

После новой сборки нажмите кнопку перезагрузки расширения на странице расширений. Обычная команда `npm run build` также собирает версию с серверным API; для mock-версии есть `npm run build:mock`.

`device_id` создаётся автоматически один раз для установки расширения и хранится в `chrome.storage.local`. В веб-режиме используется `localStorage`.
