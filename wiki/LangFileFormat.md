# Language File Format

Language files are JSON documents that live in the `lang/` directory of the embedding
application. One file per language, named after the language code
(e.g. `en_us.json`, `de_de.json`).

## Structure

A valid file has exactly two top-level fields:

| Field | Type | Description |
|---|---|---|
| `lang` | `string` | The language code, e.g. `"en_us"`. Must match `[a-z_]+` and must not contain `%`. |
| `translations` | `object` | Map of non-empty translation keys to string values. |

## Example

```json
{
  "lang": "en_us",
  "translations": {
    "app.title": "My Application",
    "app.description": "A Rust application with localization",
    "button.submit": "Submit",
    "button.cancel": "Cancel",
    "message.welcome": "Welcome, {name}!",
    "message.goodbye": "Goodbye, {name}!",
    "error.not_found": "Resource not found",
    "error.invalid_input": "Invalid input: {reason}"
  }
}
```

## Naming Conventions

- **Keys** are dot-namespaced: `section.name` (e.g. `app.title`, `button.ok`,
  `error.not_found`). Keys must not be empty.
- **Values** may contain placeholders written as `%name%` which are replaced at
  translation time. See [Placeholders.md](Placeholders.md).
- **Language codes** are lowercase with an underscore: `en_us`, `de_de`. Only the
  characters `[a-z_]` are allowed, and `%` is forbidden (it would break the
  placeholder system).

## Loading

Files are loaded from the `lang/` directory by `init_lang` at startup. Loading a file
deserializes it directly into a `LangFile` (see [LangFile.md](LangFile.md)) and rejects
the file if its `lang` code contains `%`. Structural validation happens separately via
`validate_lang_file` (see [Validation.md](Validation.md)).
