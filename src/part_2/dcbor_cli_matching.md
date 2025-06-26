# Matching Patterns with `dcbor` CLI

The `dcbor` CLI tool includes powerful pattern matching capabilities that allow you to search for, extract, and validate specific structures within dCBOR data. This chapter explores the `dcbor match` subcommand, which leverages the comprehensive pattern expression (_"patex"_) syntax of the `dcbor-pattern` crate to enable sophisticated data analysis and extraction workflows.

```admonish tip
This chapter builds upon the foundation established in [The `dcbor` Command Line Tool](dcbor_cli.md) chapter. If you haven't read that chapter yet, we recommend doing so first to familiarize yourself with the basic `dcbor` CLI operations.
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
- `<PATTERN>` is a pattern expression (or _"patex"_) written in dcbor-pattern expression syntax we'll explore in detail
- `[INPUT]` is the dCBOR data to match against (or read from stdin)
- `[OPTIONS]` control input/output formats and matching behavior

### Pattern Syntax Reference

You can find a complete reference for the patex syntax in the [Pattern Expression Syntax Appendix](../appendices/patex_syntax.md). This appendix provides a quick reference for the patex syntax, including value patterns, structure patterns, and meta patterns we'll cover later.

## Value Patterns

Value patterns are the foundation of dCBOR pattern matching. They allow you to match specific data types and exact values. Let's start with the most basic patterns and build up your understanding progressively.

### Numbers

Recall that if you simply type:

```bash
$ dcbor 42
```

You get back the hex representation of the CBOR number 42:

```bash
182a
```

If you want the CBOR diagnostic notation, you can use the `--diag` option:

```bash
$ dcbor -o diag 42
42
```

What if you have two pieces of CBOR data, and you want to check whether one of them is a number?

```bash
$ CBOR1=182a
$ CBOR2=6548656c6c6f
```

You can use the `dcbor match` command to check whether either of these is a number:

```bash
$ dcbor match NUMBER -i hex $CBOR1
42
$ dcbor match NUMBER -i hex $CBOR2
Error: No Match
```

We can see that `CBOR1` is the number `42`, and `CBOR2` is not a numeric value. So let's see whether it is a textual string by using the `TEXT` pattern:

```bash
$ dcbor match TEXT -i hex $CBOR2
"Hello"
```

The pattern matches, and we can see it is the string `"Hello"`.

The `NUMBER` pattern matches *any* numeric value, whether it's an integer or floating-point number:

```bash
$ dcbor match NUMBER 42
42
```

```bash
$ dcbor match NUMBER 3.14
3.14
```


```admonish note
Numbers in CBOR can be positive integers, negative integers, or floating-point values. The `NUMBER` pattern captures all of these types.
```

````admonish tip
To avoid confusion with command-line flags, you can use `--` to separate the pattern from the input. `--` signals that there are no command-line flags following it, allowing you to pass values that might otherwise be interpreted as flags:

```bash
$ dcbor match NUMBER -- -1
-1
```
````

### Text Strings

As we demonstrated above, the `TEXT` pattern matches any text string:

```bash
$ dcbor match TEXT '"hello"'
"hello"
```

```bash
$ dcbor match TEXT '"🌎"'
"🌎"
```

Notice that when providing text strings as input to the CLI, you need to include the quotes as part of the dCBOR diagnostic notation. This is the same quoting consideration we discussed in the [basic dcbor CLI chapter](dcbor_cli.md#quoting-input).

### Byte Strings

The `BSTR` pattern matches any byte string. Byte strings in CBOR are sequences of raw bytes, distinct from text strings which have UTF-8 character encoding semantics:

```bash
$ dcbor match BSTR "h'68656c6c6f'"
h'68656c6c6f'
```

```bash
$ dcbor match BSTR "h''"
h''
```

### Booleans and Null

The `BOOL` pattern matches boolean values:

```bash
$ dcbor match BOOL true
true
```

```bash
$ dcbor match BOOL false
false
```

```admonish note
Don't confuse the response `false` here as meaning that the pattern didn't match; it means that the input value was `false`, which is a valid match for the `BOOL` pattern.
```

The `NULL` pattern matches CBOR's `null` value:

```bash
$ dcbor match NULL null
null
```

### The Universal Pattern

The `ANY` pattern matches any CBOR value whatsoever:

```bash
$ dcbor match ANY 42
42
```

```bash
$ dcbor match ANY '"text"'
"text"
```

```bash
$ dcbor match ANY 'h"1234"'
h'1234'
```

This is useful when you want to match any value in a particular position within a larger structure.

## Specific Value Matching

Beyond matching types, you can match exact values by providing the specific value inside parentheses.

### Specific Numbers

```bash
$ dcbor match "NUMBER(42)" 42
42
```

```bash
# This won't match because 43 ≠ 42
$ dcbor match "NUMBER(42)" 43
Error: No match
```

### Specific Text Strings

```bash
$ dcbor match 'TEXT("hello")' '"hello"'
"hello"
```

```bash
# This won't match because the strings are different
$ dcbor match 'TEXT("hello")' '"world"'
Error: No match
```

### Specific Byte Strings

```bash
$ dcbor match "BSTR(h'1234')" "h'1234'"
h'1234'
```

### Specific Boolean Values

```bash
$ dcbor match "BOOL(true)" true
true
```

```bash
# This won't match because false ≠ true
$ dcbor match "BOOL(true)" false
Error: No match
```

### Advanced Value Patterns

Beyond basic type and exact value matching, dCBOR patterns support sophisticated matching criteria including ranges for numbers and regular expressions for text and byte strings.

#### Number Ranges

Numbers can be matched using ranges and inequality operators, which is useful for validating data within acceptable bounds.

##### Range Matching

You can match numbers within a specific range using the `...` syntax:

```bash
$ dcbor match "NUMBER(1...10)" 5
5
```

```bash
$ dcbor match "NUMBER(1...10)" 15
Error: No match
```

````admonish note
The `...` syntax is shorthand for an _inclusive_, or _closed_ range, meaning it includes the start and end values in the range.

The same range of numbers can also be specified with a more complex syntax using the `&` operator, which we'll cover later.

```bash
$ dcbor match "NUMBER(>=1)&NUMBER(<=10)" 5
5
```
````

##### Inequality Operators

Numbers support various inequality operators:

```bash
# Greater than
$ dcbor match "NUMBER(>5)" 10
10
```

```bash
# Greater than or equal to
$ dcbor match "NUMBER(>=5)" 5
5
```

```bash
# Less than
$ dcbor match "NUMBER(<10)" 8
8
```

```bash
# Less than or equal to
$ dcbor match "NUMBER(<=10)" 10
10
```

Using the `&` operator allows you to construct patterns that match _half-open_ ranges (where one end is inclusive and the other is exclusive):

```bash
$ dcbor match "NUMBER(>1)&NUMBER(<=10)" 10
10
$ dcbor match "NUMBER(>1)&NUMBER(<=10)" 1
Error: No match
```

##### Special Number Values

You can also match three special floating-point values: `NaN` ("not a number"), `Infinity`, and `-Infinity`.

```bash
$ dcbor match "NUMBER(NaN)" NaN
NaN
```

```bash
$ dcbor match "NUMBER(Infinity)" Infinity
Infinity
```

```bash
$ dcbor match -- "NUMBER(-Infinity)" -Infinity
-Infinity
```

```admonish note
Note the use of `--` to signal the end of command-line options, allowing you to pass values that might otherwise be interpreted as flags.
```

#### Text Regular Expressions

Regular expressions (or _regexes_) are powerful pattern matching tools for text, allowing you to search for specific patterns rather than exact text. They use special characters and syntax to define search patterns. For instance, `\d+` matches one or more digits, `[A-Z]+` matches one or more uppercase letters, and `^` and `$` anchor patterns to the beginning and end of a string respectively. With regular expressions, you can validate formats, extract information, and perform sophisticated text processing operations.

dCBOR patexes that this chapter describes are based on some of the same concepts as regexes, but they are not the same. The dCBOR pattern syntax is designed specifically for matching CBOR data structures and values, while regular expressions are a more general-purpose text processing tool. Nonetheless, some of the types you can match with dCBOR patterns, such as text strings and byte strings, can be matched using regular expressions.

Text strings can be matched using regular expressions, by using the `TEXT` pattern with a regex enclosed in forward slashes: `TEXT(/regex/)`:

```bash
# Match any email-like pattern
$ dcbor match 'TEXT(/^[^@]+@[^@]+\.[^@]+$/)' '"user@example.com"'
"user@example.com"
```

```bash
# Match strings starting with "temp"
$ dcbor match 'TEXT(/^temp/)' '"temporary"'
"temporary"
```

```bash
# This won't match because it doesn't start with "temp"
$ dcbor match 'TEXT(/^temp/)' '"permanent"'
Error: No match
```

Regular expressions use standard Rust regex syntax, which is based on Perl-compatible regular expressions (PCRE). This allows for complex pattern matching including:

- Literal characters: `abc`, `123`
- Any character: `.`
- Character classes: `[a-z]`, `[0-9]`, `\d` (digit), `\w` (word character)
- Quantifiers: `*` (zero or more), `+` (one or more), `?` (zero or one), `{n,m}` (between n and m times)
- Anchors: `^` (start), `$` (end)
- Groups and alternation: `(pattern)`, `pattern1|pattern2`

Explaining the full syntax of regular expressions is beyond the scope of this book, but you can find more information in the [Rust regex documentation](https://docs.rs/regex/latest/regex/#syntax).

#### Byte String Regular Expressions

Byte strings also support regular expression matching, useful for matching binary patterns or encoded data. Binary regexes operate on raw byte content, not on the hex string representation you see in diagnostic notation.

```admonish note
Binary regexes must start with the `(?s-u)` flags to work correctly:
- `(?s)` enables "dot matches newline" mode, allowing `.` to match across newlines (like byte `0x0a`)
- `(?-u)` disables Unicode mode, allowing `.` to match any byte value instead of just valid UTF-8 sequences
- Use `\x` notation for specific byte values (e.g., `\xFF` for byte 255)

