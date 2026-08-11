# C FFI

The library exports C-compatible symbols so non-Rust programs (C, C++, Python via
ctypes, ...) can use the framework. The bindings are declared in
`Headers/accessibility.h`.

All FFI calls are serialized behind a global mutex (`LANG_MUTEX`), so concurrent use
from multiple threads is safe at the call level.

## Functions

### `int accessibility_init(const char *fallback)`

Initializes the library by loading `lang/*.json` from the working directory.

| Return | Meaning |
|---|---|
| `0` | Success |
| `-1` | `fallback` pointer is NULL |
| `-2` | `fallback` is not a valid UTF-8 C string |
| `-3` | Loading/validation failed |

```c
if (accessibility_init("en_us") != 0) { /* error */ }
```

### `char *accessibility_translate(const char *lang, const char *key)`

Translates a key in the given language. Returns a heap-allocated string that must be
freed with `accessibility_free_string`. Falls back to the key itself when untranslatable.

- Returns `NULL` if `lang` or `key` is NULL or invalid UTF-8.
- No argument (placeholder) support currently.

```c
char *msg = accessibility_translate("en_us", "app.title");
printf("%s\n", msg);
accessibility_free_string(msg);
```

### `const char *accessibility_version(void)`

Returns the static version string `"26.1"`. **Do not free.**

### `void accessibility_free_string(char *ptr)`

Frees a string previously returned by `accessibility_translate`. Safe to call with
`NULL`.

### `int accessibility_lang_count(void)`

Returns the number of loaded languages.

### `const char *accessibility_lang_at(int index)`

Returns the language code at the given index.

- Returns `NULL` if `index` is negative or out of bounds.
- The returned string is **static** – do not free.

```c
int n = accessibility_lang_count();
for (int i = 0; i < n; i++) {
    printf("%s\n", accessibility_lang_at(i));
}
```

## Memory Rules

| Source | Ownership |
|---|---|
| `accessibility_translate` | Caller must free with `accessibility_free_string` |
| `accessibility_version` | Static, must NOT be freed |
| `accessibility_lang_at` | Static, must NOT be freed |

## C Header

The canonical header lives at `Headers/accessibility.h` and uses the `extern "C"`
guard, so it can be included from both C and C++:

```c
#include "accessibility.h"
```
