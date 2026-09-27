# LangStore

`LangStore` holds all loaded languages in a global, thread-safe singleton and performs
language resolution and fallback. Every translation in the system goes through it.

```rust
#[derive(Debug, Clone, Default)]
pub struct LangStore {
    files: HashMap<String, LangFile>,
    fallback: Option<String>,
}
```

## Global Instance

The store is a process-wide singleton behind a `Mutex`:

```rust
static LANG_STORE: std::sync::OnceLock<std::sync::Mutex<LangStore>> =
    std::sync::OnceLock::new();
```

### `LangStore::instance()`

Locks and returns a guard to the global store.

```rust
let store = LangStore::instance();
```

## Initialization

### `init(files, fallback)`

Replaces the store contents with the given files and fallback.

- Validates every language name and the fallback name (see
  [Validation.md](Validation.md)).
- **Fails** if a fallback is given that is not among the loaded files.
- Clears the previous store and inserts all files.

This is what `init_lang` calls at startup. See [InitAndSetup.md](InitAndSetup.md).

### `add(file)`

Inserts a single language file into the store (replacing an existing one with the
same code).

- Rejects files whose language code contains `%`.
- Revalidates the store's fallback; returns `Err` if the fallback no longer exists
  in the store.

## Translating

### `t(lang, key, args)`

Resolves the language code, looks up the key and returns the translated string
(placeholders applied if `args` is `Some`).

- Returns `None` if the key does not exist in the resolved language.

```rust
let store = LangStore::instance();
let title = store.t("de_de", "app.title", None);
```

## Language Resolution

### `resolve_lang(lang)`

Resolves a requested language code against the loaded files in this order:

1. **Exact match** – the requested code exists as a loaded file.
2. **Base-language match** – the part before `_` (e.g. `de` from `de_de`) matches the
   beginning of a loaded code, e.g. `de_at` → `de_de`.
3. **Configured fallback** – the fallback passed to `init`.
4. **Hardcoded default** – `en_us`.

```rust
store.resolve_lang("de_at") // -> "de_de" (if only de_de is loaded)
```

## Introspection

### `fallback()`

Returns the configured fallback language code, or `None`.

### `all_langs()`

Returns the codes of all loaded languages as a `Vec<&str>`.

```rust
let langs = LangStore::instance().all_langs();
```
