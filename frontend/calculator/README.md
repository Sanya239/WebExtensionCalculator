# Calculator frontend

Minimal React calculator UI with a keyboard-driven expression input and a scrollable calculation history.

## Development

Install dependencies:

```bash
npm install
```

Run with the local mock API (also the default for `npm run dev`):

```bash
npm run dev:mock
```

Run with the server API:

```bash
npm run dev:server
```

The API mode and base URL can also be configured in `.env.local`:

```env
VITE_API_MODE=server
VITE_API_URL=http://localhost:8080
```

## Checks

```bash
npm run lint
npm run build
```
