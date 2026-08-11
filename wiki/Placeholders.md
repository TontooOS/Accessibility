# Placeholders

Translation values can contain placeholders that are replaced with runtime arguments
at translation time. This is used for dynamic content such as user names, counts or
error reasons.

## Syntax

A placeholder is the placeholder name wrapped in `%` characters:

```
%name%
```

The placeholder map passed to a translation call maps names to replacement strings.
Every occurrence of `%name%` in the value is replaced with the mapped value.

## Formatting

The substitution is implemented in `apply_placeholders`:

```rust
fn apply_placeholders(template: &str, args: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in args {
        let pattern = format!("%{}%", key);
        result = result.replace(&pattern, value);
    }
    result
}
```

- `%` was chosen over `{name}`-style syntax so the replacement is a plain string
  operation without brace parsing.
- Unmapped placeholders are left in the string unchanged.
- Because of this syntax, `%` is **forbidden** inside language codes and file names
  (`verify_no_percent`) – otherwise file names could clash with the placeholder
  format. See [Validation.md](Validation.md).

## Example

File: `lang/en_us.json`

```json
{
  "lang": "en_us",
  "translations": {
    "message.welcome": "Welcome, %name%!",
    "error.invalid_input": "Invalid input: %reason%"
  }
}
```

Rust:

```rust
let args = HashMap::from([("name".to_string(), "Tontoo".to_string())]);

let store = LangStore::instance();
let msg = store.t("en_us", "message.welcome", Some(&args));
// -> "Welcome, Tontoo!"
```

## Usage Points

Placeholders are applied whenever arguments are passed:

- `LangFile::t(key, Some(args))` – [LangFile.md](LangFile.md)
- `LangStore::t(lang, key, Some(args))` – [LangStore.md](LangStore.md)
- `t!(lang, key, args)` macro form – [TMacro.md](TMacro.md)

The C FFI (`accessibility_translate`) currently translates without arguments, so
placeholder strings are returned untouched.
