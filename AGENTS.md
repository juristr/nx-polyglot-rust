<!-- nx configuration start-->
<!-- Leave the start & end comments to automatically receive updates. -->

# General Guidelines for working with Nx

- For navigating/exploring the workspace, invoke the `nx-workspace` skill first - it has patterns for querying projects, targets, and dependencies
- When running tasks (for example build, lint, test, e2e, etc.), always prefer running the task through `nx` (i.e. `nx run`, `nx run-many`, `nx affected`) instead of using the underlying tooling directly
- Prefix nx commands with the workspace's package manager (e.g., `pnpm nx build`, `npm exec nx test`) - avoids using globally installed CLI
- You have access to the Nx MCP server and its tools, use them to help the user
- For Nx plugin best practices, check `node_modules/@nx/<plugin>/PLUGIN.md`. Not all plugins have this file - proceed without it if unavailable.
- NEVER guess CLI flags - always check nx_docs or `--help` first when unsure

## Scaffolding & Generators

- For scaffolding tasks (creating apps, libs, project structure, setup), ALWAYS invoke the `nx-generate` skill FIRST before exploring or calling MCP tools

## When to use nx_docs

- USE for: advanced config options, unfamiliar flags, migration guides, plugin configuration, edge cases
- DON'T USE for: basic generator syntax (`nx g @nx/react:app`), standard commands, things you already know
- The `nx-generate` skill handles generator discovery internally - don't call nx_docs just to look up generator syntax

<!-- nx configuration end-->

# Polyglot workspace rules

This repository runs JavaScript/TypeScript (pnpm) and Rust (Cargo) under one Nx graph.

## Before changing configuration

Analyze the workspace itself rather than assuming: read `nx.json` (named inputs, `targetDefaults`), the affected `project.json`/`package.json`/`Cargo.toml`, and use the Nx MCP tools to inspect the project graph and the resolved task configuration. If a `docs/` folder is present, read it first, it takes precedence over inference.

## Hard rules

1. **Nx infers dependencies within an ecosystem, never across ecosystems.** Every cross-language relationship is a hand-declared edge: a codegen target with explicit inputs, a `dependsOn` on another project's target, or both. Never assume the graph picked it up.
2. **Never weaken the graph to make a build pass.** If deleting a TypeScript project reference makes `nx sync` stop complaining, the reference is not the problem. Fix the task configuration around an accurate graph.
3. **Every Rust target gets its own Cargo `target-dir`, declared as that target's output.** A shared target directory lets a `cargo clippy` cache entry be restored as if it were a `cargo build` result, silently.
4. **Every code generation target declares all generated files as outputs, and the upstream sources of the other language as explicit inputs.** Otherwise a cache hit restores a half-generated tree, or skips regeneration of stale types.
5. **Cross-language artifact edges go on every target that needs the artifact, `build` and `run` alike.** Missing it on `run` passes CI and breaks the dev server. Runtime dependencies belong on the specific target that needs them, not as an `implicitDependency`.
6. **Long-running targets are `"continuous": true`.** Nx waits for a task to exit before treating it as complete, and a server never exits.
7. **Inferred package-script targets carry no cache metadata.** Declare `cache`, `inputs` (including `externalDependencies` for tools you shell out to), and `outputs` explicitly.
8. **Organize folders by domain and ownership, never by language.** Language goes in tags (`lang:rust`, `lang:typescript`). Some packages span both.
9. **Definition of done, always:** `pnpm exec nx reset`, then `pnpm exec nx run-many -t lint test build typecheck` (twice, the second run all cache hits), then `pnpm exec nx sync:check` and `pnpm exec nx format:check`, then actually load the application in a browser. Reporting completion without that full run is a failure, and a narrow check is how a broken graph gets shipped.
