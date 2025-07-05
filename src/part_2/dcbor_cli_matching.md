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
In the examples in this chapter, the actual patex used is shown in its own block, and referred to in the command lines that follow it as `[bash] $PATTERN`. So when you see a block like this:

```patex
PATTERN=
number
```

What we're hiding is that we really wrote this:

```bash
PATTERN=$(cat <<'EOF'
number
EOF
)
```

This little bit of _heredoc_ awkwardness is the most reliable way to make sure everything in a pattern is assigned to a shell variable verbatim. For many patterns you won't need to use it yourself.

But if you do, now you know.
````

What if you have two pieces of CBOR data, and you want to check whether one of them is a number?

```bash
CBOR1=182a
CBOR2=6548656c6c6f
```

You can use the `dcbor match` command to check whether either of these is a number:

```patex
NUMBER=$(cat <<'EOF'
number
EOF
)
```

```bash
dcbor match $NUMBER -i hex $CBOR1
```

```dcbor
│ 42
```

```bash
dcbor match $NUMBER -i hex $CBOR2
```

```dcbor
│ Error: No Match
```

We can see that `CBOR1` is the number `42`, and `CBOR2` is not a numeric value. So let's see whether it is a textual string by using the `TEXT` pattern:

```patex
TEXT=$(cat <<'EOF'
text
EOF
)
```

```bash
dcbor match $TEXT -i hex $CBOR2
```

```dcbor
│ "Hello"
```

The pattern matches, and we can see it is the string `"Hello"`.

The `number` pattern matches *any* numeric value, whether it's an integer or floating-point number:

```patex
NUMBER=$(cat <<'EOF'
number
EOF
)
```

```bash
dcbor match $NUMBER 42
```

```dcbor
│ 42
```

```bash
dcbor match $NUMBER 3.14
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
NUMBER=$(cat <<'EOF'
number
EOF
)
```

```bash
dcbor match $NUMBER -- -1
```

```dcbor
│ -1
```
````

### Text Strings

As we demonstrated above, the `text` pattern matches any text string:

```patex
TEXT=$(cat <<'EOF'
text
EOF
)
```

```bash
dcbor match $TEXT '"hello"'
```

```dcbor
│ "hello"
```

```bash
dcbor match $TEXT '"🌎"'
```

```dcbor
│ "🌎"
```

