# Milestone 2, Lesson 2: Ownership, Borrowing, and the JSON Round Trip

## Purpose

This lesson completes the program promised by **Next lesson** at the end of Milestone 1: an async `main` defines a serializable struct, serializes it to JSON, deserializes it again, and prints both forms. It also uses that small program to introduce the ownership rules that make Rust different from Java.

By the end, you should be able to explain why assigning a `String` moves it, choose between `String` and `&str` in a function signature, and read the three most common borrow-checker diagnostics without treating them as mysterious compiler failures.

---

## Instructor notes: goals and lesson-design rationale

Keep this section with the lesson. It records **why this lesson is shaped this way**, so later questions can be answered consistently rather than treating each Rust rule as an isolated fact.

### Overall course goal

This repository is preparation for teaching people who primarily know Python (and for an instructor coming from Java) to build a native Rust CLI with the Goose Development Kit. The goal is **just enough Rust to read, explain, and safely adapt the course code**—not a comprehensive systems-programming curriculum.

The eventual application must construct provider requests, represent native messages, serialize data as JSON, run asynchronous inference, maintain conversation history, and expose CLI/configuration behavior. Students will repeatedly encounter owned values, borrowed values, `String`/`&str`, `Result`, async code, traits, and macros while doing that work. Ownership is placed early because it is the model that makes the later compiler messages and API signatures intelligible.

### What this lesson deliberately connects

This is both the second course lesson and the first focused ownership milestone. It joins two otherwise separate needs in one small, runnable program:

1. **Complete the contract from Lesson 1.** The prior lesson ended by promising an async JSON round trip using Serde, `serde_json`, Tokio, Cargo, and IDEA. Starting from that program makes Lesson 1's dependency and Cargo discussion concrete instead of leaving it as setup with no payoff.
2. **Use a believable agent-domain value.** `AgentMessage { role, content }` resembles provider/message data students will handle later. It is intentionally small: its two owned `String` fields make moving, borrowing, cloning, serializing, deserializing, and dropping visible without provider SDK complexity.
3. **Introduce ownership through observable behavior.** The program moves `original` to `owner`, creates a deliberately independent `text` with `.clone()`, borrows values for read and mutation, and prints from `Drop`. Each abstract rule therefore has a line of code and runtime or compiler evidence attached to it.
4. **Keep error handling intentionally shallow.** `?` appears only because the real JSON APIs are fallible. This lesson gives its plain-English meaning, but defers error types, `Result`, `Option`, `match`, `anyhow`, and `thiserror` to Milestone 3. Do not let an ownership lesson become an error-handling lesson.

### Pedagogical sequence

The ordering is intentional:

| Order | Concept | Why it comes here |
|---|---|---|
| 1 | Run the JSON round trip | Gives students a working, relevant baseline and fulfills the prior lesson's stated next step. |
| 2 | Move versus `Copy` | Explains why `let b = a` behaves differently for `String` and `i32`; this is the essential Java/Python mental-model break. |
| 3 | Borrow (`&T`) and mutably borrow (`&mut T`) | Gives the normal alternative to copying or transferring ownership and explains the function signatures students see everywhere. |
| 4 | Practice three compiler errors | Converts borrow-checker diagnostics into expected feedback, not a reason to panic or blindly clone. |
| 5 | `String` versus `&str` and a light lifetime introduction | Lets students choose practical string APIs while postponing advanced annotation detail. |
| 6 | `Drop`, then recognition-only `Box`/`Rc` | Connects ownership to deterministic cleanup and prepares students to recognize later type signatures without overloading the first lesson. |

### Deliberate scope boundaries

Teach and practice these now:

- A non-`Copy` value has one owner; assignment, argument passing, and return can move it.
- `&T` permits shared reading; `&mut T` permits exclusive mutation; the compiler enforces either-many-readers-or-one-writer.
- A function should normally borrow when it only needs temporary access, take ownership when it must retain/transform the value, and clone only when a genuinely independent owned value is required.
- `String` owns text; `&str` views text without owning it; returning a freshly constructed value should normally return `String`.
- Scope exit and `drop(value)` perform deterministic cleanup.

