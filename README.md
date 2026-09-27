# Learning Rust for Teaching Goose Agentic Applications

This repository documents a structured learning plan for an instructor who knows **Java** (and some Python) and needs enough Rust to teach a live, instructor-led course on building agentic applications with the **Goose Development Kit (GDK)**.

The course this instructor teaches is at: [gdk-sample-app-rust](https://github.com/stpousty/gdk-sample-app-rust)

## 1. Goal

Learn "just enough Rust to teach" — not to become a systems programmer, but to confidently explain every concept the students encounter and answer questions that go beyond the course material. This repository is where you practice each milestone before standing in front of a class.

---

## 2. What You Are Building Toward

The instructor-led course teaches learners (mostly Python developers) how to build a native Rust CLI application using the `goose-providers` crate:

| Lesson | GDK Concept |
|--------|-------------|
| 1 | Cargo project bootstrap, manifest, toolchain pinning |
| 2 | Declarative provider JSON, constructing a provider, model discovery |
| 3 | Native messages, model selection, streaming inference with Tokio |
| 4 | System instructions, multi-turn conversation history |
| 5–8 *(planned)* | Explicit model selection, tool definition & execution, CLI wiring, config |

You need to understand the **Rust mechanics** behind every one of these lessons and the surrounding ecosystem (package management, code hygiene, memory model, async, macros).

---

## 3. Your Prerequisites

- Strong Java background (OO, generics, interfaces, exceptions)
- Some Python exposure
- Familiar with command-line tools, terminal workflows
- **No Rust experience** — starting from zero

---

## 4. How This Repository Is Organized

```
learning-rust/
├── README.md            ← You are here (the plan and reference mappings)
├── exercises/           ← Hands-on practice for each milestone (TODO: implement later)
│   ├── m01-cargo/
│   ├── m02-ownership/
│   ├── m03-errors/
│   ├── m04-async/
│   ├── m05-traits/
│   ├── m06-macros/
│   └── m07-cli-hygiene/
└── cheat-sheet.md       ← Java → Rust mapping for quick reference (TODO: create later)
```

**This repository is the plan.** You will implement the exercises (`exercises/`) as you work through each milestone, but that comes in a future session. Right now this document captures everything.

---

## 5. Rust's Package Management Equivalent

Python developers expect `requirements.txt` / `pyproject.toml`, a virtual environment to isolate dependencies, and `pip install` to download into `.venv`. **Rust solves the same problems with fewer moving parts:**

| Concept | Python | Java (Maven/Gradle) | Rust / Cargo |
|---------|--------|---------------------|--------------|
| Dependency file in project root | `pyproject.toml` / `requirements.txt` | `pom.xml` / `build.gradle` | **`Cargo.toml`** |
| Lock file (exact resolved versions, committed to git) | `pip freeze` (manual-ish) | `pom.xml` has no lock; Maven Central is stable | **`Cargo.lock`** — auto-generated, auto-maintained, committed to git for apps |
| Per-project isolation | `.venv/` + `source activate` | Each project has its own `target/` and local repo cache | **No virtual env needed.** Dependencies download to `~/.cargo/registry`, but compilation happens per-project. `Cargo.toml` is the scope boundary — one file does what Python needs two files for. |
| Pinning a language/tool version | `.python-version` + pyenv | `maven.compiler.source` in POM | **`rust-toolchain.toml`** — same pattern: pin exact toolchain channel (`1.94.1`, `stable`) per project. Cargo reads it automatically. |
| Adding a dependency | `pip install package` or `poetry add package` | `mvn dependency:add` (not really) / manual edit | **`cargo add serde_json tokio@1 --features "macros,rt-multi-thread"`** — the idiomatic CLI way to add deps. Never hand-edit dependencies unless you have a specific reason. |
| Updating a dependency | `pip install --upgrade package` | `mvn versions:display-dependency-updates` | **`cargo update -p package_name`** (single) or bare `cargo update` (all). Cargo is conservative; it only bumps to compatible semver ranges. Alpha dependencies require exact pins (`=0.1.0-alpha.10`). |
| Feature flags / optional deps | `pip install package[extra]` | `<optional>true</optional>` in POM | **Cargo features** — e.g., `goose-providers = { version = "...", features = ["rustls-tls"] }`. A feature can gate an entire dependency (making it optional). |
| Installing a binary globally | `pip install --global ...` (not really supported) | `mvn deploy` to repo, then install locally | **`cargo install <crate>`** — puts a binary in `~/.cargo/bin/`. For app development (your case), avoid this. Every project is already self-contained. |

### The One Thing That Is Different from Python

In Python, you activate a venv and it stays active for the session. In Rust, **there is no activation step**. You simply `cd` into the project directory and run `cargo build`. Cargo reads that directory's `Cargo.toml`, downloads dependencies to the global registry cache (only once per version), and compiles everything fresh into that project's `target/` directory. The isolation is structural — one directory, one manifest.

---

## 6. Milestones

---

### Milestone 1 — Cargo: Package Management, Build, Run

**Why this matters first:** Every lesson begins with a `cargo check`, `cargo build`, or `cargo run`. Students will ask where dependencies live, why the lock file exists, and how to add new crates. You need to answer confidently.

#### Topics

| Topic | What to learn | Java analog | Hours |
|-------|---------------|-------------|-------|
| `Cargo.toml` | `[package]` (name, version, edition, rust-version), `[dependencies]`, `features`, `[[bin]]` for multi-binary projects | `pom.xml` / `build.gradle`. Edition 2021 = Java 17 equivalent (Rust's current LTS release). `rust-version` pinning = `.java` compiler target. | 2 |
| `Cargo.lock` | Deterministic builds — every CI and developer gets exactly the same transitive dep tree. Commit to git for applications. Running `cargo update` is how you explicitly bump versions. | Maven Central + `<dependencyManagement>` gives deterministic builds, but there's no lock file artifact. Cargo locks at the source level (git rev), not just semver. | 0.5 |
| `rust-toolchain.toml` | Per-project toolchain pinning with channels (`1.94.1`, `stable`, `nightly`). Profiles control which components to install (e.g., `minimal` for just the compiler + clippy + rustfmt). | Not analogous in Java — there is exactly one JDK per system, chosen manually. Rust treats toolchain version as part of the project configuration. | 0.5 |
| Basic workflow | `cargo check` (fastest: type-check only, no codegen) → `cargo build` (full compilation, artifacts in `target/debug/`) → `cargo run` (build + execute) → `cargo clean` (removes target/) | Compile and link happen in one step. No separate `javac` and `java`. Incremental compilation is fast — Rust caches it in `target/CACHEDIR.TAG`. | 1 |
| Adding/updating deps | Use `cargo add <crate>[@<version>] --features "<f1>,<f2>"` — the preferred CLI. For hand-editing Cargo.toml, keep Cargo.lock in sync (run `cargo check` after editing). | Maven: edit `pom.xml`, then run `mvn compile`. Gradle: edit `build.gradle.kts`, then sync. Cargo's approach is closer to Gradle but faster because incremental compilation caches everything. | 1 |
| Features & optional deps | A feature can enable another crate as a dep (`features = ["rustls-tls"]`). Default features are opt-in for library consumers. You control what gets compiled in your binary. | Maven: `<optional>true</optional>` + profiles. Gradle: `implementationOnly` configurations. Cargo's feature system is more direct — one boolean per feature, composable. | 1 |
| Multi-binary & workspaces | A single crate can have multiple binaries via `[[bin]]` sections in one manifest. Workspaces (for library + app combos) are mentioned briefly — the course uses a single binary; GDK SDK crates use workspaces. | Maven: multi-module projects with parent POM. Gradle: composite builds. Cargo workspaces = `Cargo.toml` with `[workspace]` and `members = ["crates/*"]`. | 1 |

#### Hands-on exercise (implement in `exercises/m01-cargo/`)

1. `cargo init m01-cargo` from scratch
2. Add `serde_json` and `tokio@1` with features via `cargo add`
3. Write a simple async `main()` that creates a struct, serializes it to JSON, deserializes it back
4. Run `cargo fmt --check`, `cargo check`, `cargo build`, `cargo run` — observe each step's output and timing
5. Add `rust-toolchain.toml` pinning to `1.94.1` (or whatever stable version is current)
6. Change a dependency version in Cargo.toml, run `cargo update -p <crate>`, inspect the diff

#### Key commands reference

```bash
cargo init myproject                    # Create new project
cd myproject && cargo add serde_json    # Add dependency
cargo fmt --check                       # Check formatting (fast)
cargo check                             # Type-check only (fastest build step)
cargo build                             # Full compilation → target/debug/myproject
cargo run                               # Build + execute
cargo clippy --all-targets -- -D warnings  # Lint: treat all warnings as errors
cargo test                              # Run all tests
cargo clean                             # Remove target/ directory
```

---

### Milestone 2 — Ownership & Memory (The Part Java Devs Struggle With)

**Why this matters:** You'll see `&`, `&mut`, `String` vs `&str`, and `Box<T>` constantly. Students will hit borrow checker errors and you must explain them without panic. This is the single hardest topic for your students too (Java devs are used to GC + references that can't dangle).

#### Topics

| Topic | What to learn | Java analog | Hours |
|-------|---------------|-------------|-------|
| **Move semantics** | `let b = a` transfers ownership for types that don't implement `Copy`. After the move, `a` is invalidated. Primitives (`i32`, `bool`, `f64`) and types implementing `Copy` (like tuples of primitives) copy by value. | Java: `String b = a;` — both references to the same object. In Rust: `let b = a;` for a `String` means `a` is now invalid. This is the #1 new mental model. Think of it as Java's `final` + explicit nulling happening automatically: once ownership moves, the old handle is dropped. | 2 |
| **Borrowing** | `&T` (immutable borrow) vs `&mut T` (mutable borrow). Rules enforced at compile time: you can have either one mutable borrow OR any number of immutable borrows, never both. No two mutable aliases allowed. | Java: multiple threads can hold references to the same mutable object — data races are possible and common in practice. Rust forbids this entirely at compile time. Think of it as `final` for references plus `synchronized(this)` enforced by the compiler: the borrow checker guarantees no aliasing mutation. | 3 |
| **Lifetimes** | The compiler tracks how long each reference lives. Most code doesn't need explicit lifetime annotations (`'a`) — the compiler infers them via "elision rules." You only write `'a` explicitly when returning a reference from a function where the compiler can't determine which input it's tied to. In practice, ~95% of Rust code never writes a lifetime annotation. | Java: GC handles dangling references automatically. Rust eliminates GC entirely by proving at compile time that no reference can outlive the data it points to. Elision rules ≈ "the returned reference borrows from whichever input lives longest." When you see `'static`, it means "this reference lives for the entire program" (like a static string literal). | 3 |
| **String vs &str** | `String` = owned, heap-allocated, growable (like Java's `StringBuilder` but immutable once created — use `.push_str()` for mutation). `&str` = borrowed view into a UTF-8 string slice. Methods that return `&str` allocate nothing. | Java: `String` is always immutable. Rust splits this: owned (`String`) ≈ mutable Java string, borrowed (`&str`) ≈ `substring()` but without allocating — just a pointer and length into the original data. This is zero-cost abstraction in action. | 2 |
| **Box&lt;T&gt;, Rc&lt;T&gt;** | `Box<T>` = heap allocation with single ownership (like Java's implicit heap objects). `Rc<T>` = reference-counted shared ownership. Rarely needed in simple CLI apps, but good to recognize. | Java: every object is on the heap + GC-owned ≈ implicit `Rc`. Rust makes sharing explicit — you must use `Rc` when you intentionally want multiple owners. No invisible GC cycles. | 1–2 (recognition only) |
| **Drop / RAII** | Resources are freed exactly when they go out of scope. The compiler inserts `drop()` calls at the end of each variable's lifetime. You can implement the `Drop` trait to customize cleanup (file handles, network connections). | Java: GC reclaims objects non-deterministically (`finalize()`/`Cleaner`). Rust: deterministic, immediate deallocation at scope exit — exactly C++ RAII but built into the language with no unsafe code needed. | 1 |

#### Key borrow checker scenarios to practice

These are the three rules that generate 90% of errors:
1. **Cannot move out of borrowed content** — you can't take ownership of a `String` stored behind a `&` reference
2. **Cannot borrow as mutable more than once at a time** — two `&mut` borrows overlap in scope
3. **Cannot borrow as immutable while mutable borrow is active** — mixing `&` and `&mut` in overlapping scopes

Each of these maps to a Java concept that Rust prevents: dangling pointers, data races, and use-after-free.

#### Hands-on exercise (implement in `exercises/m02-ownership/`)

1. Write code where ownership moves between variables — observe compile errors when you try to use the moved value
2. Experiment with borrowing: write functions that take `&String`, `&mut String`, and `&str` — observe what each can and cannot do
3. Create a function that returns an owned `String` (never a `&str`) to avoid lifetime annotations entirely
4. Write a struct with fields, create it, drop it explicitly with `drop()`, and observe the Drop output

---

### Milestone 3 — Error Handling & Control Flow

**Why this matters:** Course code uses `Result<T, E>` and `?` everywhere. Students will ask "why no exceptions?" and you need to explain the design philosophy. You also need to be comfortable reading `match` expressions — Rust's primary control flow construct.

> **New subtopic (agentic CLI focus):** When inspecting `goose-providers` source, students will encounter three error-related patterns they've never seen: `#[derive(Serialize, Deserialize)]` attributes on structs (for LLM JSON schemas), `thiserror`-derived error enums in provider libraries, and `anyhow::Result` in CLI wiring code. Understanding these prevents confusion when reading GDK SDK internals.

#### Topics

| Topic | What to learn | Java analog | Hours |
|-------|---------------|-------------|-------|
| **Result&lt;T, E&gt;** | Two variants: `Ok(T)` and `Err(E)`. Like Java's checked exceptions but encoded as a value type. The compiler forces you to handle every error path — you can't silently ignore a fallible call. | Java: checked exceptions (`throws IOException`) + runtime exceptions. Rust's `Result` = all exceptions are checked at compile time, but they're values (not control-flow). You pattern-match on them instead of catching them. | 2 |
| **The ? operator** | `maybe_err?` returns the error early if `Err`, unwraps `Ok` otherwise. Works in functions that return `Result<T, impl Into<Error>>`. The compiler applies the `From` trait to convert the error type. | Java: implicit exception propagation — `throw` bubbles up through call stack without explicit handling. Rust's `?` = explicit but ergonomic propagation. Same effect, different mechanism. | 1 |
| **Box&lt;dyn Error&gt;** | A trait object that can hold any error type implementing `Error`. Used in function signatures like `-> Result<(), Box<dyn Error>>` when you want to return "any error." The compiler converts specific errors (IO, JSON parse, etc.) into this generic form. | Java: `Exception` is the base class for all exceptions — `throws Exception` can catch everything. `Box<dyn Error>` ≈ `throws Exception`, but you must convert each error type explicitly via `From` implementations. | 2 |
| **match** | Exhaustive pattern matching on enums and results. The compiler enforces that every branch is handled. `if let Some(x) = opt { ... }` for single-variant cases. | Java: `switch` with sealed classes (Java 17+) approaches Rust's exhaustiveness, but Java's `switch` still allows fall-through bugs and missing cases compile silently. Rust's `match` has no escape hatch — forget a case, compiler complains. | 2 |
| **Option&lt;T&gt;** | Two variants: `Some(T)` or `None`. Like Python's `Optional[T]` but the compiler forces you to handle `None` (via `match`, `if let`, `unwrap_or()`, etc.). Impossible to forget — Java's `null` checks are optional and easy to skip. | Java: `Optional<T>` (a library class, not a language feature). Rust's `Option` is built into the type system — you literally cannot create an uninitialized variable. The compiler won't let you access a `T` from an `Option<T>` without first handling the `None` case. This eliminates NPEs entirely. | 1–2 |

#### New Subtopic 3a: Serde Attributes & Schema Mechanics

**Why this matters:** Agentic apps rely on structured JSON input/output for tool call signatures, system instructions, and LLM message payloads. Every provider request/response struct uses serde attributes to control how Rust types map to JSON schemas.

| Topic | What to learn | GDK example | Hours |
|-------|---------------|-------------|-------|
| `#[derive(Serialize, Deserialize)]` | Tells the serde crate to generate `serde::Serialize` and `serde::serde::Deserialize` implementations at compile time. Requires `features = ["derive"]` in `Cargo.toml`. | Every `goose-providers` message struct derives these — the macro-generated code is what turns your Rust struct into a JSON payload sent to the LLM. | 1 |
| `#[serde(rename_all = "snake_case")]` | Transforms field names from Rust's `CamelCase` to JSON's `snake_case`. Essential for APIs that expect specific casing conventions. | Provider config structs often use this — the LLM API may expect `model_id` in JSON but you name it `modelId` in Rust for convention alignment. | 0.5 |
| `#[serde(skip_serializing_if = "Option::is_none")]` | Omits optional fields from output when they are `None`. Keeps JSON payloads clean and avoids sending nulls the API doesn't expect. | Optional provider parameters (temperature, max_tokens) appear in JSON only when explicitly set by the user. | 0.5 |
| `#[serde(untagged)]` | Allows deserialization to try multiple variant types until one succeeds. Useful for fields that accept different kinds of values. | When parsing tool call arguments that may be strings, objects, or arrays depending on the tool definition. | 0.5 |

**Book references:** Programming Rust Ch. 2 ("A Tour of Rust") for derive intro → Ch. 18 ("Input and Output") for complete serde coverage. Effective Rust Ch. 1 for type system context. See `enhancement-mapping.md` for detailed chapter-by-chapter breakdown.

#### New Subtopic 3b: Error Context — anyhow / thiserror vs Box<dyn Error>

**Why this matters:** Students inspecting `goose-providers` source will encounter `thiserror`-derived error enums in provider implementations and `anyhow::Result` in CLI code. Without understanding the distinction, they'll wonder why some functions return `Box<dyn Error>` while others use custom error types.

| Topic | What to learn | GDK example | Hours |
|-------|---------------|-------------|-------|
| `thiserror` (library errors) | A derive macro for enum-based error types with `#[error("...")]` and `#[from]` attributes. Generates clean `Display + Error` implementations without exposing internal error types in the public API. | Provider crates define `enum ProviderError { Http(#[from] reqwest::Error), Json(#[from] serde_json::Error) }` — callers get a single concrete type but can still pattern-match on specific variants. | 1 |
| `anyhow::Result<T>` (app errors) | A type alias for `Result<T, anyhow::Error>`. Provides automatic `From` conversion from any error type + stack trace capture. The `anyhow!()` macro adds context; `bail!` returns early with an error. | The CLI application uses `fn main() -> anyhow::Result<()>` and chains operations with `?` — each `.map_err(|e| anyhow!("context: {e}"))?` wraps errors without changing their type. | 1 |
| Library vs Application distinction | **Library code → `thiserror`** (callers need concrete types to match on). **Application code → `anyhow`** (just surface errors with good messages). This is the pattern in GDK itself. | Provider SDK uses `thiserror`; your CLI app wraps it with `anyhow`. The boundary between library and application determines which crate to use. | 0.5 |

**Book references:** Effective Rust Ch. 1 ("Types") — author David Drysdale explicitly recommends both crates with the quote: "consider using thiserror for library error types" and "consider using anyhow for applications." Command-Line Rust chapters 6–14 show complete CLI patterns (ch11 has 12 anyhow examples). Programming Rust Ch. 7 covers the standard library `Box<dyn Error>` approach as a foundation. See `enhancement-mapping.md` for detailed chapter-by-chapter breakdown.

#### Hands-on exercise (implement in `exercises/m03-errors/`)

1. Write a function that returns `Result<String, Box<dyn Error>>` — read a file, parse JSON, return the result
2. Use `?` to propagate errors at multiple levels — observe how the compiler converts error types automatically
3. Practice `match` on both `Result<T, E>` and `Option<T>` with exhaustive handling
4. Write a function that uses `if let` for a common case (e.g., parsing an optional CLI argument)
5. **NEW:** Define a struct with `#[derive(Serialize, Deserialize)]` including at least three serde attributes (`rename_all`, `skip_serializing_if`, `untagged`). Serialize to JSON and print the output — compare with manual JSON writing.
6. **NEW:** Create a library-style error enum using `thiserror` (in an exercises crate), then write a CLI function that uses `anyhow::Result<()>` to call it. Observe how errors flow from concrete types to generic anyhow.

---

### Milestone 4 — Async & Tokio

**Why this matters:** The streaming lessons are entirely async. Students will be confused by `async fn`, `.await`, and why `fn main()` has to become `#[tokio::main] async fn main()`. You need to understand the async runtime model, not just the syntax.

> **New subtopic (agentic CLI focus):** When returning streams from dynamic providers (`dyn Provider`), students encounter `Pin<Box<dyn Stream<Item = ...>>>` and `BoxStream<T>`. Java developers expect `Stream<T>` as a return type — Rust's requirement for pinned heap allocation is a major compiler-error source. Understanding what Pin means at a conceptual level prevents panic when writing provider traits.

#### Topics

| Topic | What to learn | Java analog | Hours |
|-------|---------------|-------------|-------|
| **async/await syntax** | `async fn` returns a `Future` (a lazy promise). The function body doesn't execute until awaited. `.await` is a suspension point — it yields control back to the runtime, allowing other tasks to run. | Java: `CompletableFuture<T>` with `.thenApply()`, `.thenAccept()` callbacks. Rust's `.await` = synchronous-looking code that actually suspends the coroutine. Much cleaner than callback chains. Very similar to Kotlin coroutines (`suspend fun`). | 1 |
| **#[tokio::main]** | Makes `fn main()` async. Without it, you'd need to manually create a Tokio runtime and call `.block_on()`. For CLI apps, this macro is always used. It generates the boilerplate that starts the event loop and waits for the future to complete. | Java: `public static void main()` can't be async — you must call `.join()` on a `CompletableFuture` or block on a `CountDownLatch`. Rust uses the macro as a bridge between synchronous entry points and async code. | 0.5 |
| **The event loop** | Tokio runs on a thread pool (multi-threaded runtime with `rt-multi-thread` feature). Each `.await` point is a yield where other tasks can run. The runtime schedules work across OS threads. For single-function programs, behavior is similar to Python's asyncio. | Java: executors with `CompletableFuture.runAsync()`. Tokio's multi-threaded pool ≈ Java's common ForkJoinPool but for async tasks instead of CPU tasks. Single-threaded Tokio (`rt` feature) ≈ Java's `EventLoopGroup` from Netty. | 1 |
| **Streams & StreamExt** | `futures::StreamExt` provides `.next()` for consuming async iterators. A stream item is `Option<Result<T, E>>`: `None` = stream ended normally, `Some(Err(e))` = error, `Some(Ok(item))` = next value. Course uses `while let Some((msg, usage)) = stream.next().await.transpose()?`. | Java: `java.util.stream.Stream` (synchronous only) or Project Reactor's `Flux<T>`. Rust streams are like `Flux<T>` but consume with imperative `while` loops instead of reactive operators. Each `.next().await` is a yield point. | 2 |
| **tokio features** | `macros` = provides `#[tokio::main]`. `rt-multi-thread` = thread-pool runtime (what you need for CLI apps). `net` = TCP/UDP support (not used in course). You only compile what you link — no overhead for unused features. | Java: Spring Boot pulls in the entire async stack regardless of usage. Cargo + Tokio's feature system means your binary only contains code you actually use. | 1 |

#### New Subtopic 4a: Demystifying Pin<Box<dyn Stream>>

**Why this matters:** Dynamic provider traits (`dyn Provider`) return streams via `BoxStream<T>`, which is a type alias for `Pin<Box<dyn Stream<Item = T> + Send>>`. Java developers expect `Stream<T>` but Rust requires the heap allocation + immovable guarantee for async streams.

| Topic | What to learn | GDK example | Hours |
|-------|---------------|-------------|-------|
| **What Pin means** | "This memory location cannot move while async work is pending." Futures hold self-referential data — fields that point into their own struct across suspension points. If the struct moves (e.g., due to reallocation), those internal pointers would dangle. `Pin` tells the compiler: "lock this in place." | Conceptual analog for Java devs: think of it as an object whose GC reference is pinned so the collector cannot relocate it while a thread holds active references to its internals. | 1 |
| **Why Box + Pin together** | `Box<T>` allocates on the heap (normally movable via reallocation). `Pin<Box<T>>` locks that heap allocation in place for the duration of the async operation. This is how Rust provides stable addresses for self-referential futures. | The provider's stream result must stay at a fixed address so the Tokio runtime can poll it repeatedly without the memory location changing. | 0.5 |
| **BoxStream type alias** | `type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;` — saves typing trait object syntax. Without this alias, every dynamic provider return type would be 60+ characters of generic nesting. | `goose-providers` defines `BoxStream` in its trait signatures. When students see it in provider code, they should recognize it as "a pinned heap-allocated stream trait object." | 0.5 |
| **Returning streams from traits** | `impl Stream<...>` = different concrete type each call (can't be a trait object). `BoxStream<T>` = same concrete type (`Pin<Box<dyn Stream>>`) every call — required for trait method signatures where the return type must be consistent across all implementors. | `fn stream_messages(&self, msgs: Vec<Message>) -> BoxStream<Result<String, ProviderError>>;` — this is a pattern students will copy when building their own providers. | 1 |

**Book references:** Async Rust by Maxwell Flitton Ch. 5 ("Coroutines") is the definitive source with 31 matches for Pin/BoxStream content. Progressive reading: Ch. 2 (basic Pin + future poll), Ch. 3 (async runtime wakers + Pin), Ch. 4 (networking + Pin pattern), Ch. 5 (comprehensive coroutine/pool/pinning). Programming Rust Ch. 20 shows Pin in async context with networking examples. See `enhancement-mapping.md` for detailed chapter-by-chapter breakdown.

#### Hands-on exercise (implement in `exercises/m04-async/`)

1. Write a simple `#[tokio::main] async fn main()` that does `tokio::time::sleep(...)` between two prints — observe how `.await` yields
2. Use `futures::stream::iter()` to create an in-memory stream and consume it with the same pattern from the course: `while let Some(item) = stream.next().await { ... }`
3. Combine `tokio::time::sleep` with streams — observe concurrent execution when you spawn multiple tasks
4. **NEW:** Define a trait with a method returning `BoxStream<'_, Result<String, String>>`. Implement it for two different types (one sync-converting, one truly async). This demonstrates the Pin + BoxStream pattern used by dynamic providers.

---

### Milestone 5 — Traits & Patterns

**Why this matters:** The course uses `dyn Provider`, `Provider` trait, and `StreamExt`. Students will encounter traits constantly. You need to understand how Rust's trait system compares to Java's interfaces (it's more flexible and equally powerful once you get used to it).

#### Topics

| Topic | What to learn | Java analog | Hours |
|-------|---------------|-------------|-------|
| **Traits** | Like interfaces but with superpowers: you can implement them for types you don't own (orphan rule), add default method implementations, and use them in generic bounds. `impl Trait` = return type that is any type implementing the trait. | Java: interfaces (`interface Foo`). Same concept, but Rust traits can be implemented for external types (Java requires the type to be defined in your package or you must extend it). Rust also has "extension methods" via blanket impls — implement a trait for `T` and every `U: T` gets it. | 2 |
| **dyn Trait objects** | `&dyn Provider`, `Box<dyn Error>`. Dynamic dispatch via vtable lookup at runtime. Used when you don't know the concrete type at compile time but know it implements a trait. | Java: `Interface obj = new Impl()`. Exactly the same concept, but Rust requires explicit `dyn` syntax (older code omitted this keyword). No implicit conversions — if the compiler doesn't know you want a trait object, you'll get a type error. **Also:** `BoxStream<T>` is `Pin<Box<dyn Stream<Item = T> + Send>>` — it's a dyn trait object for streams, built on top of Pin (see Milestone 4). | 2 |
| **impl Trait vs generics** | `fn foo(x: impl MyTrait)` = "accept any single type that implements `MyTrait`." `fn foo<T: MyTrait>(x: T)` = same thing but reusable across calls with inlining for monomorphization. The former is shorthand; the latter lets you reference the type parameter elsewhere. | Java: `<T extends MyInterface> void foo(T x)`. Rust's `impl Trait` syntax ≈ Java's diamond operator (`foo(new Foo<>())`) — the type is inferred at the call site rather than named in the signature. | 2 |
| **Structs & impl blocks** | `struct Foo { field: Type }` — no methods, no inheritance. Methods come from separate `impl Foo { fn bar(&self) ... }` blocks. Struct fields are private by default. Constructor conventions: `fn new(...)` or `fn builder()`. | Java: everything is a class with fields + methods mixed. Rust splits these into struct (data only) and impl (behavior only). Multiple impl blocks can extend the same struct. No inheritance hierarchy — composition over inheritance is the idiomatic pattern. | 1 |
| **Enums with data** | `enum Message { User(String), Assistant(String) }` — variants can hold arbitrary data. Used everywhere: `Result<T, E>`, `Option<T>`, conversation messages in the course. Pattern-match on variants to extract the data. | Java: sealed classes / interfaces (`sealed interface Message permits UserMsg, AssistantMsg`). Rust enums are algebraic data types — more powerful than Java's enum (which only supports constants) and equivalent to sealed classes with value-bearing subclasses. Exhaustive matching ensures you never forget a variant. | 1–2 |

#### Hands-on exercise (implement in `exercises/m05-traits/`)

1. Define a trait, implement it for two different types (one of which is from an external crate via the orphan rule)
2. Write a function that takes `dyn Provider` and call it with two different concrete provider types
3. Create an enum with data-carrying variants, write exhaustive `match` logic for every case

---

### Milestone 6 — Macros

**Why this matters:** You'll see `#[tokio::main]`, `println!()`, `vec![]`, and potentially custom macros. You need to understand *what* macros do (compile-time code generation), not necessarily write complex ones yourself, but being able to write a simple `macro_rules!` macro builds confidence when reading existing code.

#### Topics

| Topic | What to learn | Java analog | Hours |
|-------|---------------|-------------|-------|
| **Macros 1.0 overview** | Rust has compile-time macros that expand into AST nodes (actual code). `println!("hello")` isn't a function call — it's macro-expanded into type-checked format string logic. They run at the macro expansion phase, before type checking. Unlike C preprocessor text substitution, Rust macros manipulate parsed syntax trees. | Java: annotation processors (run after parsing, before compilation) or Lombok @generated code. Rust macros run earlier and produce real AST nodes — not text patches. Think of them as Java annotation processors that actually produce code rather than generating source files to be compiled separately. | 2 |
| **Declarative macros (`macro_rules!`)** | Pattern matching on tokens, then expanding into code. `vec![1, 2, 3]` is a declarative macro — it expands to `Vec::from([1, 2, 3])`. Recognize them by the `!` suffix: `name!(...)`, `name! { ... }`, or `#[name(...)]`. | Java: none. The closest analog might be Javapoet (a code generation library) but Rust macros are inline and compile-time. Python decorators run at function definition time, which is conceptually similar to declarative macros in that they transform the decorated item. | 1–2 (recognition + writing simple ones) |
| **Attribute macros** | `#[tokio::main]` — transforms an item (function, struct, etc.) before compilation. Applied at the item level. Generates wrapper code around the annotated function. | Java: `@Override`, `@Component`, Lombok annotations. All compile-time transformations. `#[tokio::main]` ≈ generating a boilerplate `fn main()` that creates a Tokio runtime and calls your async function inside `.block_on()`. Conceptually identical to Kotlin's `@JvmStatic` — syntactic sugar for code generation. | 0.5 |
| **Derive macros** | `#[derive(Clone, Debug)]` — compiler-generated impl blocks. You never write these by hand for common traits. | Java: Lombok's `@Data` / `@Getter` / `@EqualsAndHashCode`. Rust's derive is built into the language — `Clone`, `Debug`, `Default`, `PartialEq` are all standard derive macros. | 0.5 |

#### Hands-on exercise (implement in `exercises/m06-macros/`)

1. Use `#[derive(Debug, Clone)]` on a struct and observe what the compiler generates
2. Write a simple declarative macro: `macro_rules! say_hello { () => { println!("hello!"); } };` — call it from a function
3. Read the source of `vec![]` or `println!()` (available via `cargo expand` or `rustc --print cfg`) to see how they expand

---

### Milestone 7 — CLI Application Construction & Code Hygiene

**Why this matters:** The course's planned Lesson 8 covers CLI wiring. Students will write CLI tools and need to know about formatting, linting, testing, releasing, and publishing patterns. You should model good Rust practices yourself so students can follow your example.

#### Topics

| Topic | What to learn | Java analog | Hours |
|-------|---------------|-------------|-------|
| **Argument parsing** | Course hand-rolls CLI args with `std::env::args_os()` + `match`. For production, use `clap` (the Rust equivalent of Apache Commons CLI or Picocli). Understand both patterns so students aren't confused by the difference. | Java: Apache Commons CLI / Picocli. Rust's clap is more type-safe — argument types are encoded in the return type (`ArgMatches::get_one::<String>()` returns `Option<&String>`, not `Object`). | 1 |
| **Config file handling** | Read JSON config → parse with `serde_json` → construct provider. Uses `fs::read_to_string()`, `?` for error propagation, `FromStr`/deserialization traits. Clean pattern used in the course. | Java: Jackson/Gson for JSON parsing. Rust's approach is similar but `serde` can derive serialization implementations (`#[derive(Serialize, Deserialize)]`) vs Jackson annotations (`@JsonProperty`). **NEW:** Provider config structs use serde attributes like `#[serde(rename_all = "snake_case")]` and `#[serde(skip_serializing_if = "Option::is_none")]` to control JSON output — same attributes students define for their own CLI config structs. See Milestone 3a (Serde Attributes) for the full attribute reference. | 1 |
| **cargo fmt** | Auto-formats code according to one canonical style. The Rust community has adopted a single format — no "code style debate." Run before every commit. Not optional — it's the community standard. | Java: Google Java Format / Spotless. Same idea but Rust's formatter is built into the toolchain (`rustfmt`) and has near-universal adoption. | 0.5 |
| **cargo clippy** | Linter that catches idiomatic issues, not just correctness errors. `clippy --all-targets -- -D warnings` = treat all warnings as errors. Think Pylint + mypy combined with style enforcement. Catches things like "use `.iter()` instead of manual indexing," or "this closure can be replaced by a method reference." | Java: SpotBugs / Error Prone + Checkstyle. Clippy is more aggressive — it catches idiomatic improvements, not just bugs. | 1 |
| **Testing** | `#[test]` functions in `tests/` module or `#[cfg(test)] mod tests`. Integration tests live in `tests/` directory (access to public API only). Unit tests go alongside code. Test files run as separate binaries — each test file is its own compilation unit. | Java: JUnit + Mockito. Rust tests are simpler (no framework) and integrated with Cargo's test runner (`cargo test`). No need for mockito — dependency injection works naturally with traits + `dyn Trait` objects. | 2 |
| **Release builds** | `cargo build --release` (enables LTO, inlining, optimization level 3). Binary size matters for distribution. The course uses `--release` for the final artifact but `debug` during development (faster compile + debug info). | Java: `mvn package -P release` with ProGuard/R8 shrinking. Rust's `--release` is simpler — one flag enables all optimizations, and LLVM handles the rest. | 1 |
| **Publishing to crates.io** | `cargo login` (interactive OAuth), `cargo publish`. Requires a crates.io account and API token. The GDK crates (`goose-providers`, etc.) are published this way. Private registries exist for internal libraries. | Java: Maven Central / Sonatype OSSRH publishing. Much more complex (PGP signing, staging repositories). Cargo is simpler — one command after configuration. | 1 |

#### Hands-on exercise (implement in `exercises/m07-cli-hygiene/`)

1. Write a CLI tool with hand-rolled arg parsing (`env::args_os()`) that reads a config file and prints its contents
2. Add tests for the argument parsing and config loading functions
3. Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` — ensure everything passes with zero warnings
4. Build with `--release` and compare binary sizes (`target/debug/` vs `target/release/`)

---

### Milestone 8 — Teaching Preparation: Connecting Rust Concepts to Course Lessons

**Why this matters:** Now you understand the Rust mechanics. This milestone maps each concept to the specific course lessons so you can anticipate student confusion and explain things confidently.

| Course lesson | Key Rust concepts students will struggle with | Your prep focus |
|---------------|----------------------------------------------|-----------------|
| **1 — Bootstrap** | Cargo.toml structure, edition pinning, rust-version, what `cargo check` actually does vs `cargo build` | Demo the full workflow live: `cargo init → cargo add → cargo check → cargo run`. Show that Rust compiles much faster than Java for small projects. |
| **2 — Provider smoke test** | `Result`, `?`, `Box<dyn Error>`, async in main, why `main()` is async | Explain: "The provider call is network I/O — it's non-blocking by design. Tokio's `#[tokio::main]` creates the event loop that runs our async code." Practice reading error messages from borrow checker failures. |
| **3 — Streaming call** | Streams, `StreamExt::next()`, `transpose()`, `while let` with `Option<Result>` | Students will ask "why does `.await` return an `Option`?" Answer: `None` means the stream ended (no more items). `Some(Ok(item))` = next value. `Some(Err(e))` = error. This is how Rust encodes iteration end + errors in a single type. |
| **4 — Conversation** | Ownership of `Vec<Message>`, `&dyn Provider`, returning owned data (`String`) from async functions | The `text_parts.concat()` pattern shows where students may wonder about allocation. Explain: "Each stream fragment is small; we collect them in a Vec<String>, then concatenate into one String at the end." |

#### What to practice before teaching each lesson

For every lesson, build the exercise yourself first. Then:
1. Write down three questions a student *will* ask (they always differ from what you expect)
2. Practice the exact `cargo` commands for that lesson
3. Have an alternate provider configuration ready (in case the demo endpoint is unavailable)

---

## 7. Recommended Book Reading Order (from your local collection)

Your books are located at `/home/stpousty/Downloads/rust-books/`:

| Book | Chapters to read | Why | When |
|------|-----------------|-----|------|
| **Programming Rust** (Blandy, O'Reilly) | Ch. 3 "Ownership", Ch. 4 "Borrowing", Ch. 12 "Errors", Ch. 36+ "Async Programming" | The definitive reference. Read Ch. 3–4 for the mental model shift; skip to Ch. 12 for error handling; read async chapter when doing Milestone 4. Skip deep compiler chapters — not needed for teaching. | Weeks 1–2 (Ch. 3–4), Week 3 (Ch. 12), Week 4 (Async) |
| **Effective Rust** (Drysdale, O'Reilly) | Read entire book, prioritizing Ch. 1 "Types" (error handling + anyhow/thiserror recommendations) and Items 27–35 (CI/hygiene, macros, serde context) | Collection of idioms — "don't do X, do Y instead." Ch. 1 is the authoritative source on the anyhow vs thiserror distinction (explicitly recommends both crates). Chapters on derive macros and CI/hygiene map to Milestone 7 & Serde patterns. | Weeks 3–5 (read in parallel with hands-on) |
| **Async Rust** (Flitton) | Ch. 2 "Basic Async Rust", Ch. 5 "Coroutines" (definitive Pin/BoxStream source), plus Streams chapter | Directly relevant to streaming lessons. Best single source for understanding the event loop, stream consumption patterns, AND demystifying Pin<Box<dyn Stream>>. Read Ch. 2–4 progressively before tackling Ch. 5. | Week 4 (during Milestone 4) |
| **Command-Line Rust** (Youens-Clark) | Chapters on argument parsing, config file handling, error handling in CLIs, and anyhow/thiserror usage patterns across all chapters | Specifically targets CLI apps — maps directly to planned Lesson 8. All chapters use `anyhow::{anyhow, bail, Result}` pattern extensively (68+ occurrences across ch06–ch14), making it the best practical reference for application error handling with context wrapping. | Week 5–6 (during Milestone 7) |
| **Rust in Action** (McNamara) | Ch. 2 "Memory model", Ch. 4 "Concurrency" | Systems concepts that explain *why* Rust's ownership model exists. Useful for answering "why not GC?" type questions. | As needed (reference during Milestone 2) |
| **Rust Servers, Services, and Apps** (Eshwarla) | Skip for now | Not relevant to agentic CLI apps. Revisit if you ever need web server knowledge. | — |

---

## 8. Suggested Pacing

Assuming ~10 hours/week of focused study:

| Week | Focus | Deliverable |
|------|-------|-------------|
| **1–2** | Milestone 1 (Cargo) + start Milestone 2 (Ownership) | `exercises/m01-cargo/` complete; ownership exercises started |
| **3** | Finish Milestone 2 (Ownership/Lifetimes) + Milestone 3 (Errors) | Can read/write `Result`, `?`, `match`, `Option`; understand borrow errors |
| **4** | Milestone 4 (Async/Tokio) | Comfortable reading async code, understanding streams |
| **5** | Milestones 5–6 (Traits + Macros) | Recognize all patterns in course code; understand what macros expand to |
| **6** | Milestones 7–8 (CLI/Hygiene + Teaching Prep) | Can explain every line of lessons 1–4; anticipate student errors |

Adjust based on your actual pace. Ownership/Lifetimes (Milestone 2) often takes longer than expected — that's normal.

---

## 9. Key Mental Model Shifts: Java → Rust

| Concept | Java mindset | Rust mindset |
|---------|-------------|--------------|
| Object ownership | Objects are GC-managed references. Any method can hold a reference to any object indefinitely. | Values have exactly one owner. When the owner drops, the value is deallocated immediately. |
| Aliasing + mutation | Multiple mutable references to the same object is normal (and dangerous). | Either one `&mut` (exclusive) OR many `&` (shared), never both. Compiler enforces this. |
| Null safety | `null` is everywhere. NPEs are common runtime failures. | `Option<T>` — the compiler forces you to handle "no value" at compile time. No nulls exist. |
| Error handling | Checked exceptions or unchecked runtime exceptions. Easy to ignore errors. | `Result<T, E>` — every fallible function's return type makes errors explicit. `?` propagates; you can't forget. |
| Abstraction | Classes + inheritance + interfaces. | Structs (data only) + traits (behavior only) + composition. No inheritance hierarchy. |
| String handling | One string type (`java.lang.String`). Immutable, reference-counted under the hood in some JVMs. | Two types: `String` (owned, heap-allocated) and `&str` (borrowed slice). Zero-cost abstraction. |
| Concurrency | Threads + shared mutable state + locks. Data races are possible. | Ownership + borrowing prevent data races at compile time. `Send` and `Sync` traits enforce thread safety. |
| Code generation | Annotations processed by Lombok/Spring AOP / annotation processors. | Macros run at compile time — declarative (`macro_rules!`), attribute-based (`#[tokio::main]`), or derive-based (`#[derive(Debug)]`). |

---

## 10. Cargo Quick Reference Card

```bash
# Project setup
cargo init myproject                    # Initialize new project in current dir
cargo new myproject                     # Create new project in a new directory
cargo add serde_json tokio@1 --features "macros,rt-multi-thread"  # Add dependency with features
cargo update -p package_name            # Update single dependency

# Build & run
cargo check                             # Type-check only (fastest — use for CI)
cargo build                             # Compile (artifacts in target/debug/)
cargo build --release                   # Optimized build (artifacts in target/release/)
cargo run                               # Build + execute in one step
cargo clean                             # Delete target/ directory

# Code quality
cargo fmt --check                       # Verify formatting matches canonical style
cargo fmt                               # Fix formatting
cargo clippy --all-targets -- -D warnings  # Lint: all warnings become errors

# Tests
cargo test                              # Run all tests (unit + integration)
cargo test --lib                        # Library tests only
cargo test --doc                        # Doc tests only
cargo test my_function_name             # Run specific test

# Documentation
cargo doc --open                        # Build and open local docs in browser
cargo doc --no-deps                     # Docs for this crate only (faster)

# Publishing (crates.io)
cargo login                             # Authenticate with crates.io
cargo publish                           # Publish current version
cargo publish --dry-run                 # Simulate publish without actually publishing
```

---

## 11. What This Plan Does NOT Cover (Yet)

These are advanced topics you can learn later, once you've taught the course:

- **Unsafe Rust** (`unsafe` blocks, raw pointers, FFI) — not needed for GDK apps
- **Complex lifetime annotations** — ~95% of Rust code never writes `'a` explicitly
- **Advanced trait patterns** (associated types vs generics, impl Trait in return position) — useful for library authors, not app developers
- **Zero-cost abstraction internals** (how monomorphization works, vtable layout) — interesting but not required for teaching
- **Async advanced patterns** (task spawning, channels, select! macro) — only relevant for multi-task concurrent programs
- **Build scripts & custom procedural macros** — relevant for library authors, not CLI app developers

---

## 12. Next Session Plan

When you open a new chat with this README, you'll say something like:

> "I've read the learning plan in README.md. Let's start Milestone 1 — create the Cargo exercises in `exercises/m01-cargo/`."

The next session will implement the hands-on exercises for each milestone, starting with cargo basics and progressing through the full roadmap. Each exercise directory will contain a working Rust project that demonstrates the concepts covered in that milestone. **Note:** Milestone 3 now includes Serde attributes (3a) and anyhow/thiserror error context (3b) exercises; Milestone 4 includes Pin/BoxStream trait exercises (4a). See `enhancement-mapping.md` for detailed book-to-enhancement mappings.

---

*Last updated: 2026-09-24*
