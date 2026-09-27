# Milestone 1, Lesson 1: Cargo, Project Isolation, and IntelliJ IDEA

## Purpose

This first exercise builds the foundation for every later Rust exercise: using **Cargo** to define a project, manage packages, compile code, and run it. We use **IntelliJ IDEA with the Rust plugin** as the editor and debugger, but Cargo remains the build system underneath IDEA.

By the end of the lesson, you should be able to explain why a Rust project does not need a Python-style virtual environment and distinguish `cargo check`, `cargo build`, and `cargo run`.

## Cargo favors conventions

Rust and Cargo favor a standard, predictable project layout over extensive per-project build configuration. Once Cargo finds a `Cargo.toml`, it already knows the conventional locations and purposes of many files:

```text
Cargo.toml       # package manifest
src/main.rs      # default binary entry point
src/lib.rs       # default library entry point
tests/           # integration tests
examples/        # runnable examples
benches/         # benchmarks
target/          # generated build output
```

Therefore a minimal executable needs almost no configuration. Given the generated `Cargo.toml` and `src/main.rs`, `cargo run` can infer that it should:

1. read the package manifest;
2. compile the default binary source at `src/main.rs`;
3. place debug output under `target/debug/`;
4. reuse cached compilation data from the conventional `target/` subdirectories; and
5. run the resulting executable.

Cargo may create standard output directories such as `target/debug/build/` and `target/debug/examples/` even before your package has a build script or example. This gives every Cargo build stable locations for those artifact categories and allows Cargo, IDEs, and other tools to behave consistently without project-specific setup.

These conventions make unfamiliar Rust projects easier to navigate and let IntelliJ Rust, CI pipelines, `cargo fmt`, `cargo clippy`, and `cargo test` work with little configuration. When customization is genuinely necessary, Cargo supports it—for example, additional binaries can be declared with `[[bin]]` entries in `Cargo.toml`—but the conventional layout covers most applications.

---

## The project boundary is `Cargo.toml`

Cargo uses a manifest named `Cargo.toml` in the project root. It identifies the package and declares its direct dependencies.

```text
m01-cargo/
├── Cargo.toml        # package metadata and declared dependencies
├── Cargo.lock        # exact resolved dependency versions
├── src/
│   └── main.rs       # executable entry point
└── target/           # generated local build output
```

For comparison:

| Concern | Python | Rust |
|---|---|---|
| Dependency declaration | `pyproject.toml` / `requirements.txt` | `Cargo.toml` |
| Exact resolved dependency graph | often manually captured with `pip freeze` | `Cargo.lock`, maintained by Cargo |
| Environment isolation | `.venv`, activated in a shell | the project directory and its manifest |
| Local build artifacts | often outside the virtual environment | `target/` inside the project |

### No activation step

Rust does **not** use a project-local virtual environment. You do not run `source .venv/bin/activate`.

Instead, when you run a Cargo command from a project directory, Cargo finds that directory's `Cargo.toml` and uses it as the dependency and build boundary.

- Downloaded crate source code is cached globally, normally below `~/.cargo/registry`.
- Each project compiles into its own `target/` directory.
- The manifest and lockfile determine which dependency versions this project uses.

So `cd m01-cargo` (or opening the project in IDEA) is the practical equivalent of selecting the project environment.

### Global cache is not shared dependency selection

It is useful to separate **where Cargo stores downloaded bytes** from **which versions a project uses**.

Cargo normally keeps downloaded crate archives and source in a per-user cache (usually under `~/.cargo/registry`). This is best practice: a crate version is immutable once published, so many projects can safely reuse the same downloaded copy rather than each storing an identical copy. Maven normally does something similar with its per-user `~/.m2/repository`; a Python virtual environment is the more project-local model.

The global Cargo cache can contain many versions simultaneously. It does not make all projects use one version. Each project independently determines its dependency graph from:

1. its `Cargo.toml` version requirements and enabled features;
2. its `Cargo.lock` exact resolved versions; and
3. Cargo's dependency resolver.

For example, one application can lock `serde_json` to one compatible release while another locks a different release. Both source versions may sit side by side in the same global cache. Cargo builds them separately for each project's `target/` directory.

For an application, committing `Cargo.lock` is the normal and important isolation mechanism: a fresh clone resolves to the same exact dependency graph, regardless of what other projects or cached crates exist on the machine. `Cargo.toml` and `Cargo.lock` are therefore the equivalent of the project environment's dependency definition—not the cache location.

If a project must also have its **crate source files physically inside the repository**—for example, for an offline or tightly controlled build—Cargo supports vendoring:

