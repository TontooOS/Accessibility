# Init & Setup

Two functions handle bootstrapping the library and scaffolding a new embedding project.

## `init_lang(fallback)`

Loads every `*.json` file from the resolved `lang/` directory and initializes
the global `LangStore`.

```rust
pub fn init_lang(fallback: &str) -> Result<(), LangError>
```

```rust
pub fn resolve_lang_dir() -> std::path::PathBuf
```

Resolution order for the directory (`resolve_lang_dir`):

1. `$ACCESSIBILITY_LANG_DIR` override.
2. LiveOS sidecar: `/Library/System/accessibility.resources/lang`.
3. Staged sources: `/Library/System/accessibility/lang`.
4. Relative crate dir: `./lang` (dev / `cargo run`).

Behavior:

1. If the resolved dir does not exist, it is created.
2. Every `.json` file inside is parsed via `LangFile::from_file` and collected.
   Invalid files are skipped silently.
3. `LangStore::init(files, Some(fallback))` replaces the store contents and validates
   the fallback.

`LangFile::file_path` points into the same resolved directory, so `write()`
saves next to the loaded files.

This should be called **once at application startup**, before any translation.

```rust
fn main() {
    accessibility::init_lang("en_us").expect("failed to init languages");
}
```

## `setup_lang_dir(dest)`

Scaffolds a fresh language setup for an embedding project. Creates the target directory
(if missing) and writes two example files with the same key set:

| File | Content |
|---|---|
| `en_us.json` | `app.title`, `button.ok`, `button.cancel` in English |
| `de_de.json` | Same keys in German |

```rust
setup_lang_dir("./lang")?;
// creates ./lang/en_us.json and ./lang/de_de.json
```

Both files are generated with `LangFile::new` and written as pretty-printed JSON via
`write()` (see [LangFile.md](LangFile.md)). Use this as a starting point for new apps.

## Recommended Startup Sequence

1. `setup_lang_dir("./lang")` – once, when creating a new app (or ship the files by hand).
2. `init_lang("en_us")` – every startup.
3. Translate with `t!` or the store, see [TMacro.md](TMacro.md).
