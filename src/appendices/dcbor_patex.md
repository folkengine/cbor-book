# dCBOR Pattern Expression Syntax (_patex_)

This syntax is inspired by regular expressions but is specifically designed for dCBOR.

The pattern syntax is designed to be flexible and expressive. Patterns can be composed of *value patterns*, *structure patterns*, and combinators known as *meta-patterns*.

Keywords like `[patex] bool`, `[patex] number`, etc., are case-sensitive. Patterns can include specific values, ranges, or regexes to match against the corresponding parts of the dCBOR item.

Parentheses are used to group patterns or specify ranges. The syntax `[patex] (pattern)` is really just the repeat pattern with a repeat that matches the pattern exactly once.

White space is ignored between tokens, so you can use it to make patterns more readable. The syntax examples below include white space both to show where it can be used and to show where it *cannot* be used (i.e., between characters of a token like `[patex] *?`)

## Value Patterns

All value patterns match atomic CBOR values.

- Boolean
    - `[patex] bool`
        - Matches any boolean value.
    - `[patex] true`
        - Matches the boolean value `true`.
    - `[patex] false`
        - Matches the boolean value `false`.
- ByteString
    - `[patex] bstr`
        - Matches any byte string.
    - `[patex] h'hex'`
        - Matches a byte string with the specified hex value. Note that the `[patex] h'...'` syntax is used to denote hex strings in CBOR diagnostic notation, so we use it here for familiarity.
    - `[patex] h'/regex/'`
        - Matches a byte string that matches the specified binary regex.
- Date
    - `[patex] date`
        - Matches any date value.
    - `[patex] date'iso-8601'`
        - Matches a date value with the specified ISO 8601 format.
    - `[patex] date'iso-8601...iso-8601'`
        - Matches a date value within the specified range.
    - `[patex] date'iso-8601...'`
        - Matches a date value greater than or equal to the specified ISO 8601 date.
    - `[patex] date'...iso-8601'`
        - Matches a date value less than or equal to the specified ISO 8601 date.
    - `[patex] date'/regex/'`
        - Matches a date value that matches the specified regex.
- Known Value
    - `[patex] known`
        - Matches any known value. (See the `known-values` crate for more information.)
    - `[patex] 'value'`
        - Matches the specified known value, which is a u64 value. dCBOR prints known values enclosed in single quotes, so we use that syntax here for familiarity. Note: This is a non-prefixed single-quoted pattern.
    - `[patex] 'name'`
        - Matches the known value with the specified name. Again we use single quotes here for familiarity. Note: This is a non-prefixed single-quoted pattern.
    - `[patex] '/regex/'`
        - Matches a known value with a name that matches the specified regex. We do not use the single quotes here. Note: This is a non-prefixed single-quoted pattern.
- Null
    - `[patex] null`
        - Matches the null value.
- Number
    - `[patex] number`
        - keyword `number` matches any number.
    - `[patex] value`
        - Bare numeric value matches the specified number.
    - `[patex] value...value`
        - Matches a number within the specified range.
    - `[patex] >=value`
        - Matches a number greater than or equal to the specified value.
    - `[patex] <=value`
        - Matches a number less than or equal to the specified value.
    - `[patex] >value`
        - Matches a number greater than the specified value.
    - `[patex] <value`
        - Matches a number less than the specified value.
    - `[patex] NaN`
        - Matches the NaN (Not a Number) value.
    - `[patex] Infinity`
        - Matches the Infinity value.
    - `[patex] -Infinity`
        - Matches the negative Infinity value.
- Text
    - `[patex] text`
        - Matches any text value.
    - `[patex] "string"`
        - Matches a text value with the specified string. dCBOR diagnostic notation uses double quotes for text strings, so we use that syntax here for familiarity.
    - `[patex] /text-regex/`
        - Matches a text value that matches the specified regex. No double quotes are used here, as the regex is not a string but a pattern to match against the text value.
- Digest
    - `[patex] digest`
        - Matches any digest value.
    - `[patex] digest'hex'`
        - Matches a digest whose value starts with the specified hex prefix. Up to 32 bytes can be specified, which is the length of the full SHA-256 digest.
    - `[patex] digest'ur:digest/value'`
        - Matches the specified `ur:digest` value.
    - `[patex] digest'/regex/'`
        - Matches a digest value that matches the specified binary regex.

## Structure Patterns