```bash
cargo vendor vendor/
```

This copies the locked dependency sources into `vendor/`. Cargo prints the `.cargo/config.toml` source-replacement configuration needed to make builds use that directory. Vendoring is useful for exceptional offline, archival, or compliance requirements, but it is not the usual day-to-day workflow because it makes the repository much larger. It supplements, rather than replaces, `Cargo.lock`.

You may also relocate Cargo's user-level cache by setting `CARGO_HOME`, such as in CI, but that changes cache storage only; it is not needed to isolate a project's selected dependency versions.

---

## IntelliJ IDEA's role

IntelliJ IDEA and the Rust plugin provide a user interface around Cargo; they do not replace it.

| Task | IDEA support | Cargo command to understand |
|---|---|---|
| Create/edit source and `Cargo.toml` | Editor, completion, inspections | — |
| Reload dependencies after a manifest change | Cargo tool window reload action | Cargo resolves dependencies automatically |
| Quickly type-check code | Cargo action/run configuration | `cargo check` |
| Build the executable | Cargo tool window or run configuration | `cargo build` |
| Run `main` | Green gutter Run icon | `cargo run` |
| Reformat Rust code | **Code → Reformat Code** | `cargo fmt` |
| Run lint checks | Inspections or a Cargo run configuration | `cargo clippy --all-targets -- -D warnings` |
| Run tests | Gutter test actions | `cargo test` |

For this learning project, use IDEA productively, but also run the explicit Cargo commands in IDEA's integrated terminal. This lets you explain to students what the IDE is doing.

> **IDE project choice:** Open the `learning-rust/` repository root, so every milestone stays in one IDEA project. Each exercise becomes its own Cargo project when its `Cargo.toml` is created.

---

## Create the first Cargo project

In IDEA's integrated terminal, from the repository root:

```bash
cd exercises/m01-cargo
cargo init --name m01-cargo
```

`cargo init` creates a binary package in the current directory. The generated `src/main.rs` has a synchronous `main` function and prints a greeting.

Open `Cargo.toml` after initialization. Its initial shape will be similar to:

```toml
[package]
name = "m01-cargo"
version = "0.1.0"
edition = "2021" # or the edition selected by your installed Cargo

[dependencies]
```

Do not worry yet about every `[package]` field. For this lesson, recognize that `[dependencies]` is where direct crate dependencies are declared.

After Cargo creates the manifest, IDEA should recognize and load it as a Cargo project. If it does not automatically refresh, use the reload/sync control in the **Cargo** tool window.

---

## Add the dependencies for the exercise

The eventual program will serialize a Rust struct to JSON, deserialize it again, and use an async `main` function.

Run these commands in IDEA's integrated terminal:

```bash
cargo add serde_json
cargo add serde --features derive
cargo add tokio@1 --features macros,rt-multi-thread
```

Why three dependencies?

| Dependency | Role |
|---|---|
| `serde` with `derive` | Supplies the serialization traits and the `#[derive(Serialize, Deserialize)]` macros. |
| `serde_json` | Converts serde-compatible Rust values to and from JSON text. |
| `tokio` | Provides the async runtime and the `#[tokio::main]` macro. |

`serde_json` alone is not enough to derive `Serialize` and `Deserialize`; those traits and macros come from `serde`.

Adding Tokio separately is intentional: `macros` and `rt-multi-thread` are **Tokio features**, not features of `serde_json` or `serde`.

Cargo will update `Cargo.toml` and generate/update `Cargo.lock`. IDEA should then refresh its view of the dependency graph.

### Features are opt-in capabilities, not subpackages

A dependency entry names a **package** (which provides one or more Rust crates). A package can define optional, named **features**: compile-time switches that select extra code, optional dependencies, platform integrations, or APIs. Features are not separately installed subpackages and do not create a second project dependency boundary.

`cargo add serde` enables only the default features selected by the `serde` package authors. Serde's derive support is deliberately optional, so `#[derive(Serialize, Deserialize)]` needs an explicit opt-in:

```bash
cargo add serde --features derive
```

The `derive` feature causes serde's derive-macro support to be included. Keeping it optional avoids making every program that merely uses serde traits compile and depend on procedural-macro tooling.

Tokio takes the same approach, more extensively. It is a modular asynchronous runtime: an application may need only a current-thread runtime, networking, time, synchronization primitives, macros, or some combination. Cargo does not guess which capabilities an application wants, so adding `tokio` alone does not mean “enable every Tokio capability.” For this exercise we explicitly request only what the program needs:

```bash
cargo add tokio@1 --features macros,rt-multi-thread
```

