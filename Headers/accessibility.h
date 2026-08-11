/*
 * TontooAccessibility - C Header
 * TontooOS Internationalization Framework
 *
 * This header provides C bindings for the Accessibility library.
 */

#ifndef TONTOO_ACCESSIBILITY_H
#define TONTOO_ACCESSIBILITY_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Initialize the library with a fallback language.
 *
 * @param fallback The fallback language code (e.g., "en_us")
 * @return 0 on success, negative on error
 */
int accessibility_init(const char *fallback);

/**
 * Translate a key using the specified language.
 *
 * @param lang The language code (e.g., "en_us", "de_de")
 * @param key The translation key
 * @return The translated string (must be freed with accessibility_free_string)
 */
char *accessibility_translate(const char *lang, const char *key);

/**
 * Get the library version string.
 *
 * @return The version string (do NOT free)
 */
const char *accessibility_version(void);

/**
 * Free a string previously returned by accessibility_translate.
 *
 * @param ptr The string to free
 */
void accessibility_free_string(char *ptr);

/**
 * Get the number of loaded languages.
 *
 * @return The number of languages
 */
int accessibility_lang_count(void);

/**
 * Get the language code at the given index.
 *
 * @param index The index
 * @return The language code (do NOT free), or NULL if out of bounds
 */
const char *accessibility_lang_at(int index);

#ifdef __cplusplus
}
#endif

#endif /* TONTOO_ACCESSIBILITY_H */
