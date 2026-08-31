# Nx Polyglot

This is a demo repository for running JavaScript, TypeScript, and Rust projects in one [Nx](https://nx.dev) monorepo.

It combines a [TanStack Start](https://tanstack.com/start/latest) dashboard with a [Topcoat](https://github.com/tokio-rs/topcoat) Rust application. Both applications have their own packages, tests, builds, and development workflows. Nx brings them into one project graph and coordinates their tasks locally and in CI.

pnpm manages the JavaScript packages, Cargo manages the Rust crates, and Nx provides the shared task orchestration and caching layer.

## Video and blog post

<a href="https://youtu.be/lyHbNUdLQhI">
  <img src="https://img.youtube.com/vi/lyHbNUdLQhI/maxresdefault.jpg" alt="Polyglot Nx Monorepo with Rust and TanStack" width="600">
</a>

- Video: [Polyglot Nx Monorepo with Rust and TanStack](https://youtu.be/lyHbNUdLQhI) (9:40), a walkthrough of the project graph, the shared types generated from Rust, cross-language caching, and distributed CI on Nx Cloud.
- Blog post: [Exploring Polyglot Monorepos with Nx, TanStack and Rust](https://nx.dev/blog/polyglot-nx-monorepo-rust-tanstack/)

## What's in the repository

| Area                                                                       | Description                                                                                       |
| -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| [`apps/web`](./apps/web)                                                   | TanStack Start operations dashboard using TanStack Charts                                         |
| [`apps/topcoat-security`](./apps/topcoat-security)                         | Topcoat security incident tracker with server-rendered UI, JSON API, and Server-Sent Events       |
| [`packages/web`](./packages/web)                                           | Dashboard data, React UI, and the client for the Topcoat API                                      |
| [`packages/topcoat`](./packages/topcoat)                                   | Rust domain, store, and UI crates, plus the TypeScript globe bundled into the Topcoat application |
| [`packages/shared/security-contract`](./packages/shared/security-contract) | Shared Rust schema and generated TypeScript types                                                 |

The security service continuously generates incidents in memory. The standalone Topcoat application renders them as a security operations center. The TanStack application reads the same data through a server function and displays it with TanStack Charts.

## Run the demo

The workspace expects Node 24, pnpm 11, and the Rust toolchain configured in [`.mise.toml`](./.mise.toml).

```sh
mise install
pnpm install
pnpm exec nx run web:dev
```

The `web:dev` target starts the complete development environment:

```text
web:dev
  -> topcoat-security:run (continuous)
       -> security-globe:vite:build
```

Nx builds the TypeScript globe, starts Topcoat on [localhost:3000](http://localhost:3000), then starts TanStack Start on [localhost:4200](http://localhost:4200). Stopping the Nx command stops the process group.

You can also run the Rust application by itself:

```sh
pnpm exec nx run topcoat-security:run
```

## How Nx orchestrates JavaScript and Rust

Every application and package is represented as an Nx project. Each project exposes targets such as `build`, `test`, `lint`, `typecheck`, `run`, or `dev`.

For JavaScript projects, Nx plugins infer many targets from package scripts and tool configuration. Vite, TypeScript, ESLint, and Vitest targets are visible in the project graph, with their existing configuration as the source of truth.

Rust projects define targets with the [`@monodon/rust`](https://github.com/Cammisuli/monodon) executors. A Rust library target looks like this:

```json
{
  "build": {
    "executor": "@monodon/rust:check",
    "outputs": ["{options.target-dir}"],
    "options": {
      "target-dir": "dist/target/topcoat-security-domain/build"
    }
  }
}
```

Nx schedules the target and handles its dependencies and cache. The executor invokes the matching Cargo operation. The same pattern maps Nx targets to `cargo build`, `cargo test`, `cargo clippy`, and `cargo run`.

The shared configuration in [`nx.json`](./nx.json) adds Rust-specific cache inputs, including the Cargo manifests, lockfile, toolchain, and dependency sources. Separate target directories keep build, test, lint, and run artifacts isolated.

You can run the main targets across the whole workspace with one command:

```sh
pnpm exec nx run-many -t lint test build typecheck
```

Nx determines which projects provide each target and invokes the appropriate JavaScript or Rust toolchain.

## Task pipelines and cross-language dependencies

The main pipeline examples are:

- [`apps/web/package.json`](./apps/web/package.json) defines the continuous `web:dev` target and its dependency on the Rust server.
- [`apps/topcoat-security/project.json`](./apps/topcoat-security/project.json) defines Rust build, test, lint, and run targets. Its build and run pipelines depend on the TypeScript globe bundle.
- [`packages/shared/security-contract/project.json`](./packages/shared/security-contract/project.json) generates a JSON Schema from Rust types and converts it into TypeScript types.
- [`nx.json`](./nx.json) configures shared target dependencies, cache inputs, and the Monodon plugin.

The explicit globe dependency is important because it crosses toolchains. The Topcoat application serves a JavaScript artifact produced by Vite, so Nx needs an explicit task edge:

```json
{
  "dependsOn": [
    "^build",
    { "projects": ["security-globe"], "target": "vite:build" }
  ]
}
```

The `^build` entry builds project dependencies first. The project-specific entry builds the globe before the Rust application.

Explore the complete graph with:

```sh
pnpm exec nx graph
```

## CI and distributed execution

The GitHub Actions workflow is defined in [`.github/workflows/ci.yml`](./.github/workflows/ci.yml). It:

1. Installs Node, pnpm, and Rust through mise.
2. Calculates the affected Git revisions with `nx-set-shas`.
3. Checks Nx sync state and formatting.
4. Runs affected `lint`, `test`, `build`, and `typecheck` targets.
5. Uses Nx Cloud to distribute those tasks across two polyglot agents.

The custom Nx Cloud launch template is in [`.nx/workflows/agents.yaml`](./.nx/workflows/agents.yaml). Each agent has both JavaScript and Rust toolchains, restores pnpm and Cargo caches, installs dependencies, and fetches the locked Cargo dependencies before executing tasks.

```sh
pnpm exec nx affected -t lint test build typecheck
```

Nx uses the project graph to select affected projects, restores valid cached results, and distributes the remaining JavaScript and Rust work through the same pipeline.

This demo is connected to the Nx Cloud staging environment so the repository can also exercise custom launch templates and distributed execution.

## Useful commands

```sh
# List every project
pnpm exec nx show projects

# Inspect resolved targets, including inferred targets
pnpm exec nx show project topcoat-security
pnpm exec nx show project web

# Generate the shared Rust to TypeScript contract
pnpm exec nx run security-contract:generate

# Check TypeScript project-reference synchronization
pnpm exec nx sync:check

# Run the main verification targets
pnpm exec nx run-many -t lint test build typecheck
```

The repository favors small, visible examples of project boundaries, cache configuration, task dependencies, continuous tasks, and CI distribution. It is intended as a reference for developers exploring how Nx can coordinate multiple languages while each ecosystem keeps its native tooling.