Do **not** attempt to teach these in depth here:

- lifetime-elision rules, complex multi-reference lifetime design, self-referential types, or `'static` beyond recognizing a string literal;
- interior mutability (`RefCell`, `Cell`, mutexes), `Arc`, thread-safety traits (`Send`/`Sync`), or concurrent ownership;
- smart-pointer implementation details, reference-count cycles, or manual memory management;
- the provider SDK's full message types or network calls;
- robust error architecture. Those are later milestones.

The simplification is deliberate. A student should first be able to predict whether code moves, borrows, mutates, or drops a value. Only then should later lessons add async/concurrency, traits, dynamic dispatch, and richer provider types.

### Choices in the example code

- `AgentMessage` uses owned `String` fields rather than borrowed `&str` fields. Real message objects commonly need to survive beyond the call that created their text; owned fields avoid introducing struct lifetime parameters before students need them.
- `serde_json::to_string(&owner)` borrows the message so it remains usable; `from_str(&json)` borrows JSON input but creates a fresh owned message. This contrast demonstrates that an API's signature—not a blanket rule about serialization—determines whether ownership moves.
- `let mut text = owner.content.clone()` is intentionally explicit. It demonstrates a valid reason to duplicate data: the mutable text must diverge from the message's original content. It is not an endorsement of cloning to silence every borrow-checker error.
- `describe(&str)` models the preferred read-only text API. The exercise specification also calls for observing `&String`; discuss it as a more restrictive borrowed parameter that works for a `String` but rejects a literal or arbitrary string slice. Prefer `&str` unless the API specifically requires `String` methods or ownership layout.
- `append_period(&mut String)` is intentionally more specific because it calls `push`; it demonstrates mutation of an owned growable string. Later, introduce generic alternatives only when they solve a real problem.
- The `Drop` implementation only prints. Real `Drop` implementations release resources such as file handles, sockets, locks, and transactions; printing is solely a safe way to observe timing.
- `#[tokio::main]` is retained to fulfill Lesson 1 and match the async shape of the future GDK CLI. There is no `.await` yet, so Tokio is not the subject of this lesson; async runtime behavior belongs to Milestone 4.

### Anticipated questions and concise teaching answers

| Likely question | Answer to anchor the discussion |
|---|---|
| “Is a move copying the object?” | No. For a non-`Copy` value, it transfers the one ownership responsibility. Rust then rejects use through the old binding so the allocation is freed exactly once. |
| “Why not automatically copy like Java references?” | Java copies a reference while the garbage collector retains ownership of the object. Rust has no tracing GC here, so it makes ownership and sharing explicit and verifies safety at compile time. |
| “Why doesn't Rust just clone automatically?” | Cloning can allocate and be expensive, and it can change program semantics. Rust requires the programmer to make that independent copy visible with `.clone()`. |
| “Why can Serde borrow when `AgentMessage` owns its strings?” | The serializer only reads the message during the call, so its API accepts a borrow. The deserializer creates a new value and returns ownership of it. |
| “Does `&mut` mean the object is permanently mutable?” | No. It is a temporary, exclusive permission. The binding must be mutable to create it, and the borrow ends after its last use. |
| “Why does `&str` accept a `String`?” | A `String` can be borrowed as a string slice through deref coercion. `&str` is therefore a flexible read-only input type for both literals and owned strings. |
| “Should I return `&str` to avoid allocating?” | Only if the returned text already lives inside an input or another value that will outlive the call. Freshly formatted text must be returned as an owned `String`. |
| “Does `drop` replace garbage collection?” | `drop` is deterministic cleanup of an owned value. Rust values are ordinarily cleaned up automatically at scope exit; explicit `drop` is only needed to release something earlier. |
| “Why are Box and Rc not used in the program?” | They solve special data-shape or shared-ownership needs. Ordinary ownership and borrowing are simpler and should be the default. |