Without these flags, patterns may fail on byte strings containing newlines or invalid UTF-8 sequences.
```

```bash
# Match byte strings containing the byte 0xFF anywhere
$ dcbor match 'BSTR(/(?s-u).*\xFF.*/)' "h'ff01020304'"
h'ff01020304'
```

```bash
# Match byte strings starting with specific bytes
$ dcbor match 'BSTR(/(?s-u)^\x01\x02/)' "h'01020304'"
h'01020304'
```

```bash
# Match byte strings ending with specific bytes
$ dcbor match 'BSTR(/(?s-u)\x03\x04$/)' "h'01020304'"
h'01020304'
```

```bash
# Match any 4-byte sequence
$ dcbor match 'BSTR(/(?s-u)^.{4}$/)' "h'12345678'"
h'12345678'
```

#### Practical Examples

These advanced patterns are particularly useful for data validation and extraction:

```bash
# Validate that ages are reasonable (0-120)
$ dcbor match "NUMBER(0...120)" 25
25

# Extract valid email addresses from text
$ dcbor match 'TEXT(/^\w+@\w+\.\w+$/)' '"john@example.com"'
"john@example.com"

# Find numeric IDs above a threshold
$ dcbor match "NUMBER(>1000)" 1001
1001

# Match ISO date-like strings
$ dcbor match 'TEXT(/^\d{4}-\d{2}-\d{2}$/)' '"2023-12-25"'
"2023-12-25"
```

These advanced value patterns form the building blocks for more complex structure matching, which we'll explore in the next section.

### Understanding Match Output

When a pattern matches, the default output shows the matched value. This seems simple now, but it becomes more meaningful when we start working with complex structures where patterns might match multiple values or nested elements.

```bash
$ dcbor match NUMBER 42
42
```

The output `42` tells us that the pattern `NUMBER` matched the input value `42`. When we move to structure patterns, you'll see how this output format shows the path through complex data structures.

### Pattern Validation and Error Messages

When a pattern doesn't match, the CLI returns an error:

```bash
$ dcbor match TEXT 42
Error: No match
```

This happens because the input `42` is a number, but the pattern `TEXT` expects a string. Understanding these error messages helps you debug your patterns and understand why they might not be working as expected.

Finally, here's are a couple of example of patterns that fail to parse:

```bash
$ dcbor match TEX '"Hello"'
Error: Failed to parse pattern at position 0..1: unrecognized token 'T'
Pattern: TEX
         ^

