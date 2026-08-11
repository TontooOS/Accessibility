# `t!` Macro

The `t!` macro is the ergonomic way to translate from Rust code. It accesses the global
`LangStore` directly, so the store must be initialized first (see
[InitAndSetup.md](InitAndSetup.md)).

```rust
#[macro_export]
macro_rules! t {
    ($key:expr) => { ... };
    ($lang:expr, $key:expr) => { ... };
    ($lang:expr, $key:expr, $args:expr) => { ... };
}
```

All forms fall back to a debug representation of the key (`format!("{:?}", key)`) when
the key cannot be resolved, so missing translations never panic.

## Form 1 – Default Language

```rust
t!("key")
```

Translates with the hardcoded default language `en_us` (no fallback resolution).

```rust
let title = t!("app.title");
```

## Form 2 – Specific Language

```rust
t!("lang", "key")
```

Translates with the given language code, resolved through the store's normal fallback
logic (`resolve_lang`).

```rust
let title = t!("de_de", "app.title");
```

## Form 3 – With Arguments

```rust
t!("lang", "key", args)
```

Translates with a placeholder map applied. The map must be a `&HashMap<String, String>`.

```rust
let args = HashMap::from([("name".to_string(), "Tontoo".to_string())]);
let msg = t!("en_us", "message.welcome", &args);
// -> "Welcome, Tontoo!"
```

See [Placeholders.md](Placeholders.md) for the placeholder syntax.

## Usage Notes

- Use the single-argument form for the system default UI language (currently `en_us`).
- Use the two/three-argument forms when a specific language or dynamic arguments are
  needed.
- The macro never panics; an unresolvable key returns its debug string, which makes
  missing translations visible during development.