Notice that when providing text strings as input to the CLI, you need to include the double-quotes as part of the dCBOR diagnostic notation. This is the same quoting consideration we discussed in the [basic dcbor CLI chapter](dcbor_cli.md#quoting-input).

### Byte Strings

The `bstr` pattern matches any byte string. Byte strings in CBOR are sequences of raw bytes, distinct from text strings which have UTF-8 character encoding semantics:

```patex
BSTR=$(cat <<'EOF'
bstr
EOF
)
```

```bash
dcbor match $BSTR "h'68656c6c6f'"
```

```dcbor
│ h'68656c6c6f'
```

The empty byte string is perfectly legal:

```bash
dcbor match $BSTR "h''"
```

```dcbor
│ h''
```

### Booleans and Null

The `bool` pattern matches both boolean values:

```patex
BOOL=$(cat <<'EOF'
bool
EOF
)
```

```bash
dcbor match $BOOL true
```

```dcbor
│ true
```

```bash
dcbor match $BOOL false
```

```dcbor
│ false
```

```admonish note
Don't confuse the response `[dcbor] false` here as meaning that the pattern didn't match; it means that the input value was `[dcbor] false`, which is a valid match for the `bool` pattern.
```

The `[patex] null` pattern matches CBOR's `[dcbor] null` value:

```patex
NULL=$(cat <<'EOF'
null
EOF
)
```

```bash
dcbor match $NULL null
```

```dcbor
│ null
```

### The Universal Pattern

The `[patex] *` ("any") pattern matches any CBOR value whatsoever.

```patex
ANY=$(cat <<'EOF'
*
EOF
)
```

```bash
dcbor match $ANY 42
```

```dcbor
│ 42
```

```bash
dcbor match $ANY '"hello"'
```

```dcbor
│ "hello"
```

```bash
dcbor match $ANY "h'1234'"
```

```dcbor
│ h'1234'
```

`[patex] *` is useful when you want to match any value in a particular position within a larger structure.

## Specific Value Matching

Beyond matching types, you can match exact values by providing the specific value as your pattern.

### Specific Numbers

```patex
FORTY_TWO=$(cat <<'EOF'
42
EOF
)
```

```bash
dcbor match $FORTY_TWO 42
```

```dcbor
│ 42
```

This won't match because 43 ≠ 42:

```bash
dcbor match $FORTY_TWO 43
```

```dcbor
│ Error: No match
```

### Specific Text Strings

```patex
HELLO=$(cat <<'EOF'
"hello"
EOF
)
```

```bash
dcbor match $HELLO '"hello"'
```

```dcbor
│ "hello"
```

This won't match because the strings are different:

```bash
dcbor match $HELLO '"world"'
```

```dcbor
│ Error: No match
```

### Specific Byte Strings

```patex
TWO_BYTES=$(cat <<'EOF'
h'1234'
EOF
)
```

```bash
dcbor match $TWO_BYTES "h'1234'"
```

```dcbor
│ h'1234'
```

### Specific Boolean Values

```patex
BOOL_TRUE=$(cat <<'EOF'
true
EOF
)
```

```bash
dcbor match $BOOL_TRUE true
```

```dcbor
│ true
```

This won't match because false ≠ true:

```bash
dcbor match $BOOL_TRUE false
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
ONE_TO_TEN=$(cat <<'EOF'
1...10
EOF
)
```

```bash
dcbor match $ONE_TO_TEN 5
```

```dcbor
│ 5
```

```bash
dcbor match $ONE_TO_TEN 15
```

```dcbor
│ Error: No match
```

````admonish note
The `...` syntax is shorthand for an _inclusive_, or _closed_ range, meaning it includes the start and end values in the range.

The same range of numbers can also be specified with a more complex syntax using the `&` operator, which we'll cover later.

```patex
ONE_TO_TEN=$(cat <<'EOF'
>=1 & <=10
EOF
)
```

```bash
dcbor match $ONE_TO_TEN 5
```

```dcbor
│ 5
```
````

##### Inequality Operators

Numbers support various inequality operators. Quoting is important here to ensure the shell doesn't misinterpret the operators as command-line directives:

Greater than:

```bash
dcbor match ">5" 10
```

```dcbor
│ 10
```

Greater than or equal to:

```bash
dcbor match ">=5" 5
```

```dcbor
│ 5
```

Less than:

```bash
dcbor match "<10" 8
```

```dcbor
│ 8
```

Less than or equal to:

```bash
dcbor match "<=10" 10
```

```dcbor
│ 10
```

##### Half-Open Ranges

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

Text strings can be matched using regular expressions, by using the a regex enclosed in forward slashes: `[patex] /regex/`:

##### Match strings starting with "temp"

```patex
STARTS_WITH_TEMP=$(cat <<'EOF'
/^temp/
EOF
)
```

```bash
dcbor match $STARTS_WITH_TEMP '"temporary"'
```

```dcbor
│ "temporary"
```

This won't match because it doesn't start with "temp":

```bash
dcbor match $STARTS_WITH_TEMP '"permanent"'
```

```dcbor
│ Error: No match
```

##### Match any email-like pattern

```patex
EMAIL_ADDRESS=$(cat <<'EOF'
/^[^@]+@[^@]+\.[^@]+$/
EOF
)
```

```bash
dcbor match $EMAIL_ADDRESS '"user@example.com"'
```

```dcbor
│ "user@example.com"
```

```admonish note "About Regular Expressions"
Regular expressions use standard Rust regex syntax, which is based on [Perl-compatible regular expressions (PCRE)](https://www.pcre.org/). This allows for complex pattern matching including:

- Literal characters: `[patex] /abc/`, `[patex] /123/`
- Any character: `[patex] /./`
- Character classes: `[patex] /[a-z]/`, `[patex] /[0-9]/`, `[patex] /\\d/` (digit), `[patex] /\\w/` (word character)
- Quantifiers: `[patex] /<pattern>*/` (zero or more), `[patex] /<pattern>+/` (one or more), `[patex] /<pattern>?/` (zero or one), `[patex] /<pattern>{n,m}/` (between n and m times)
- Anchors: `[patex] /^<pattern>/` (start), `[patex] /<pattern>$/` (end)
- Groups: `[patex] /(<pattern>)/`
- Alternation: `[patex] /<pattern1>|<pattern2>/`

Explaining the full syntax of regular expressions is beyond the scope of this book, but you can find more information on the specific Rust implementation in the [Rust regex documentation](https://docs.rs/regex/latest/regex/#syntax).
```

#### Byte String Regular Expressions

Byte strings also support regular expression matching, useful for matching binary patterns or encoded data. Binary regexes operate on raw byte content, not on the hex string representation you see in diagnostic notation. The syntax is like `[dcbor] h'hex'` above, but for regexes its: `[dcbor] h'/regex/'`.

```admonish note "Flags for Binary Regexes"
Binary regexes must start with the `(?s-u)` flags to work correctly:
- `(?s)` enables "dot matches newline" mode, allowing `.` to match across newlines (like byte `0x0a`)
- `(?-u)` disables Unicode mode, allowing `.` to match any byte value instead of just valid UTF-8 sequences
- Use `\x` notation for specific byte values (e.g., `\xFF` for byte 255)

Without these flags, patterns may fail on byte strings containing newlines or invalid UTF-8 sequences.
```

##### Match byte strings containing the byte `[dcbor] 0xFF` anywhere

```patex
CONTAINS_FF=$(cat <<'EOF'
h'/(?s-u).*\xFF.*/'
EOF
)
```

```bash
dcbor match $CONTAINS_FF "h'ff01020304'"
```

```dcbor
│ h'ff01020304'
```

##### Match byte strings starting with specific bytes `[dcbor] 0102`

```patex
STARTS_WITH_0102=$(cat <<'EOF'
h'/(?s-u)^\x01\x02/'
EOF
)
```

```bash
dcbor match $STARTS_WITH_0102 "h'01020304'"
```

```dcbor
│ h'01020304'
```

##### Match byte strings ending with specific bytes

```patex
ENDS_WITH_0304=$(cat <<'EOF'
h'/(?s-u)\x03\x04$/'
EOF
)
```

```bash
dcbor match $ENDS_WITH_0304 "h'01020304'"
```

```dcbor
│ h'01020304'
```

##### Match any 4-byte sequence

```patex
ANY_FOUR_BYTES=$(cat <<'EOF'
h'/(?s-u)^.{4}$/'
EOF
)
```

```bash
dcbor match $ANY_FOUR_BYTES "h'12345678'"
```

```dcbor
│ h'12345678'
```

#### Practical Examples

These advanced patterns are particularly useful for data validation and extraction:

##### Validate that ages are reasonable (0-120)

```bash
dcbor match "0...120" 25
```

```dcbor
│ 25
```

##### Extract valid email addresses from text

```patex
EMAIL_ADDRESS=$(cat <<'EOF'
/^\w+@\w+\.\w+$/
EOF
)
```

```bash
dcbor match $EMAIL_ADDRESS '"john@example.com"'
```

```dcbor
│ "john@example.com"
```

##### Find numeric IDs above a threshold

```bash
dcbor match ">1000" 1001
```

```dcbor
│ 1001
```

##### Match ISO-8601 date-like strings

```patex
ISO_DATE=$(cat <<'EOF'
/^\d{4}-\d{2}-\d{2}$/
EOF
)
```

```bash
dcbor match $ISO_DATE '"2023-12-25"'
```

```dcbor
│ "2023-12-25"
```

These advanced value patterns form the building blocks for more complex structure matching, which we'll explore in the next section.

### Understanding Match Output

When a pattern matches, the default output shows the matched value. This seems simple now, but it becomes more meaningful when we start working with complex structures where patterns might match multiple values or nested elements.

```bash
dcbor match number 42
```

```dcbor
│ 42
```

The output `42` tells us that the pattern `number` matched the input value `42`. When we move to structure patterns, you'll see how this output format shows the path through complex data structures.

### Pattern Validation and Error Messages

When a pattern doesn't match, the CLI returns an error:

```bash
dcbor match text 42
```

```dcbor
│ Error: No match
```

This happens because the input `42` is a number, but the pattern `text` expects a string. Understanding these error messages helps you debug your patterns and understand why they might not be working as expected.

Finally, here's are a couple of example of patterns that fail to parse:

```bash
dcbor match tex '"Hello"'
```

```dcbor
│ Error: Failed to parse pattern at position 0..1: unrecognized token 't'
│ Pattern: tex
│          ^
```

```bash
dcbor match '"Hello' '"Hello"'
```

```dcbor
│ Error: Failed to parse pattern: Unterminated string literal at 0..1
```

## Structure Patterns

Beyond matching individual values, dCBOR patterns support matching complex structures like arrays, maps, and tagged values. These patterns allow you to validate data schemas and extract elements from nested structures.

### Array Patterns

#### Basic Array Matching

The `[patex] [*]` pattern matches any array structure

```patex
ANY_ARRAY=$(cat <<'EOF'
[*]
EOF
)
```

```bash
dcbor match $ANY_ARRAY '[1, 2, 3]'
```

```dcbor
│ [1, 2, 3]
```

```bash
dcbor match $ANY_ARRAY '["hello", "world"]'
```

```dcbor
│ ["hello", "world"]
```

```bash
dcbor match $ANY_ARRAY '[]'
```

```dcbor
│ []
```

#### Array Sequence Patterns

The array pattern can contain a comma-separated list of patterns, where each pattern matches zero or more elements in the array in sequence.

##### Match an array with a number followed by text

```patex
NUMBER_THEN_TEXT=$(cat <<'EOF'
[number, text]
EOF
)
```

```bash
dcbor match $NUMBER_THEN_TEXT '[42, "hello"]'
```

```dcbor
│ [42, "hello"]
```

````admonish note
`[patex] [number, text]` means the first element must be a number, followed by a text string, and that's it: these must be the only elements and they must appear in that order, so adding another element would not match:

```bash
dcbor match $NUMBER_THEN_TEXT '[42, "hello", 0]'
```

```dcbor
│ Error: No match
```
````

In this case the first element must be the exact number `42`, but the second element can be any text string:

```patex
FORTY_TWO_THEN_TEXT=$(cat <<'EOF'
[42, text]
EOF
)
```

```bash
dcbor match $FORTY_TWO_THEN_TEXT '[42, "hello"]'
```

```dcbor
│ [42, "hello"]
```

This won't match because the elements are in wrong order:

```bash
dcbor match $FORTY_TWO_THEN_TEXT '["hello", 42]'
```

```dcbor
│ Error: No match
```

##### Match array starting with number, then text, then anything else

```patex
NUMBER_THEN_TEXT_THEN_ANY=$(cat <<'EOF'
[number, text, *]
EOF
)
```

```bash
dcbor match $NUMBER_THEN_TEXT_THEN_ANY '[42, "hello", true]'
```

```dcbor
│ [42, "hello", true]
```

````admonish note
In the example above, the `[patex] *` operator by itself matches *exactly one* element. If you want to match zero or more of any elements from this point on, you can use the repeating pattern `[patex] (*)*`:

```patex
NUMBER_THEN_TEXT_THEN_REST=$(cat <<'EOF'
[number, text, (*)*]
EOF
)
```

```bash
dcbor match $NUMBER_THEN_TEXT_THEN_REST '[42, "hello"]'
dcbor match $NUMBER_THEN_TEXT_THEN_REST '[42, "hello", true]'
dcbor match $NUMBER_THEN_TEXT_THEN_REST '[42, "hello", true, false]'
```

```dcbor
│ [42, "hello"]
│ [42, "hello", true]
│ [42, "hello", true, false]
```

We'll cover repeating patterns more thoroughly later.
````

### Map Patterns

#### Key-Value Constraints

Maps can be matched by specifying key-value constraints using `[patex] <key>: <value>` notation. For each constraint, the target map must have at least one key-value pair that satisfies the constraint.

##### Match map with a specific key, and a text value

```patex
HAS_KEY_NAME=$(cat <<'EOF'
{"name": text}
EOF
)
```

```bash
dcbor match $HAS_KEY_NAME '{"name": "Alice", "age": 30}'
```

```dcbor
│ {"age": 30, "name": "Alice"}
```

Notice that it is not necessary to match every key-value pair in the map; you can match just the ones you care about. The output will show the entire map.

##### Match map with number-valued key

```patex
HAS_KEY_1=$(cat <<'EOF'
{1: text}
EOF
)
```

```bash
dcbor match $HAS_KEY_1 '{1: "first", 2: "second"}'
```

```dcbor
│ {1: "first", 2: "second"}
```

If you want to match a map that *only* contains a specific key-value pair, you can specify the exact number of entries using the `&` operator and a map pattern containing a quantifier:

##### Match map with exactly one key-value pair, where key is 1 and value is any text

```patex
HAS_SINGLE_ENTRY_WITH_KEY_1=$(cat <<'EOF'
{ {1} } & {1: text}
EOF
)
```

This will not match because it has two entries, and the patex specifies one:

```bash
dcbor match $HAS_SINGLE_ENTRY_WITH_KEY_1 '{1: "first"}'
```

```dcbor
│ {1: "first"}
```

There are two entries, so no match:

```bash
dcbor match $HAS_SINGLE_ENTRY_WITH_KEY_1 '{1: "first", 2: "second"}'
```

```dcbor
│ Error: No match
```

##### Match map with multiple required entries

```patex
HAS_ID_AND_NAME=$(cat <<'EOF'
{"id": number, "name": text}
EOF
)
```

Both key-value pairs must exist, but other entries are allowed

```bash
dcbor match $HAS_ID_AND_NAME '{"id": 1, "name": "Alice", "age": 30}'
```

```dcbor
│ {"id": 1, "age": 30, "name": "Alice"}
```

### Tagged Value Patterns

CBOR tagged values apply semantic meaning to data. Patterns can match both the tag and the content.

#### Tag Number Matching

##### Match any value with tag 1234 containing a number

```bash
dcbor match "tagged(1234, number)" "1234(42)"
```

```dcbor
│ 1234(42)
```

##### Match tag 12345 with any content

```bash
dcbor match "tagged(12345, *)" '12345("tagged string")'
```

```dcbor
│ 12345("tagged string")
```

#### Content Pattern Matching

Tagged patterns specify both the tag value and required content patterns:

##### Match tag 2 (bignum) with byte string content

```bash
dcbor match "tagged(2, bstr)" "2(h'0102')"
```

```dcbor
│ 2(h'0102')
```

##### Match tag with array content having specific structure

```bash
dcbor match "tagged(42, [number, text])" '42([1, "data"])'
```

```dcbor
│ 42([1, "data"])
```

## Basic Output Understanding

### Default Path Output

When a pattern matches, the default output shows the matching value. For structures, this represents the entire matching structure:

```bash
dcbor match '[*]' '[1, 2, 3]'
```

```dcbor
│ [1, 2, 3]
```

```bash
dcbor match '{"key": *}' '{"key": "value", "other": 42}'
```

```dcbor
│ {"key": "value", "other": 42}
```

The output shows you what matched, which becomes more meaningful when working with search patterns or captures that can match multiple items or nested elements. For example, later we'll discuss the `search` pattern, which visits all the elements in a dcbor item. For a quick example, if you match a pattern that finds all numbers in an array, the output will show each number along with its context, or _path_ from the root of the structure:

```bash
dcbor match 'search(number)' '[1, [2, 3]]'
```

```dcbor
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
```

```dcbor
│ 1
│ 2
│ 3
```

### Output Options Overview

The `dcbor match` command provides several options for controlling output format:

- `--captures`: Show named capture information (covered in advanced chapter)
- `--last-only`: Show only the final matched items
- `--in FORMAT` / `--out FORMAT`: Control input/output formats (hex, diag, etc.)
