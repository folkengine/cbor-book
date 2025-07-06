# Envelope Pattern Expressions (_patex_)

This syntax is inspired by regular expressions but is specifically designed for Gordian Envelope.

The patex syntax is designed to be flexible and expressive. Patterns can be composed of *leaf patterns*, *structure patterns*, and combinators known as *meta-patterns*.

Keywords like `[patex] bool`, `[patex] number`, etc., are case-sensitive. Patterns can include specific values, ranges, or regexes to match against the corresponding parts of the envelope.

Parentheses are used to group patterns or specify ranges. The syntax `[patex] (pattern)` is really just the repeat pattern with a repeat that matches the pattern exactly once.

White space is ignored between tokens, so you can use it to make patterns more readable. The syntax examples below include white space both to show where it can be used and to show where it *cannot* be used (i.e., between characters of a token like `[patex] *?`)

## Leaf Patterns

All leaf patterns match Envelope leaves, which are CBOR values.

### dCBOR Value Patterns

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
    - `[patex] h'<hex>'`
        - Matches a byte string with the specified hex value.
    - `[patex] h'/<regex>/'`
        - Matches a byte string that matches the specified binary regex.
- Date
    - `[patex] date`
        - Matches any date value.
    - `[patex] date'<iso-date>'`
        - Matches a date value with the specified ISO 8601 format.
    - `[patex] date'<iso-date>...<iso-date>'`
        - Matches a date value within the specified range.
    - `[patex] date'<iso-date>...'`
        - Matches a date value greater than or equal to the specified ISO 8601 date.
    - `[patex] date'...<iso-date>'`
        - Matches a date value less than or equal to the specified ISO 8601 date.
    - `[patex] date'/<regex>/'`
        - Matches a date value that matches the specified regex.
- Known Value
    - `[patex] known`
        - Matches any known value. (See the `known-values` crate for more information.)
    - `[patex] '<value>'`
        - Matches the specified known value, which is a u64 value. dCBOR prints known values enclosed in single quotes, so we use that syntax here for familiarity. Note: This is a non-prefixed single-quoted pattern.
    - `[patex] '<name>'`
        - Matches the known value with the specified name. Again we use single quotes here for familiarity. Note: This is a non-prefixed single-quoted pattern.
    - `[patex] '/<regex>/'`
        - Matches a known value with a name that matches the specified regex. We do not use the single quotes here. Note: This is a non-prefixed single-quoted pattern.
- Null
    - `[patex] null`
        - Matches the null value.
- Number
    - `[patex] number`
        - Matches any number.
    - `[patex] <n>`
        - Bare numeric value matches the specified number.
    - `[patex] <n>...<m>`
        - Matches a number within the specified range.
    - `[patex] >= <n>`
        - Matches a number greater than or equal to the specified value.
    - `[patex] <= <n>`
        - Matches a number less than or equal to the specified value.
    - `[patex] > <n>`
        - Matches a number greater than the specified value.
    - `[patex] < <n>`
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
    - `[patex] "<string>"`
        - Matches a text value with the specified string. dCBOR diagnostic notation uses double quotes for text strings, so we use that syntax here for familiarity.
    - `[patex] /<regex>/`
        - Matches a text value that matches the specified regex. No double quotes are used here, as the regex is not a string but a pattern to match against the text value.
- Digest
    - `[patex] digest`
        - Matches any digest value.
    - `[patex] digest'<hex>'`
        - Matches a digest whose value starts with the specified hex prefix. Up to 32 bytes can be specified, which is the length of the full SHA-256 digest.
    - `[patex] digest'<ur:digest>'`
        - Matches the specified `ur:digest` value.
    - `[patex] digest'/<regex>/'`
        - Matches a digest value that matches the specified binary regex.
- Array
    - `[patex] array`
        - Matches any array.
    - `[patex] [{n}]`
        - Matches an array with exactly `n` elements.
    - `[patex] [{n,m}]`
        - Matches an array with between `n` and `m` elements, inclusive.
    - `[patex] [{n,}]`
        - Matches an array with at least `n` elements.
    - `[patex] [<patex>, <patex>, ...]`
        - Matches an array where the elements match the specified pattern. The pattern can be a simple pattern, a sequence of patterns, or patterns with repeat quantifiers.
        - Examples:
            - `[patex] [*]` - Array containing exactly one element of any type
            - `[patex] [42]` - Array containing exactly one element: the number 42
            - `[patex] ["a", "b", "c"]` - Array containing exactly `[dcbor] ["a", "b", "c"]` in sequence
            - `[patex] [(*)*, 42, (*)*]` - Array containing 42 anywhere within it
            - `[patex] [42, (*)*]` - Array starting with 42, followed by any elements
            - `[patex] [(*)*, 42]` - Array ending with 42, preceded by any elements
- Map
    - `[patex] map`
        - Matches any map.
    - `[patex] {{n}}`
        - Matches a map with exactly `n` entries.
    - `[patex] {{n,m}}`
        - Matches a map with between `n` and `m` entries, inclusive.
    - `[patex] {{n,}}`
        - Matches a map with at least `n` entries.
    - `[patex] {<patex>: <patex>, <patex>: <patex>, ...}`
        - Matches if the specified patterns match the map's keys and values (order isn't important).
