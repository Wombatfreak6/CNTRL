# Contributing to CNTRL Browser

Thanks for your interest in CNTRL Browser. This project is early, experimental, and security-sensitive because it combines web browsing, embedded webviews, and AI model integrations. Contributions are welcome, but changes should be scoped, tested, and easy to review.

## Development Setup

Install dependencies:

```bash
npm install
```

Run the desktop app:

```bash
npm run tauri dev
```

Run the frontend dev server only:

```bash
npm run dev
```

Run frontend tests:

```bash
npx vitest run
```

Run Rust tests:

```bash
cd src-tauri
cargo test
```

Format code:

```bash
npx biome format . --write
```

## Branching

Use short, descriptive branch names:

```text
feature/command-bar
fix/fallback-timeout
docs/open-source-readme
security/csp-hardening
```

## Commit Style

Use conventional commits:

```text
feat: add command bar shell
fix: hide inactive child webviews
docs: document tauri prerequisites
test: add ai router tier scoring tests
chore: update tooling config
```

## Pull Request Guidelines

Before opening a pull request:

- Keep the scope focused.
- Include a clear summary of behavior changes.
- Add or update tests for browser behavior, AI routing, or security-sensitive code.
- Mention any manual testing performed.
- Avoid committing generated app data, API keys, screenshots with private data, or local build artifacts.

For UI changes, include:

- What changed visually.
- Which browser states were checked.
- Any accessibility considerations.

For Tauri or Rust backend changes, include:

- Which commands or services changed.
- Whether capabilities or permissions changed.
- How errors are surfaced to the frontend.

For AI-provider changes, include:

- Which provider or API endpoint changed.
- How credentials are stored or read.
- Failure behavior when provider calls fail.

## Code Style

- Prefer SolidJS primitives and `solid-js/store` for frontend state.
- Keep Tauri commands thin; place business logic in Rust services.
- Return clear errors from backend commands.
- Avoid broad capability permissions unless a feature requires them.
- Keep docs aligned with actual implementation status.

## Security Expectations

Do not open public pull requests that include:

- API keys.
- Local app data.
- OAuth tokens.
- Browser cookies.
- User browsing history.
- Credentials or private model-provider configuration.

Security fixes should be handled carefully. See [SECURITY.md](SECURITY.md).
