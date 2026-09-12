# Calculator Backend

## Требования

- Rust stable (1.85+)
- Docker (или локальный PostgreSQL)

## Запуск

1. Скопируйте пример окружения:
   cp .env.example .env

2. Поднимите PostgreSQL:
   docker compose up -d

3. Запустите сервер:
   cargo run -p calculator-backend

4. Проверьте:
   curl http://127.0.0.1:3000/health
   # => ok

## Ручки

- `POST /api/calculate` — тело: `{"expression": "...", "device_id": "..."}`
- `GET /api/history?limit=10&device_id=...`