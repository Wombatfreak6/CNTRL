# Open-Source Release Checklist

Use this checklist before making the private repository public.

## Repository Metadata

- Rename `package.json` package from `tauri-app` to a project-specific name such as `vibe-browser`.
- Add a real package description.
- Confirm `package.json` license is `MIT` or update docs if another license is preferred.
- Update `src-tauri/Cargo.toml` authors from placeholder values.
- Confirm the Tauri identifier in `src-tauri/tauri.conf.json`.
- Add repository URL, homepage, and issue tracker metadata where appropriate.

## Documentation

- Replace the existing README with the open-source README.
- Add `CONTRIBUTING.md`.
- Add `SECURITY.md` with a real contact.
- Add `CODE_OF_CONDUCT.md` with a real maintainer contact.
- Add `LICENSE`.
- Add `docs/ARCHITECTURE.md`.
- Add `docs/ROADMAP.md`.
- Add issue templates.
- Add a pull request template.

## Security

- Search for secrets before publishing:

```bash
rg -n "sk-|api[_-]?key|secret|token|password|Authorization|Bearer"
```

- Confirm no real provider keys are committed.
- Confirm no app-data files are committed.
- Review `src-tauri/tauri.conf.json` CSP settings.
- Review Tauri capabilities in `src-tauri/capabilities/default.json`.
- Document fallback iframe security behavior.
- Add dependency audits to CI.

## Build and Test

- Ensure the app runs with `npm run tauri dev`.
- Ensure frontend builds with `npm run build`.
- Ensure Tauri builds with `npm run tauri build`.
- Ensure frontend tests pass with `npx vitest run`.
- Ensure Rust tests pass with `cargo test` from `src-tauri`.
- Add or update npm scripts for tests and formatting.

Suggested package scripts:

```json
{
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "serve": "vite preview",
    "tauri": "tauri",
    "test": "vitest run",
    "format": "biome format . --write",
    "format:check": "biome check ."
  }
}
```

## GitHub Community Files

- `.github/ISSUE_TEMPLATE/bug_report.md`
- `.github/ISSUE_TEMPLATE/feature_request.md`
- `.github/PULL_REQUEST_TEMPLATE.md`
- Optional: `.github/workflows/ci.yml`

## Maintainer Decisions

- Decide whether binaries will be published.
- Decide whether AI-provider integrations are experimental or supported.
- Decide whether autonomous browsing features need an explicit safety policy.
- Decide whether plugin SDK work should wait until the browser security model is hardened.
