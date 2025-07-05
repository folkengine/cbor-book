# Matching Patterns with `dcbor` CLI

The `dcbor` CLI tool includes powerful pattern matching capabilities that allow you to search for, extract, and validate specific structures within dCBOR data. This chapter introduces the `dcbor match` subcommand, which leverages the comprehensive pattern expression (AKA _"patex"_) syntax of the `dcbor-pattern` crate to enable sophisticated data analysis and extraction workflows.

```admonish tip
This chapter builds on the foundation established in [The `dcbor` Command Line Tool](dcbor_cli.md) chapter. If you haven't read that chapter yet, we recommend doing so first to familiarize yourself with the basic `dcbor` CLI operations.
```

## What is Pattern Matching?

Pattern matching in the context of dCBOR allows you to:
- **Find specific data structures** within complex CBOR documents
- **Extract values** that match certain criteria
- **Validate data conformance** to expected patterns
- **Find the paths** that lead to matching values within nested structures
- **Transform data** by capturing and reformatting matches

### The `dcbor match` Command

The basic syntax of the `dcbor match` command is:

```bash
dcbor match <PATTERN> [INPUT] [OPTIONS]
```

Where:
- `[bash] <PATTERN>` is a pattern expression (AKA _"patex"_) written in dcbor-pattern expression syntax we'll explore in detail
- `[bash] [INPUT]` is the dCBOR data to match against (or read from stdin)
- `[bash] [OPTIONS]` control input/output formats and matching behavior

### Pattern Syntax Reference

You can find a complete reference for the patex syntax in the [dCBOR Expression Syntax Appendix](../appendices/dcbor_patex.md). This appendix provides a quick reference for the patex syntax, including value patterns, structure patterns, and meta patterns we'll cover later.

## Value Patterns

Value patterns are the foundation of dCBOR pattern matching. They allow you to match specific data types and exact values. Let's start with the most basic patterns and build up your understanding progressively.

### Numbers

Recall that if you simply type:

```bash
dcbor 42
```

You get back the hex representation of the CBOR number 42:

```dcbor
│ 182a
```

If you want the CBOR diagnostic notation, you can use the `--diag` option:

```bash
dcbor -o diag 42
```

```dcbor
│ 42
```

````admonish note
For clarity, in the examples in this chapter, the actual patex used is shown in its own block, and referred to in the command lines that follow it as `[bash] $PATTERN`. So when you see a block like this:

```patex
number
```

What we're hiding is that we really wrote this:

```bash
PATTERN=$(cat <<'EOF'
number
EOF
)
```
````

What if you have two pieces of CBOR data, and you want to check whether one of them is a number?

```bash
CBOR1=182a
CBOR2=6548656c6c6f
```

You can use the `dcbor match` command to check whether either of these is a number:

```patex
PATTERN=$(cat <<'EOF'
number
EOF
)
```

```bash
dcbor match $PATTERN -i hex $CBOR1
```

```dcbor
│ 42
```

```bash
dcbor match $PATTERN -i hex $CBOR2
```

```dcbor
│ Error: No Match
```

We can see that `CBOR1` is the number `42`, and `CBOR2` is not a numeric value. So let's see whether it is a textual string by using the `TEXT` pattern:

```patex
PATTERN=$(cat <<'EOF'
text
EOF
)
```

```bash
dcbor match $PATTERN -i hex $CBOR2
```

```dcbor
│ "Hello"
```

The pattern matches, and we can see it is the string `"Hello"`.

The `number` pattern matches *any* numeric value, whether it's an integer or floating-point number:

```patex
PATTERN=$(cat <<'EOF'
number
EOF
)
```

```bash
dcbor match $PATTERN 42
```

```dcbor
│ 42
```

```bash
dcbor match $PATTERN 3.14
```

```dcbor
│ 3.14
```


```admonish note
Numbers in CBOR can be positive or negative integers, or floating-point values.
```

````admonish tip
To avoid confusion with command-line flags, you can use `--` to separate the pattern from the input. `--` signals that there are no command-line flags following it, allowing you to pass values that might otherwise be interpreted as flags. This is especially useful for negative numbers or special values like `-Infinity`.

```patex
PATTERN=$(cat <<'EOF'
number
EOF
)
```

```bash
dcbor match $PATTERN -- -1
```