$ dcbor match 'TEXT("Hello"' '"Hello"'
Error: Failed to parse pattern: Expected closing parenthesis
```

## Structure Patterns

Beyond matching individual values, dCBOR patterns support matching complex structures like arrays, maps, and tagged values. These patterns allow you to validate data schemas and extract elements from nested structures.

### Array Patterns

#### Basic Array Matching

The `ARRAY` pattern matches any array structure:

```bash
$ dcbor match ARRAY '[1, 2, 3]'
[1, 2, 3]
```

```bash
$ dcbor match ARRAY '["hello", "world"]'
["hello", "world"]
```

```bash
$ dcbor match ARRAY '[]'
[]
```

#### Array Sequence Patterns

The `>` operator creates sequences within arrays, requiring elements to appear in the specified order:

```bash
# Match an array with a number followed by text
$ dcbor match "ARRAY(NUMBER > TEXT)" '[42, "hello"]'
[42, "hello"]
```

````admonish note
`NUMBER > TEXT` means the first element must be a number, followed by a text string, and that's it: these must be the only elements and they must appear in that order, so adding another element would not match:

```bash
$ dcbor match "ARRAY(NUMBER > TEXT)" '[42, "hello", 0]'
Error: No match
```
````

In this case the first element must be the exact number `42`, but the second element can be any text string:

```bash
$ dcbor match "ARRAY(NUMBER(42) > TEXT)" '[42, "hello"]'
[42, "hello"]
```

This won't match because the elements are in wrong order:

```bash
$ dcbor match "ARRAY(NUMBER > TEXT)" '["hello", 42]'
Error: No match
```

```bash
# Match array starting with number, then text, then anything else
$ dcbor match "ARRAY(NUMBER > TEXT > ANY)" '[42, "hello", true]'
[42, "hello", true]
```

```bash
# Match array starting with a boolean, then a number, then any text
$ dcbor match "ARRAY(BOOL > NUMBER > TEXT)" '[true, 42, "world"]'
[true, 42, "world"]
```

### Map Patterns

#### Key-Value Matching

Maps can be matched by specifying key-value patterns using `:` notation:

```bash
# Match map with a specific key, and a text value
$ dcbor match 'MAP(TEXT("name"): TEXT)' '{"name": "Alice", "age": 30}'
{"age": 30, "name": "Alice"}
```

Notice that it is not necessary to match every key-value pair in the map; you can match just the ones you care about. The output will show the entire map.

```bash
# Match map with number key
$ dcbor match 'MAP(NUMBER(1): TEXT)' '{1: "first", 2: "second"}'
{1: "first", 2: "second"}
```

If you want to match a map that *only* contains a specific key-value pair, you can specify the exact number of entries using the `&` operator:

```bash
# Match map with exactly one key-value pair, where key is 1 and value is any text
$ dcbor match 'MAP({1})&MAP(NUMBER(1): TEXT)' '{1: "first", 2: "second"}'
Error: No match