- `macros` enables `#[tokio::main]`.
- `rt-multi-thread` enables Tokio's multithreaded runtime.

Tokio also offers a convenience `full` feature that enables its broad common feature set:

```bash
cargo add tokio@1 --features full
```

It is useful for quick experiments, but requesting the small, explicit set is normally better for learning and production applications: the manifest communicates intent and Cargo downloads/compiles less code.

To add features to a dependency already present in `Cargo.toml`, repeat `cargo add` with the feature flag; Cargo updates the existing entry rather than creating a duplicate:

```bash
cargo add serde --features derive
cargo add tokio --features macros,rt-multi-thread
```

You *can* depend directly on a separate package when it is part of that package's public API, but normally you should enable the owning package's feature. For example, use `serde` with `derive`, not a direct `serde_derive` dependency; use Tokio's `macros` feature, not a direct `tokio-macros` dependency. Those implementation-level packages are selected transitively by the public feature and do not belong in this application's direct-dependency list.

---

## The three core build commands

Use these commands from `exercises/m01-cargo/` after adding the dependencies or changing source code.

### `cargo check`

```bash
cargo check
```

Cargo type-checks the project and its dependencies but does not finish code generation for an executable. It is usually the fastest feedback loop while editing.

Use it when asking: **“Does this compile?”**

### `cargo build`

```bash
cargo build
```

Cargo type-checks and fully compiles the executable. Debug output is written beneath:

```text
target/debug/m01-cargo
```

Use it when asking: **“Can Cargo produce the binary?”**

### `cargo run`

```bash
cargo run
```

Cargo builds if needed and then runs the executable. If nothing relevant changed since the previous build, it reuses cached outputs from `target/`.

Use it when asking: **“What does the program do?”**

### First-build versus later-build timing

The first command that needs a dependency will download and compile it, so it can take noticeably longer. Run the same command again without changes: Cargo should reuse the project build cache and be much faster.

### Why one source file produces many `target/debug` entries

`src/main.rs` is the source input, not the only file Rust needs while compiling. Cargo and `rustc` retain intermediate outputs and metadata so that later `check`, `build`, and `run` commands can avoid rebuilding unchanged work. A typical `target/debug/` contains:

| Entry | Purpose |
|---|---|
| `m01-cargo` | The debug executable produced by `cargo build` or `cargo run`. |
| `deps/` | Compiled crate outputs, Rust metadata (`.rmeta`), dependency files (`.d`), and dependency executables. Even the package itself is handled as a crate here. |
| `.fingerprint/` | Records of the inputs and compiler settings used for artifacts. Cargo uses these to decide whether an artifact is still valid. |
| `incremental/` | Incremental-compilation state, allowing Rust to reuse unchanged compiler work after a small edit. |
| `build/` | Output for crate build scripts. Cargo may create this standard directory even when this simple package currently has no build scripts. |
| `examples/` | Location for compiled examples; it can exist even if the project has none. |

The hash-like suffixes in names below `deps/`, `.fingerprint/`, and `incremental/` distinguish artifact variants. They reflect compilation inputs such as the crate, target, profile, features, and compiler configuration. More than one can remain after source or configuration changes; this lets Cargo safely reuse compatible results rather than guessing that differently built artifacts are interchangeable.

So this structure is expected even for the generated one-file program and no dependencies. It will grow when we add `serde` and Tokio, because every dependency also has compilation outputs and metadata. It is a cache, not source code or a deployment layout. If you need to reclaim space or diagnose a suspicious cache, run:

```bash
cargo clean
```

That removes this package's `target/` contents. The next Cargo command rebuilds them; it does not remove the global downloaded-crate cache under `~/.cargo/registry`.

---

## Version control rules for this exercise

For this executable application:

- Commit `Cargo.toml`.
- Commit `Cargo.lock`, because it records the exact dependency graph used by the application.
- Do **not** commit `target/`; it is reproducible generated output and can be regenerated with Cargo.

---

## Checkpoint questions

Before moving to the JSON program, be able to answer these in your own words:

1. Why does Cargo not require a `.venv` activation step?
2. What belongs in `Cargo.toml` versus `Cargo.lock`?
3. Why are downloaded crates globally cached but compiled artifacts project-local?
4. Which command would you use for the fastest compile feedback: `check`, `build`, or `run`?
5. What Cargo command is IDEA ultimately using when you click the green Run icon beside `main`?

---

## Next lesson

Write an async `main` that defines a serializable struct, serializes it to JSON, deserializes it back, and prints both forms. Then run formatting, type-checking, building, and execution through both Cargo and IDEA.
