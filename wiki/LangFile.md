# LangFile

`LangFile` represents a single language loaded from a JSON file. It is a serializable
struct used both for reading files from disk and for writing them back.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangFile {
    pub lang: String,
    pub translations: HashMap<String, String>,
}
```

## Constructors

### `LangFile::new(lang, translations)`

Creates a `LangFile` in memory from a language code and a map of key/value pairs.

```rust
let mut map = HashMap::new();
map.insert("app.title".to_string(), "My Application".to_string());

let en = LangFile::new("en_us", map);
```

### `LangFile::from_file(path)`

Reads and deserializes a JSON file into a `LangFile`.

- Returns `Err` on I/O or JSON errors.
- Returns `Err` if the `lang` code contains `%` (see `verify_no_percent`).

```rust
let en = LangFile::from_file("./lang/en_us.json")?;
```

## File Operations

### `file_path()`

Returns the conventional path `./lang/<lang>.json` for this file.

### `write()`

Serializes the file to pretty-printed JSON and writes it to `./lang/<lang>.json`.
Fails with an I/O error if the file cannot be written.

```rust
let de = LangFile::new("de_de", map_de);
de.write()?; // writes ./lang/de_de.json
```

## Translation

### `t(key, args)`

Looks up a key in this file and returns the translated string.

- `args: None` returns the raw value.
- `args: Some(map)` replaces every `%key%` placeholder with the corresponding value
  (see [Placeholders.md](Placeholders.md)).
- Returns `None` if the key does not exist.

```rust
let welcome = en.t("message.welcome", Some(&args))?;
```

For the store-wide lookup with fallback logic, see [LangStore.md](LangStore.md).
