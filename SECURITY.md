# Security Policy

CNTRL Browser is an experimental desktop browser and AI-assisted browsing project. Treat all browser, webview, fallback rendering, credentials, model-provider, and automation code as security-sensitive.

## Supported Versions

The project is pre-1.0. Security fixes will target the latest `main` branch unless maintainers publish a versioned support policy.

## Reporting a Vulnerability

Please do not disclose vulnerabilities publicly before maintainers have had time to investigate.

When reporting a vulnerability, include:

- A concise summary.
- Affected commit, branch, or release.
- Steps to reproduce.
- Expected and actual behavior.
- Logs, screenshots, or proof-of-concept details when safe to share.
- Whether the issue exposes credentials, local files, browsing data, model-provider keys, or arbitrary code execution.

Preferred private reporting channel:

```text
security contact pending
```

Before publishing the repository, replace the line above with a real email address, GitHub private vulnerability reporting instructions, or another monitored security channel.

## Security-Sensitive Areas

Pay extra attention to:

- Tauri capabilities and permissions.
- Child webview creation and bounds updates.
- Native webview navigation.
- Compatibility fallback HTML fetching and iframe rendering.
- CSP configuration.
- `dangerousDisableAssetCspModification` in `tauri.conf.json`.
- API key storage and masking.
- Calls to OpenRouter, Ollama, Hugging Face, or other model providers.
- Future autonomous browsing actions and command execution.
- Future memory and macro-recorder features.

## Current Hardening Checklist

Before the first public release:

- Revisit `src-tauri/tauri.conf.json` and replace `csp: null` with a deliberate CSP.
- Reevaluate `dangerousDisableAssetCspModification`.
- Document the fallback iframe sandbox model and its limits.
- Add tests around fallback rendering and unsafe markup.
- Define how provider keys are stored per operating system.
- Prefer OS keychain-backed storage for secrets before production release.
- Add dependency audit workflow for npm and Cargo dependencies.
- Add CI for frontend tests, Rust tests, formatting, and build checks.
- Add a real security contact.

## Scope

Reports are in scope when they affect:

- The desktop app.
- Browser tab isolation.
- Webview navigation.
- Fallback renderer behavior.
- AI-provider credentials.
- Local model or cloud model routing.
- Local user files or app data.
- Future autonomous browsing actions.

Reports are out of scope when they only affect:

- Third-party sites loaded inside the browser, unless CNTRL Browser makes the issue worse.
- Provider-side behavior in OpenRouter, Ollama, Hugging Face, or other external APIs.
- Social engineering against maintainers.
