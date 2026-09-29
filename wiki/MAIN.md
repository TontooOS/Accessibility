# TontooAccessibility – Wiki

TontooAccessibility is the internationalization (i18n) framework for TontooOS.
It loads JSON translation files from a `lang/` directory, stores them in a
global in-memory store, and provides translations to both Rust code (via the
`t!` macro) and C / C++ / Python programs (via the C FFI layer).

- Repository: https://github.com/TontooOS/Accessibility
- License: TCL v27.0
- Version: 26.1

## Feature Index

| Feature | File | Description |
|---|---|---|
| Main index | [MAIN.md](MAIN.md) | This page |
| Rules | [RULE.md](RULE.md) | Development and usage rules |
| Language file format | [LangFileFormat.md](LangFileFormat.md) | Structure of `lang/*.json` files |
| LangFile | [LangFile.md](LangFile.md) | Loading, writing and translating with a single language file |
| LangStore | [LangStore.md](LangStore.md) | Global store, language resolution and fallback logic |
| Placeholders | [Placeholders.md](Placeholders.md) | `%key%` argument substitution in translations |
| Validation & Errors | [Validation.md](Validation.md) | Validating files, names and the `LangError` type |
| Init & Setup | [InitAndSetup.md](InitAndSetup.md) | Loading the `lang/` directory and scaffolding examples |
| `t!` Macro | [TMacro.md](TMacro.md) | The translation macro with all three forms |
| C FFI | [FFI.md](FFI.md) | C bindings, memory rules and error codes |

## Quick Start

Create a `lang/` folder with one JSON file per language:

```json
{
  "lang": "en_us",
  "translations": {
    "app.title": "My Application",
    "message.welcome": "Welcome, {name}!"
  }
}
```

Initialize the store and translate:

```rust
use accessibility::t;

accessibility::init_lang("en_us")?;

let title = t!("app.title");
let welcome = t!("en_us", "message.welcome", &{ "name".to_string() => "Tontoo".to_string() });
```

From C:

```c
#include "accessibility.h"

accessibility_init("en_us");
char *msg = accessibility_translate("en_us", "app.title");
printf("%s\n", msg);
accessibility_free_string(msg);
```

See [InitAndSetup.md](InitAndSetup.md) and [TMacro.md](TMacro.md) for details.