### Success criterion before moving on

Do not judge success by memorizing terminology or by being able to recite lifetime syntax. A learner is ready for Milestone 3 when they can look at an unfamiliar function call and explain:

1. which value owns each piece of data before and after the call;
2. whether an argument is moved, immutably borrowed, or mutably borrowed;
3. whether a clone is necessary or merely avoiding thought; and
4. why the compiler accepts or rejects the overlapping accesses.

---

## Prerequisite: the Milestone 1 setup

This exercise is a separate Cargo package at `exercises/m02-ownership/`. It intentionally uses the same dependencies that the previous lesson asked you to add:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The dependency roles are unchanged:

| Dependency | Why it is here |
|---|---|
| `serde` with `derive` | Provides the `Serialize` and `Deserialize` traits and their derive macros. |
| `serde_json` | Converts Rust values to and from JSON text. |
| `tokio` | Provides the runtime used by `#[tokio::main]`. This example does not await work yet, but the async entry point prepares the shape used in later agent applications. |

From the repository root, enter this package and let Cargo resolve the manifest:

```bash
cd exercises/m02-ownership
cargo check
```

Cargo creates `Cargo.lock` the first time it resolves this standalone application. Commit that lockfile with the manifest; do not commit `target/`.

---

## First, finish the promised JSON program

Open `src/main.rs`. Its central data model is deliberately close to a message an agent application might send to a provider:

```rust
#[derive(Debug, Deserialize, Serialize)]
struct AgentMessage {
    role: String,
    content: String,
}
```

`String` means this struct **owns** both pieces of text. `Serialize` lets `serde_json::to_string` turn the struct into JSON; `Deserialize` lets `serde_json::from_str` reconstruct it.

The essential round trip is:

```rust
let json = serde_json::to_string(&owner)?;
println!("Serialized JSON: {json}");

let round_tripped: AgentMessage = serde_json::from_str(&json)?;
println!("{}", acknowledgement(&round_tripped));
```

Notice the different kinds of arguments:

- `&owner` is an immutable borrow. Serialization only needs to inspect the value, so it does not need ownership.
- `&json` is a borrowed string slice accepted by `from_str`; JSON parsing can read the text without taking or changing the `String` that holds it.
- `round_tripped` is a new, owned `AgentMessage` produced by deserialization.
- `?` returns a JSON error from `main` if serialization or deserialization fails. Error handling is the next milestone; for now, read it as “if this fails, return the error.”

Run the completed program:

```bash
cargo fmt
cargo check
cargo build
cargo run
```

The exact JSON field order is an implementation detail, but the output should show a borrowed view, a mutated `String`, JSON text, the deserialized message, and two `Drop` messages. One drop occurs where the program explicitly calls `drop`; the other happens automatically when `owner` leaves `main`.

> **IDEA check:** Open the repository root in IntelliJ IDEA, allow the Rust plugin to load this Cargo project, then use the green gutter icon beside `main`. Compare its output with `cargo run` in IDEA's integrated terminal. The IDE is still delegating compilation and execution to Cargo.

---

## The ownership model

Java variables that contain object values are normally references managed by garbage collection. Assigning one Java reference to another makes another alias to the same object:

```java
String b = a; // a and b refer to the same String object
```

For an owning Rust type such as `String`, an ordinary assignment transfers ownership instead:

```rust
let original = String::from("hello");
let owner = original;

// println!("{original}"); // error: borrow of moved value
println!("{owner}");
```

A `String` contains a pointer, length, and capacity for heap-allocated bytes. Blindly copying those three fields would create two values that both believe they must free the same allocation. Rust prevents the resulting double-free by moving ownership to `owner` and making `original` unavailable.

This is the line in the exercise:

```rust
let owner = original;
```

Uncomment the indicated `println!` in `src/main.rs`, run `cargo check`, read the compiler message, then restore the comment. The compiler tells you where the value moved and suggests how you might clone it when an independent copy is actually wanted.

