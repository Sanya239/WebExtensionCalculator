# Calculator frontend

Minimal React calculator UI with a keyboard-driven expression input and a scrollable calculation history.

## Development

Install dependencies:

```bash
npm install
```

Run with the local mock API:

```bash
npm run dev:mock
```

Run with the server API:

```bash
npm run dev:server
```

The server API is the default for `npm run dev` and `npm run build`.

The API mode and base URL can also be configured in `.env.local`:

```env
VITE_API_MODE=server
VITE_API_URL=http://127.0.0.1:3000
```

## Checks

```bash
npm run lint
npm run build:server
```

Use `npm run build:mock` to build an extension backed by the mock API.