# Same thing, but specify there must be two entries
$ dcbor match 'MAP({2})&MAP(NUMBER(1): TEXT)' '{1: "first", 2: "second"}'
{1: "first", 2: "second"}
```

#### Specific Key Patterns

You can match maps containing specific keys regardless of other content:

```bash
# Match any map that contains a "name" key with text value
$ dcbor match 'MAP(TEXT("name"): TEXT)' '{"name": "Bob", "id": 42, "active": true}'
{"id": 42, "name": "Bob", "active": true}
```

#### Multiple Entry Patterns

Maps can specify multiple key-value requirements using comma-separated patterns:

```bash
# Match map with multiple required key-value pairs
$ dcbor match 'MAP(TEXT("id"): NUMBER, TEXT("name"): TEXT)' '{"id": 1, "name": "Alice", "extra": "data"}'
{"id": 1, "name": "Alice", "extra": "data"}
```

```bash
# Both key-value pairs must exist, but other entries are allowed
$ dcbor match 'MAP(TEXT("id"): NUMBER(1), TEXT("name"): TEXT("Alice"))' '{"id": 1, "name": "Alice", "age": 30}'
{"id": 1, "name": "Alice", "age": 30}
```

### Tagged Value Patterns

CBOR tagged values apply semantic meaning to data. Patterns can match both the tag and the content.

#### Tag Number Matching

```bash
# Match any value with tag 1234 containing a number
$ dcbor match "TAG(1234, NUMBER)" "1234(42)"
1234(42)
```

```bash
# Match tag 12345 with any content
$ dcbor match "TAG(12345, ANY)" '12345("tagged string")'
12345("tagged string")
```

#### Content Pattern Matching

Tagged patterns can specify both the tag value and required content patterns:

```bash
# Match tag 2 (bignum) with byte string content
$ dcbor match "TAG(2, BSTR)" "2(h'0102')"
2(h'0102')
```

```bash
# Match tag with array content having specific structure
$ dcbor match "TAG(42, ARRAY(NUMBER > TEXT))" '42([1, "data"])'
42([1, "data"])
```

## Basic Output Understanding

### Default Path Output

When a pattern matches, the default output shows the matching value. For structures, this represents the entire matching structure:

```bash
$ dcbor match ARRAY '[1, 2, 3]'
[1, 2, 3]
```

```bash
$ dcbor match 'MAP(TEXT("key"): ANY)' '{"key": "value", "other": 42}'
{"key": "value", "other": 42}
```

The output shows you what matched, which becomes more meaningful when working with search patterns or captures that can match multiple items or nested elements. For example, later we'll discuss the `SEARCH` pattern, which visits all the elements in a dcbor item. For a quick example, if you match a pattern that finds all numbers in an array, the output will show each number along with its context, or _path_ from the root of the structure:

```bash
$ dcbor match "SEARCH(NUMBER)" '[1, [2, 3]]'
[1, [2, 3]]
    1
[1, [2, 3]]
    [2, 3]
        2
[1, [2, 3]]
    [2, 3]
        3
```

You can choose to output the last item of each path using the `--last-only` option, which will only show the final matched items:

```bash
$ dcbor match --last-only "SEARCH(NUMBER)" '[1, [2, 3]]'
1
2
3
```

### Output Options Overview

The `dcbor match` command provides several options for controlling output format:

- `--captures`: Show named capture information (covered in advanced chapter)
- `--last-only`: Show only the final matched items
- `--in FORMAT` / `--out FORMAT`: Control input/output formats (hex, diag, etc.)