### `Copy` types are different

Small types whose values can be duplicated safely and cheaply implement `Copy`:

```rust
let first: i32 = 7;
let second = first;
println!("{first}, {second}"); // valid: `i32` is Copy
```

Common examples are integers, `bool`, floating-point values, and tuples made entirely of `Copy` values. `String`, `Vec<T>`, file handles, and most resource-owning structs are not `Copy`.

### Clone only when you want another owned value

The exercise contains this explicit clone:

```rust
let mut text = owner.content.clone();
```

Here a separate, mutable string is useful: changing `text` must not change `owner.content`. `.clone()` allocates and copies the text. That cost is intentional and visible in the source; it is not an automatic side effect of assignment.

---

## Borrow instead of moving

Most functions do not need to take ownership. They can borrow a value for the duration of a call.

```rust
fn describe(message: &str) {
    println!("Borrowed view: {message}");
}

fn append_period(message: &mut String) {
    if !message.ends_with('.') {
        message.push('.');
    }
}
```

Call them like this:

```rust
let mut text = String::from("Hello");
describe(&text);          // immutable borrow: read only
append_period(&mut text); // mutable borrow: may change `text`
println!("{text}");      // valid after the mutable borrow ends
```

The `&` in `&text` creates an immutable borrow (`&T`). The `&mut` in `&mut text` creates an exclusive mutable borrow (`&mut T`). A borrow is not a second owner; it is temporary permission to access an owner’s value.

### The rule to memorize

At any one time, for one value, Rust permits either:

- any number of immutable borrows (`&T`), **or**
- exactly one mutable borrow (`&mut T`).

It never permits mutable and immutable borrows whose usable lifetimes overlap. This makes it impossible for one part of a program to read a value while another part changes it through an alias.

### Practice the three common diagnostics

Make each temporary edit below, run `cargo check`, read the diagnostic, then undo the edit.

#### 1. Cannot move out of borrowed content

```rust
fn take_content(message: &AgentMessage) -> String {
    message.content
}
```

This fails because `message` is only borrowed and `content` is an owned `String`; moving it out would leave a partially emptied struct behind someone else’s reference. Fix it according to the desired API:

```rust
fn view_content(message: &AgentMessage) -> &str {
    &message.content
}

fn copy_content(message: &AgentMessage) -> String {
    message.content.clone()
}

fn take_message_content(message: AgentMessage) -> String {
    message.content
}
```

The first borrows, the second allocates a separate copy, and the third consumes the whole message so moving its field is safe.

#### 2. Cannot borrow as mutable more than once

```rust
let mut text = String::from("hello");
let first = &mut text;
let second = &mut text; // error
println!("{first} {second}");
```

Two simultaneous writers would be aliases to the same data. End the first borrow before making the second, usually by using `first` and then letting it go out of scope:

```rust
let mut text = String::from("hello");
{
    let first = &mut text;
    first.push('!');
}
let second = &mut text;
second.push('?');
```

#### 3. Cannot borrow as immutable while a mutable borrow is active

```rust
let mut text = String::from("hello");
let writer = &mut text;
let reader = &text; // error while `writer` is still used
writer.push('!');
println!("{reader}");
```

Again, make the mutable borrow finish before creating the reader:

```rust
let mut text = String::from("hello");
{
    let writer = &mut text;
    writer.push('!');
}
let reader = &text;
println!("{reader}");
```

Rust's non-lexical lifetimes often end a borrow at its last actual use rather than the closing brace, but a small scope is a clear teaching tool when you want to make that boundary explicit.

---

## `String` versus `&str`

`String` is an owned, heap-allocated, growable UTF-8 string. It owns its characters and can be mutated if the binding is declared `mut`.

`&str` is a borrowed UTF-8 string slice: a view consisting of a pointer and a length into text owned elsewhere. It does not allocate or own the characters.

```rust
let owned = String::from("agent");
let slice: &str = &owned;
let literal: &str = "agent";
```