```dcbor
│ -1
```
````

### Text Strings

As we demonstrated above, the `text` pattern matches any text string:

```patex
PATTERN=$(cat <<'EOF'
text
EOF
)
```

```bash
dcbor match $PATTERN '"hello"'
```

```dcbor
│ "hello"
```

```bash
dcbor match $PATTERN '"🌎"'
```

```dcbor
│ "🌎"
```

Notice that when providing text strings as input to the CLI, you need to include the double-quotes as part of the dCBOR diagnostic notation. This is the same quoting consideration we discussed in the [basic dcbor CLI chapter](dcbor_cli.md#quoting-input).

### Byte Strings

The `bstr` pattern matches any byte string. Byte strings in CBOR are sequences of raw bytes, distinct from text strings which have UTF-8 character encoding semantics:

```patex
PATTERN=$(cat <<'EOF'
bstr
EOF
)
```

```bash
dcbor match $PATTERN "h'68656c6c6f'"
```

```dcbor
│ h'68656c6c6f'
```

The empty byte string is perfectly legal:

```bash
dcbor match $PATTERN "h''"
```

```dcbor
│ h''
```

### Booleans and Null

The `bool` pattern matches both boolean values:

```patex
PATTERN=$(cat <<'EOF'
bool
EOF
)
```

```bash
dcbor match $PATTERN true
```

```dcbor
│ true
```

```bash
dcbor match $PATTERN false
```

```dcbor
│ false
```

```admonish note
Don't confuse the response `[dcbor] false` here as meaning that the pattern didn't match; it means that the input value was `[dcbor] false`, which is a valid match for the `bool` pattern.
```

The `[patex] null` pattern matches CBOR's `[dcbor] null` value:

```patex
PATTERN=$(cat <<'EOF'
null
EOF
)
```

```bash
dcbor match $PATTERN null
```

```dcbor
│ null
```

### The Universal Pattern

The `*` ("any") pattern matches any CBOR value whatsoever.

```patex
PATTERN=$(cat <<'EOF'
*
EOF
)
```

```bash
dcbor match $PATTERN 42
```

```dcbor
│ 42
```

```bash
dcbor match $PATTERN '"hello"'
```

```dcbor
│ "hello"
```

```bash
dcbor match $PATTERN "h'1234'"
```

```dcbor
│ h'1234'
```

`*` is useful when you want to match any value in a particular position within a larger structure.

## Specific Value Matching

Beyond matching types, you can match exact values by providing the specific value as your pattern.

### Specific Numbers

```patex
PATTERN=$(cat <<'EOF'
42
EOF
)
```

```bash
dcbor match $PATTERN 42
```

```dcbor
│ 42
```

```bash
# This won't match because 43 ≠ 42
dcbor match $PATTERN 43
```

```dcbor
│ Error: No match
```

### Specific Text Strings

```patex
PATTERN=$(cat <<'EOF'
"hello"
EOF
)
```

```bash
dcbor match $PATTERN '"hello"'
```

```dcbor
│ "hello"
```

```bash
# This won't match because the strings are different
dcbor match $PATTERN '"world"'
```

```dcbor
│ Error: No match
```

### Specific Byte Strings

```patex
PATTERN=$(cat <<'EOF'
h'1234'
EOF
)
```

```bash
dcbor match $PATTERN "h'1234'"
```

```dcbor
│ h'1234'
```

### Specific Boolean Values

```patex
PATTERN=$(cat <<'EOF'
true
EOF
)
```

```bash
dcbor match $PATTERN true
```

```dcbor
│ true
```

```bash
# This won't match because false ≠ true
dcbor match $PATTERN false
```

```dcbor
│ Error: No match
```

### Advanced Value Patterns

Beyond basic type and exact value matching, dCBOR patterns support sophisticated matching criteria including ranges for numbers and regular expressions for text and byte strings.

#### Number Ranges

Numbers can be matched using ranges and inequality operators, which is useful for validating data within acceptable bounds.

##### Range Matching

You can match numbers within a specific range using the `...` syntax:

```patex
PATTERN=$(cat <<'EOF'
1...10
EOF
)
```

```bash
dcbor match $PATTERN 5
```

```dcbor
│ 5
```

```bash
dcbor match $PATTERN 15
```

```dcbor
│ Error: No match
```

````admonish note
The `...` syntax is shorthand for an _inclusive_, or _closed_ range, meaning it includes the start and end values in the range.

The same range of numbers can also be specified with a more complex syntax using the `&` operator, which we'll cover later.

```patex
PATTERN=$(cat <<'EOF'
>=1 & <=10
EOF
)
```

```bash
dcbor match $PATTERN 5
```

```dcbor
│ 5
```
````

##### Inequality Operators

Numbers support various inequality operators. Quoting is important here to ensure the shell doesn't misinterpret the operators as command-line directives:

```patex
PATTERN=$(cat <<'EOF'
>5
EOF
)
```

```bash
# Greater than
dcbor match $PATTERN 10
```

```dcbor
│ 10
```

```bash
# Greater than or equal to
dcbor match ">=5" 5
```

```dcbor
│ 5
```

```bash
# Less than
dcbor match "<10" 8
```

```dcbor
│ 8
```

```bash
# Less than or equal to
dcbor match "<=10" 10
```

```dcbor
│ 10
```

Using the `&` operator allows you to construct patterns that match _half-open_ ranges (where one end is inclusive and the other is exclusive):

```bash
dcbor match ">1 & <=10" 10
```

```dcbor
│ 10
```

```bash
dcbor match ">1 & <=10" 1
```

```dcbor
│ Error: No match
```

##### Special Number Values

You can also match three special floating-point values: `NaN` ("not a number"), `Infinity`, and `-Infinity`.

```bash
dcbor match "NaN" NaN
```

```dcbor
│ NaN
```

```bash
dcbor match "Infinity" Infinity
```

```dcbor
│ Infinity
```

```bash
dcbor match -- "-Infinity" -Infinity
```

```dcbor
│ -Infinity
```

```admonish note
Note the use of `--` to signal the end of command-line options, allowing you to pass values that might otherwise be interpreted as flags.
```

#### Text Regular Expressions

Regular expressions (or _regexes_) are powerful pattern matching tools for text, allowing you to search for specific patterns rather than exact text. They use special characters and syntax to define search patterns. For instance, `\d+` matches one or more digits, `[plain] [A-Z]+` matches one or more uppercase letters, and `^` and `$` anchor patterns to the beginning and end of a string respectively. With regular expressions, you can validate formats, extract information, and perform sophisticated text processing operations.

dCBOR patexes that this chapter describes are based on some of the same concepts as regexes, but they are not the same. The dCBOR pattern expression syntax is designed specifically for matching CBOR data structures and values, while regular expressions are specifically for processing text. Nonetheless, some of the types you can match with dCBOR patterns, such as text strings and byte strings, can be matched using regular expressions.

Text strings can be matched using regular expressions, by using the a regex enclosed in forward slashes: `/regex/`:

```bash
# Match any email-like pattern
dcbor match '/^[^@]+@[^@]+\.[^@]+$/' '"user@example.com"'
```

```dcbor
│ "user@example.com"
```

```bash
# Match strings starting with "temp"
dcbor match '/^temp/' '"temporary"'
```

```dcbor
│ "temporary"
```

```bash
# This won't match because it doesn't start with "temp"
dcbor match '/^temp/' '"permanent"'
```

```dcbor
│ Error: No match
```

Regular expressions use standard Rust regex syntax, which is based on Perl-compatible regular expressions (PCRE). This allows for complex pattern matching including:

- Literal characters: `abc`, `123`
- Any character: `.`
- Character classes: `[plain] [a-z]`, `[plain] [0-9]`, `\d` (digit), `\w` (word character)
- Quantifiers: `*` (zero or more), `+` (one or more), `?` (zero or one), `{n,m}` (between n and m times)
- Anchors: `^` (start), `$` (end)
- Groups and alternation: `(pattern)`, `pattern1|pattern2`

Explaining the full syntax of regular expressions is beyond the scope of this book, but you can find more information in the [Rust regex documentation](https://docs.rs/regex/latest/regex/#syntax).

#### Byte String Regular Expressions

Byte strings also support regular expression matching, useful for matching binary patterns or encoded data. Binary regexes operate on raw byte content, not on the hex string representation you see in diagnostic notation. The syntax is like `[dcbor] h'hex'` above, but for regexes its: `[dcbor] h'/regex/'`.

```admonish note
Binary regexes must start with the `(?s-u)` flags to work correctly:
- `(?s)` enables "dot matches newline" mode, allowing `.` to match across newlines (like byte `0x0a`)
- `(?-u)` disables Unicode mode, allowing `.` to match any byte value instead of just valid UTF-8 sequences
- Use `\x` notation for specific byte values (e.g., `\xFF` for byte 255)

Without these flags, patterns may fail on byte strings containing newlines or invalid UTF-8 sequences.
```

```bash
# Match byte strings containing the byte 0xFF anywhere
dcbor match " h'/(?s-u).*\xFF.*/' " "h'ff01020304'"

│ h'ff01020304'
```

```bash
# Match byte strings starting with specific bytes
dcbor match " h'/(?s-u)^\x01\x02/' " "h'01020304'"

│ h'01020304'
```

```bash
# Match byte strings ending with specific bytes
dcbor match " h'/(?s-u)\x03\x04$/' " "h'01020304'"

│ h'01020304'
```

```bash
# Match any 4-byte sequence
dcbor match " h'/(?s-u)^.{4}$/' " "h'12345678'"

│ h'12345678'
```

#### Practical Examples

These advanced patterns are particularly useful for data validation and extraction:

```bash
# Validate that ages are reasonable (0-120)
dcbor match "0...120" 25

│ 25

# Extract valid email addresses from text
dcbor match " /^\w+@\w+\.\w+$/ " '"john@example.com"'

│ "john@example.com"

# Find numeric IDs above a threshold
dcbor match ">1000" 1001

│ 1001

# Match ISO date-like strings
dcbor match ' /^\d{4}-\d{2}-\d{2}$/ ' '"2023-12-25"'

│ "2023-12-25"
```

These advanced value patterns form the building blocks for more complex structure matching, which we'll explore in the next section.

### Understanding Match Output

When a pattern matches, the default output shows the matched value. This seems simple now, but it becomes more meaningful when we start working with complex structures where patterns might match multiple values or nested elements.

```bash
dcbor match number 42

│ 42
```

The output `42` tells us that the pattern `number` matched the input value `42`. When we move to structure patterns, you'll see how this output format shows the path through complex data structures.

### Pattern Validation and Error Messages

When a pattern doesn't match, the CLI returns an error:

```bash
dcbor match text 42

│ Error: No match
```

This happens because the input `42` is a number, but the pattern `text` expects a string. Understanding these error messages helps you debug your patterns and understand why they might not be working as expected.

Finally, here's are a couple of example of patterns that fail to parse:

```bash
dcbor match tex '"Hello"'

│ Error: Failed to parse pattern at position 0..1: unrecognized token 'T'
│ Pattern: TEX
│          ^

dcbor match '"Hello' '"Hello"'

│ Error: Failed to parse pattern: Unterminated string literal at 0..1
```

## Structure Patterns

Beyond matching individual values, dCBOR patterns support matching complex structures like arrays, maps, and tagged values. These patterns allow you to validate data schemas and extract elements from nested structures.

### Array Patterns

#### Basic Array Matching

The `[patex] [*]` pattern matches any array structure:

```bash
dcbor match '[*]' '[1, 2, 3]'

│ [1, 2, 3]
```

```bash
dcbor match '[*]' '["hello", "world"]'

│ ["hello", "world"]
```

```bash
dcbor match '[*]' '[]'

│ []
```

#### Array Sequence Patterns

The array pattern can contain a comma-separated list of patterns, where each pattern matches zero or more elements in the array in sequence.

```bash
# Match an array with a number followed by text
dcbor match '[number, text]' '[42, "hello"]'

│ [42, "hello"]
```

````admonish note
`[patex] [number, text]` means the first element must be a number, followed by a text string, and that's it: these must be the only elements and they must appear in that order, so adding another element would not match:

```bash
dcbor match "[number, text]" '[42, "hello", 0]'

│ Error: No match
```
````

In this case the first element must be the exact number `42`, but the second element can be any text string:

```bash
dcbor match '[42, text]' '[42, "hello"]'

│ [42, "hello"]
```

This won't match because the elements are in wrong order:

```bash
dcbor match '[number, text]' '["hello", 42]'

│ Error: No match
```

```bash
# Match array starting with number, then text, then anything else
dcbor match "[number, text, *]" '[42, "hello", true]'

│ [42, "hello", true]
```

```bash
# Match array starting with a boolean, then a number, then text
dcbor match "[bool, number, text]" '[true, 42, "world"]'

│ [true, 42, "world"]
```

### Map Patterns

#### Key-Value Matching

Maps can be matched by specifying key-value patterns using `:` notation:

```bash
# Match map with a specific key, and a text value
dcbor match '{"name": text}' '{"name": "Alice", "age": 30}'

│ {"age": 30, "name": "Alice"}
```

Notice that it is not necessary to match every key-value pair in the map; you can match just the ones you care about. The output will show the entire map.

```bash
# Match map with number key
dcbor match '{1: text}' '{1: "first", 2: "second"}'

│ {1: "first", 2: "second"}
```

If you want to match a map that *only* contains a specific key-value pair, you can specify the exact number of entries using the `&` operator:

```bash
# Match map with exactly one key-value pair, where key is 1 and value is any text
dcbor match '{ {1} } & {1: text}' '{1: "first", 2: "second"}'

│ Error: No match

# Same thing, but specify there must be two entries
dcbor match '{ {2} } & {1: text}' '{1: "first", 2: "second"}'

│ {1: "first", 2: "second"}
```

#### Specific Key Patterns

You can match maps containing specific keys regardless of other content:

```bash
# Match any map that contains a "name" key with text value
dcbor match '{"name": text}' '{"name": "Bob", "id": 42, "active": true}'

│ {"id": 42, "name": "Bob", "active": true}
```

#### Multiple Entry Patterns

Maps can specify multiple key-value requirements using comma-separated patterns:

```bash
# Match map with multiple required key-value pairs
dcbor match '{"id": number, "name": text}' '{"id": 1, "name": "Alice", "extra": "data"}'

│ {"id": 1, "name": "Alice", "extra": "data"}
```

```bash
# Both key-value pairs must exist, but other entries are allowed
dcbor match '{"id": 1, "name": "Alice"}' '{"id": 1, "name": "Alice", "age": 30}'

│ {"id": 1, "name": "Alice", "age": 30}
```

### Tagged Value Patterns

CBOR tagged values apply semantic meaning to data. Patterns can match both the tag and the content.

#### Tag Number Matching

```bash
# Match any value with tag 1234 containing a number
dcbor match "tagged(1234, number)" "1234(42)"

│ 1234(42)
```

```bash
# Match tag 12345 with any content
dcbor match "tagged(12345, *)" '12345("tagged string")'

│ 12345("tagged string")
```

#### Content Pattern Matching

Tagged patterns specify both the tag value and required content patterns:

```bash
# Match tag 2 (bignum) with byte string content
dcbor match "tagged(2, bstr)" "2(h'0102')"

│ 2(h'0102')
```

```bash
# Match tag with array content having specific structure
dcbor match "tagged(42, [number, text])" '42([1, "data"])'

│ 42([1, "data"])
```

## Basic Output Understanding

### Default Path Output

When a pattern matches, the default output shows the matching value. For structures, this represents the entire matching structure:

```bash
dcbor match '[*]' '[1, 2, 3]'

│ [1, 2, 3]
```

```bash
dcbor match '{"key": *}' '{"key": "value", "other": 42}'

│ {"key": "value", "other": 42}
```

The output shows you what matched, which becomes more meaningful when working with search patterns or captures that can match multiple items or nested elements. For example, later we'll discuss the `search` pattern, which visits all the elements in a dcbor item. For a quick example, if you match a pattern that finds all numbers in an array, the output will show each number along with its context, or _path_ from the root of the structure:

```bash
dcbor match 'search(number)' '[1, [2, 3]]'

│ [1, [2, 3]]
│     1
│ [1, [2, 3]]
│     [2, 3]
│         2
│ [1, [2, 3]]
│     [2, 3]
│         3
```

You can choose to output the last item of each path using the `--last-only` option, which will only show the final matched items:

```bash
dcbor match --last-only "search(number)" '[1, [2, 3]]'

│ 1
│ 2
│ 3
```

### Output Options Overview

The `dcbor match` command provides several options for controlling output format:

- `--captures`: Show named capture information (covered in advanced chapter)
- `--last-only`: Show only the final matched items
- `--in FORMAT` / `--out FORMAT`: Control input/output formats (hex, diag, etc.)
