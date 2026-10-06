# wstack CLI pilot

## Baseline

Use this Git checkout and `./project`. `./project doctor --json` checks pilot prerequisites. The recipe copies tracked known-good/invalid maps into a new run-owned directory, invokes the actual built CLI and removes fixture state. No app instance, network, model or provider is needed.

## Conventions

Feature IDs come from indexed Markdown filenames. Use `./project info --json --base <base-ref>` to identify the candidate and comparison. Preserve command stdout/stderr, exit codes and report after cleanup. Evidence defaults to ignored `.evidence/`; export useful delivery reports durably. Structural validity alone is not behavior proof.

## Features

- [Feature map validation](./feature-map.md) Valid and invalid disposable repositories, text and JSON CLI entrypoints.

Only this bounded feature is mapped. Suite lint, synchronization and skill listing have automated tests but no live feature recipe in this pilot.