- Tagged
    - `[patex] tagged`
        - Matches any CBOR tagged value.
    - `[patex] tagged ( <value>, <patex> )`
        - Matches the specified CBOR tagged value with content that matches the given pattern. The tag value is a u64 value, formatted as a bare integer with no delimiters apart from the enclosing parentheses.
    - `[patex] tagged ( <name>, <patex> )`
        - Matches the CBOR tagged value with the specified name and content that matches the given patex. The tag name is formatted as a bare alphanumeric string (including hyphens and underscores) with no delimiters apart from the enclosing parentheses.
    - `[patex] tagged ( /<regex>/, <patex> )`
        - Matches a CBOR tagged value with a name that matches the specified regex and content that matches the given pattern.

### Envelope dCBOR Patterns

- CBOR
    - `[patex] cbor`
        - Matches any subject CBOR value.
    - `[patex] cbor ( <dcbor-diagnostic-notation> )`
        - Matches a subject CBOR value that matches the specified diagnostic notation.
    - `[patex] cbor ( <ur:type/value> )`
        - Matches a subject CBOR value that matches the specified `ur`.
    - `[patex] cbor ( /<dcbor-patex>/ )`
        - Matches a subject CBOR value that matches the specified dcbor-pattern expression. This enables advanced pattern matching within CBOR structures including quantifiers, captures, and complex structural patterns. The pattern expression uses dcbor-pattern syntax.

## Structure Patterns

Structure patterns match parts of Gordian Envelope structures.

- Leaf
    - `[patex] leaf`
        - Matches any leaf envelope (terminal nodes in the envelope tree), a "bare subject". This is distinct from the `node` pattern, which matches a subject with one or more assertions.
- Assertions
    - `[patex] assert`
        - Matches any assertion.
    - `[patex] assertpred ( <patex> )`
        - Matches an assertion having a predicate that matches the specified pattern.
    - `[patex] assertobj ( <patex> )`
        - Matches an assertion having an object that matches the specified pattern.
- Digest
    - `[patex] digest ( <hex> )`
        - Matches a digest whose value starts with the specified hex prefix. Up to 32 bytes can be specified, which is the length of the full SHA-256 digest.
    - `[patex] digest ( <ur:digest> )`
        - Matches the specified `ur:digest` value, parsed using the `bc-ur` crate.
- Node
    - `[patex] node`
        - Matches any Gordian Envelope node, which is an envelope with at least one assertion.
    - `[patex] node ( { n, m } )`
        - Matches a Gordian Envelope node with between `n` and `m` assertions, inclusive. An `n` of zero will never match.
- Objects
    - `[patex] obj`
        - Matches any object.
    - `[patex] obj ( <patex> )`
        - Matches an object that matches the specified pattern.
- Obscured
    - `[patex] obscured`
        - Matches any obscured (elided, encrypted, or compressed) branch of the Envelope tree.
    - `[patex] elided`
        - Matches any elided branch of the Envelope tree.
    - `[patex] encrypted`
        - Matches any encrypted branch of the Envelope tree.
    - `[patex] compressed`
        - Matches any compressed branch of the Envelope tree.
- Predicates
    - `[patex] pred`
        - Matches any predicate.
    - `[patex] pred ( <patex> )`
        - Matches a predicate that matches the specified pattern.
- Subjects
    - `[patex] subj`
        - Matches any subject. If the envelope is not a NODE, then this is the identity function.
    - `[patex] subj ( <patex> )`
        - Matches a subject that matches the specified pattern.
- Wrapped
    - `[patex] wrapped`
        - Matches any wrapped Envelope.
    - `[patex] unwrap`
        - Matches on the content of a wrapped Envelope.

## Meta Patterns

The following meta patterns are available to combine or modify other patterns.

Precedence: Repeat has the highest precedence, followed by And, Not, Traversal, and then Or. Parentheses can be used to group patterns and change precedence.

- And
    - `[patex] <patex> & <patex> & <patex>...`
        - Matches if all specified patterns match.
- Any
    - `[patex] *`
        - Always matches.
- Capture
    - `@[name] ( <patex> )`
        - Matches the specified pattern and captures the match for later use with the given name.
- Not
    - `[patex] ! <patex>`
        - Matches if the specified patex does not match.
        - A pattern that never matches can be represented as `[patex] !*`.
- Or
    - `[patex] <patex> | <patex> | <patex> ...`
        - Matches if any of the specified patterns match.
- Repeat
    - Greedy — grabs as many repetitions as possible, then backtracks if the rest of the patex cannot match.
        - `[patex] ( <patex> )` (exactly once, this is used to group patterns)
        - `[patex] ( <patex> )*` (0 or more)
        - `[patex] ( <patex> )?` (0 or 1)
        - `[patex] ( <patex> )+` (1 or more)
        - `[patex] ( <patex> ){ n , m }` (`n` to `m` repeats, inclusive)
    - Lazy — starts with as few repetitions as possible, adding more only if the rest of the pattern cannot match.
        - `[patex] ( <patex> )*?` (0 or more)
        - `[patex] ( <patex> )??` (0 or 1)
        - `[patex] ( <patex> )+?` (1 or more)
        - `[patex] ( <patex> ){ n , m }?` (`n` to `m` repeats, inclusive)
    - Possessive — grabs as many repetitions as possible and never backtracks; if the rest of the pattern cannot match, the whole match fails.
        - `[patex] ( <patex> )*+` (0 or more)
        - `[patex] ( <patex> )?+` (0 or 1)
        - `[patex] ( <patex> )++` (1 or more)
        - `[patex] ( <patex> ){ n , m }+` (`n` to `m` repeats, inclusive)
- Search
    - `[patex] search ( <patex> )`
      - Visits every node in the Envelope tree, matching the specified pattern against each node.
- Traversal
    - `[patex] <patex> -> <patex> -> <patex>`
        - Matches if the specified patterns match a traversal path, with no other nodes in between.
