# Frontend (Angular 16)

## ⚠️ Work In Progress ⚠️

> Angular team deprecated karma and started integration with jest as experimental

> Some tests may show wrong warnings/errors

## Production deploy

```bash
docker-compose up -d --build
```

## Local development

> Requirements

- **Node v20.9.0 (npm)**
- **Python 3.12.0 (poetry)**

> Setup project

```bash
curl -sSL https://raw.githubusercontent.com/tednaaa/moner/main/scripts/setup.sh | bash -
```

> Start development servers

> Parallel Frontend + Backend

```bash
npm start
```

### Commands (Frontend)

> Run linter

```bash
npm run lint
```

> Run unit tests `(Jest + Testing Library)`

```bash
npm test
```

> Run e2e tests `(Cypress)`

```bash
npm run test:e2e
```

### Commands (Backend)

> Run linter

```bash
make lint
```

> Run tests

```bash
make test
```
