# Web Extension Calculator

Калькулятор с Rust-бекендом, PostgreSQL и интерфейсом на React. Фронтенд работает как обычная веб-страница и как Chromium-расширение.

## Возможности

- вычисление выражений с `+`, `-`, `*`, `/` и скобками;
- ввод с клавиатуры и отправка по `Enter`;
- история вычислений с результатом и временем;
- повторная загрузка выражения и результата по клику на историю;
- сохранение состояния и отдельной истории для каждого устройства;
- вычисление выделенного на странице текста через контекстное меню или `Ctrl+Shift+Y`;
- mock-режим для разработки без бекенда.

## Запуск

Технологии Rust, Node.js, npm и Docker.

Сборка и запуск локальной базы данных PostgreSQL из корня репозитория:

```bash
docker build -f backend/Dockerfile -t calculator-postgres-image backend
docker run --name calculator-postgres \
  --restart unless-stopped \
  -p 127.0.0.1:5432:5432 \
  -v calculator-postgres-data:/var/lib/postgresql/data \
  -d calculator-postgres-image
```

Если контейнер уже создан: `docker start calculator-postgres`.

Запуск бекенда:

```bash
cp backend/.env.example .env
cargo run -p calculator-backend
```

Сервер будет доступен на `http://127.0.0.1:3000`.

Запуск веб-интерфейса:
```bash
cd frontend/calculator
npm ci
npm run dev:server
```

## Chromium-расширение

Сборка:
```bash
cd frontend/calculator
npm ci
npm run build:server
```

Подключение к браузеру:
Откройте `chrome://extensions`, включите режим разработчика, выберите **Загрузить распакованное расширение** и укажите каталог `frontend/calculator/dist`.

После повторной сборки нажмите кнопку перезагрузки расширения на странице `chrome://extensions`.
