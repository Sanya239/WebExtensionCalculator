# Calculator Backend

## Требования

- Rust stable (1.85+)
- Docker Compose

## Запуск

Из корня репозитория запустите PostgreSQL и бекенд:

```bash
docker compose up --build
```

Бекенд будет доступен на `http://127.0.0.1:3000`. Переменные окружения для
контейнеров задаются в `docker-compose.yml`

## Ручки

- `POST /api/calculate` — тело: `{"expression": "...", "device_id": "..."}`
- `GET /api/history?limit=10&device_id=...`
