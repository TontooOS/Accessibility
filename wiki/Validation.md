# Validation & Errors

This page covers the validation functions and the error type of the library.

## `LangError`

Every fallible operation in the library returns `Result<_, LangError>`.

```rust
#[derive(Debug)]
pub enum LangError {
    Io(std::io::Error),
    Json(String),
    Lang(String),
}
```

| Variant | Source |
|---|---|
| `Io` | File system errors (reading, writing, creating directories) |
| `Json` | JSON deserialization/serialization errors |
| `Lang` | Semantic validation errors, e.g. `"Root must be an object"`, `"Missing 'lang' field"`, `"Invalid lang name: only [a-z_] allowed"` |

`LangError` implements `Display` (`"IO error: ..."`, `"JSON error: ..."`,
`"Lang error: ..."`) and `std::error::Error`, and converts automatically from
`std::io::Error` and `foundation::error::FoundationError` via `From`.
JSON parsing and serialization run through Foundation's `JSONSerialization` API,
so `accessibility` has no direct `serde_json` dependency.

## Validation Functions

### `validate_lang_file(path)`

Validates a single JSON language file without loading it. Parsing uses
Foundation's `JSONSerialization` API:

1. The document must be valid JSON (`JSONSerialization::is_valid_json`).
2. It must deserialize into a `LangFile` (`JSONSerialization::from_string`),
   which enforces the `lang` and `translations` fields.
3. The `lang` code is validated via `validate_lang_name`.
4. Every translation key must be non-empty.

Returns `Ok(())` when valid, or a `LangError` describing the problem.

### `validate_lang_dir(dir)`

Runs `validate_lang_file` on every `.json` file in a directory and collects all
problems. Does not fail on invalid files.

- **Returns:** `Ok(Vec<(path, error_message)>)` – one tuple per invalid file.
- Returns `Err(LangError)` only on directory read errors.

```rust
let errors = validate_lang_dir("./lang")?;
for (path, msg) in errors {
    eprintln!("{}: {}", path, msg);
}
```

### `validate_lang_name(lang)`

- Rejects codes containing `%` (via `verify_no_percent`).
- Rejects codes with any character other than lowercase ASCII letters and `_`.

Allowed: `en_us`, `de_de`. Rejected: `En_US`, `en-US`, `fr2`.

### `verify_no_percent(value)`

Rejects any string containing `%`. Used for language codes and file names to protect
the placeholder syntax (see [Placeholders.md](Placeholders.md)).

### `assert_valid(files, fallback)`

Used by `LangStore::init` and `LangStore::add`:

- Validates the language name of every file.
- Validates the fallback name.
- **Fails** if the fallback is not present among the loaded files.

```rust
LangStore::init(files, Some("en_us".to_string()));
// Err("Invalid fallback lang: en_us") if en_us is not in files
```

See also [LangStore.md](LangStore.md).