Structure patterns match parts of dCBOR items.
- Array
    - `[patex] [*]`
        - Matches any array.
    - `[patex] [{n}]`
        - Matches an array with exactly `n` elements.
    - `[patex] [{n,m}]`
        - Matches an array with between `n` and `m` elements, inclusive.
    - `[patex] [{n,}]`
        - Matches an array with at least `n` elements.
    - `[patex] [patex]`
        - Matches an array where the elements match the specified pattern. The pattern can be a simple pattern, a sequence of patterns, or patterns with repeat quantifiers.
        - Examples:
            - `[patex] [42]` - Array containing exactly one element: the number 42
            - `[patex] ["a", "b", "c"]` - Array containing exactly `[dcbor] ["a", "b", "c"]` in sequence
            - `[patex] [(*)*, 42, (*)*]` - Array containing 42 anywhere within it
            - `[patex] [42, (*)*]` - Array starting with 42, followed by any elements
            - `[patex] [(*)*, 42]` - Array ending with 42, preceded by any elements
- Map
    - `[patex] {*}`
        - Matches any map.
    - `[patex] {{n}}`
        - Matches a map with exactly `n` entries.
    - `[patex] {{n,m}}`
        - Matches a map with between `n` and `m` entries, inclusive.
    - `[patex] {{n,}}`
        - Matches a map with at least `n` entries.
    - `[patex] {patex: patex, patex: patex, ...}`
        - Matches if the specified patterns match the map's keys and values (order isn't important).
- Tagged
    - `[patex] tagged`
        - Matches any CBOR tagged value.
    - `[patex] tagged ( value, patex )`
        - Matches the specified CBOR tagged value with content that matches the given pattern. The tag value is a u64 value, formatted as a bare integer with no delimiters apart from the enclosing parentheses.
    - `[patex] tagged ( name, patex )`
        - Matches the CBOR tagged value with the specified name and content that matches the given patex. The tag name is formatted as a bare alphanumeric string (including hyphens and underscores) with no delimiters apart from the enclosing parentheses.
    - `[patex] tagged ( /regex/, patex )`
        - Matches a CBOR tagged value with a name that matches the specified regex and content that matches the given pattern.

## Meta Patterns

The following meta patterns are available to combine or modify other patterns.

Precedence: Repeat has the highest precedence, followed by And, Not, Sequence, and then Or. Parentheses can be used to group patterns and change precedence.

- And
    - `[patex] patex & patex & patex`…
        - Matches if all specified patterns match.
- Any
    - `[patex] *`
        - A bare asterisk matches any value.
- Capture
    - `[patex] @name ( patex )`
        - Matches the specified pattern and captures the match for later use with the given name.
- Not
    - `[patex] ! patex`
        - Matches if the specified pattern does not match.
            - The pattern `[patex] !*` matches no values.
- Or
    - `[patex] patex | patex | patex...`
        - Matches if any of the specified patterns match.
- Repeat
    - Greedy — grabs as many repetitions as possible, then backtracks if the rest of the pattern cannot match.
        - `[patex] ( patex )` (exactly once, this is used to group patterns)
        - `[patex] ( patex )*` (0 or more)
        - `[patex] ( patex )?` (0 or 1)
        - `[patex] ( patex )+` (1 or more)
        - `[patex] ( patex ){ n , m }` (`n` to `m` repeats, inclusive)
    - Lazy — starts with as few repetitions as possible, adding more only if the rest of the pattern cannot match.
        - `[patex] ( patex )*?` (0 or more)
        - `[patex] ( patex )??` (0 or 1)
        - `[patex] ( patex )+?` (1 or more)
        - `[patex] ( patex ){ n , m }?` (`n` to `m` repeats, inclusive)
    - Possessive — grabs as many repetitions as possible and never backtracks; if the rest of the pattern cannot match, the whole match fails.
        - `[patex] ( patex )*+` (0 or more)
        - `[patex] ( patex )?+` (0 or 1)
        - `[patex] ( patex )++` (1 or more)
        - `[patex] ( patex ){ n , m }+` (`n` to `m` repeats, inclusive)
- Search
    - `[patex] search ( patex )`
      - Visits every node in the CBOR tree, matching the specified pattern against each node.

## Example Composite Patterns

The following patterns show examples of combining structure patterns with meta patterns to create complex matching expressions:

- Nested Structure Patterns
    - `[patex] tagged ( value , [ patex ] )`
        - Matches a tagged value containing an array with the specified pattern. The pattern can be simple patterns, sequences, or patterns with repeat quantifiers.
    - `[patex] {patex: [{{n,}}]}`
        - Matches a map where the specified key pattern maps to an array with at least `n` elements.
    - `[patex] [{patex: patex}, (patex)*]`
        - Matches an array starting with a map that contains the specified key-value pattern, followed by any other elements.
