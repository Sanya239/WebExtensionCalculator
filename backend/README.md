# Calculator Backend

## Требования

- Rust stable (1.85+)
- Docker (или локальный PostgreSQL)

## Запуск

1. Из корня репозитория скопируйте пример окружения:

   ```bash
   cp backend/.env.example .env
   ```

2. Поднимите PostgreSQL:

   ```bash
   docker build -f backend/Dockerfile -t calculator-postgres-image backend

   docker run --name calculator-postgres \
     --restart unless-stopped \
     -p 127.0.0.1:5432:5432 \
     -v calculator-postgres-data:/var/lib/postgresql/data \
     -d calculator-postgres-image
   ```

   Если контейнер уже создан, выполните `docker start calculator-postgres`.

3. Запустите сервер:

   ```bash
   cargo run -p calculator-backend
   ```

4. Проверьте:

   ```bash
   curl http://127.0.0.1:3000/health
   # => ok
   ```

## Ручки

- `POST /api/calculate` — тело: `{"expression": "...", "device_id": "..."}`
- `GET /api/history?limit=10&device_id=...`
