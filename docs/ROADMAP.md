# Roadmap

This roadmap reflects the current repository state and the original phased build plan.

## Phase 1: Project Scaffold and CI Pipeline

Status: complete or mostly complete.

- Tauri v2 app scaffold.
- SolidJS frontend.
- TypeScript and Vite setup.
- Biome formatting setup.
- Frontend test setup.
- Rust project setup.

Recommended next step:

- Add public CI workflows for frontend tests, Rust tests, formatting, and build checks.

## Phase 2: Webview Engine and Browser Chrome

Status: complete or mostly complete.

- Tab bar.
- URL bar.
- Native child webviews.
- Active/inactive tab visibility.
- Navigation commands.
- Back, forward, and reload commands.
- Compatibility fallback mode.

Recommended next step:

- Add tests around tab lifecycle, active tab switching, fallback mode, and bounds updates.

## Phase 3: Hybrid Brain and Model Router

Status: complete prototype.

- AI settings page.
- Local Ollama config.
- OpenRouter key flow.
- Free model discovery.
- Premium model selection scaffolding.
- AI request test.
- Intent-tier scoring.

Recommended next step:

- Clarify provider naming, error states, and storage guarantees.
- Add tests for config updates and intent-tier scoring.

## Phase 4: Intent Layer and Command Bar

Status: planned.

Goals:

- Natural-language command bar.
- Intent parsing.
- User confirmation model for risky actions.
- Mapping between intent results and browser operations.
- Safe failure states.

## Phase 5: Memory Engine and Security Layer

Status: planned.

Goals:

- Local memory store.
- User-controlled retention.
- Permission model for remembered context.
- Browser data boundaries.
- CSP hardening.
- Better secret storage.
- Security documentation and tests.

## Phase 6: Background Agents and Macro Recorder

Status: planned.

Goals:

- Background task queue.
- Agent status UI.
- User approval checkpoints.
- Macro recording.
- Macro replay safeguards.
- Task cancellation and audit logs.

## Phase 7: Design System, Plugin SDK, and OSS Release

Status: planned.

Goals:

- Stable design tokens.
- Component polish.
- Plugin API design.
- Extension or plugin examples.
- Public documentation.
- Public issue templates.
- Release workflow.
- Signed binaries if distribution is planned.

## Suggested Open-Source Milestone

Before the first public announcement:

- Replace placeholder package metadata.
- Add license and maintainer information.
- Add CI.
- Add a security contact.
- Tighten Tauri CSP and permissions.
- Add contribution guidelines.
- Add issue and PR templates.
- Add tests around browser and AI-router behavior.
- Remove stale references to React or Zustand; the app uses SolidJS and Solid stores.