A string literal has type `&'static str`: its bytes are embedded in the program and live for the entire program. A slice taken from `owned` cannot outlive `owned`.

Prefer `&str` for input when a function only needs to read text. It accepts both literals and borrowed `String` values through deref coercion:

```rust
fn label(text: &str) -> String {
    format!("message: {text}")
}

let from_literal = label("hello");
let from_string = label(&String::from("hello"));
```

Return `String` when the function constructs and gives the caller text it should own. The exercise’s `acknowledgement` does exactly that:

```rust
fn acknowledgement(message: &AgentMessage) -> String {
    format!("Received {} message: {}", message.role, message.content)
}
```

The returned text is newly created, so returning an owned `String` avoids lifetimes entirely. Do **not** return `&str` pointing at a temporary `format!` result; that temporary would be dropped before the caller could use the reference.

---

## Lifetimes: the practical first view

A lifetime is Rust’s compile-time proof that a reference never outlives the value it points to. Most ordinary code needs no written annotation because the compiler can infer it:

```rust
fn describe(message: &str) {
    println!("{message}");
}
```

The `message` borrow clearly only needs to last for the call. The `&str` returned by this function is tied to the input, so Rust needs an explicit relationship only in cases such as choosing between two input references:

```rust
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
```

Read `'a` as a label saying: “the returned slice is valid no longer than both applicable input slices.” It does not make data live longer, allocate anything, or mean the value is static. For now, use owned return values such as `String` whenever a borrowed return value is not clearly needed.

---

## `Drop` and deterministic cleanup

Rust runs cleanup when an owned value goes out of scope. This is RAII (resource acquisition is initialization), familiar from C++ but unlike Java’s nondeterministic garbage collection.

The exercise implements `Drop` so that cleanup is visible:

```rust
impl Drop for AgentMessage {
    fn drop(&mut self) {
        println!("Dropping message from {:?}", self.role);
    }
}
```

Near the end of `main`:

```rust
drop(round_tripped);
println!("The explicit drop above ran before the end of main.");
```

Calling `drop` consumes `round_tripped`, so it cannot be used afterwards. Its cleanup happens at that exact line. `owner` is not explicitly dropped; Rust automatically drops it when `main` ends.

In real applications, this deterministic cleanup closes files, sockets, locks, database transactions, and other resources—even when a function returns early because of `?`. You normally do not implement `Drop` merely to print; this implementation exists to make the timing observable.

---

## Box and Rc: recognize the names

You will encounter these types in Rust code, even though this lesson does not need them.

- `Box<T>` gives one owner a value stored on the heap. It is useful for recursive types, large values, or trait objects. It is still single ownership and drops its contents when the box drops.
- `Rc<T>` gives multiple owners a shared immutable-by-default value through reference counting. It is for single-threaded shared ownership; it does **not** make mutation automatically safe. The thread-safe cousin is `Arc<T>`.

Use ordinary owned values and borrowing first. Reach for `Box`, `Rc`, or `Arc` only when the data shape actually requires heap indirection or multiple owners.

---

## Checkpoint questions

Answer these without running code:

1. Why does `let owner = original;` make an ordinary `String` binding unavailable, but not an `i32` binding?
2. When should a function accept `&str` rather than `String`?
3. Why can `serde_json::to_string(&owner)` borrow `owner` instead of consuming it?
4. State the immutable/mutable borrow rule in one sentence.
5. What are the three legitimate ways to handle a `String` field behind `&AgentMessage`: borrow it, clone it, or do what?
6. Why does `acknowledgement` return `String`, not `&str`?
7. What is the observable difference between `drop(round_tripped)` and allowing a value to reach the end of its scope?

---

## Next lesson

Turn JSON failures and other fallible operations into clear program behavior. You will use `Result<T, E>`, `match`, `?`, `Option`, and error context; then compare a simple `Box<dyn Error>` application boundary with `anyhow` for CLI applications and `thiserror` error enums for reusable provider libraries.
