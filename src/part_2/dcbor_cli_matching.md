# Matching Patterns with `dcbor` CLI

## Chapter Overview

We'll cover the fundamental pattern matching concepts progressively, building a solid foundation before moving to advanced techniques:

1. **Pattern Matching Fundamentals** - Basic syntax and matching concepts
2. **Value Patterns** - Matching specific data types and values
3. **Structure Patterns** - Arrays, maps, and tagged values
4. **Basic Output Formats** - Understanding simple match results
5. **Getting Started Examples** - Simple, practical use cases
6. **Foundation for Advanced Work** - Preparing for complex

The `dcbor` CLI tool includes powerful pattern matching capabilities that allow you to search for, extract, and validate specific structures within dCBOR data. This chapter explores the `dcbor match` subcommand, which leverages the comprehensive pattern syntax of the `dcbor-pattern` crate to enable sophisticated data analysis and extraction workflows.

```admonish tip
This chapter builds upon the foundation established in [The `dcbor` Command Line Tool](dcbor_cli.md) chapter. If you haven't read that chapter yet, we recommend doing so first to familiarize yourself with the basic `dcbor` CLI operations.
```

## Chapter Overview

We'll cover the pattern matching system progressively, starting with simple examples and building toward complex search patterns:

1. **Pattern Matching Fundamentals** - Basic syntax and matching concepts
2. **Value Patterns** - Matching specific data types and values
3. **Structure Patterns** - Arrays, maps, and tagged values
4. **Search Patterns** - Recursive searching through data structures
5. **Named Captures** - Extracting and labeling matched data
6. **Advanced Patterns** - Sequences, quantifiers, and logical combinations
7. **Output Formats** - Understanding match results and formatting options
8. **Practical Examples** - Real-world scenarios and workflows
9. **Error Handling** - Debugging patterns and understanding failures

## Basic Pattern Matching Concepts

### What is Pattern Matching?

Pattern matching in the context of dCBOR allows you to:
- **Find specific data structures** within complex CBOR documents
- **Extract values** that match certain criteria
- **Validate data conformance** to expected patterns
- **Transform data** by capturing and reformatting matches

### The `dcbor match` Command

The basic syntax of the `dcbor match` command is:

```bash
dcbor match <PATTERN> [INPUT] [OPTIONS]
```

Where:
- `<PATTERN>` is a pattern expression written in dcbor-pattern syntax
- `[INPUT]` is the dCBOR data to match against (or read from stdin)
- `[OPTIONS]` control input/output formats and matching behavior

## Value Patterns

Value patterns are the foundation of dCBOR pattern matching. They allow you to match specific data types, exact values, or any combination thereof. Let's start with the most basic patterns and build up your understanding progressively.

### Basic Type Matching

The simplest patterns match any value of a specific CBOR data type. These are fundamental building blocks that you'll use constantly.

#### Numbers

The `NUMBER` pattern matches any numeric value, whether it's an integer or floating-point number:

```bash
$ dcbor match NUMBER 42
42
```

```bash
$ dcbor match NUMBER 3.14
3.14
```

```bash
$ dcbor match NUMBER -- -1
-1
```

Numbers in CBOR can be positive integers, negative integers, or floating-point values. The `NUMBER` pattern captures all of these types.

```admonish tip
When using negative numbers as input to the CLI, you need to use `--` to prevent them from being interpreted as command-line flags.
```

#### Text Strings

The `TEXT` pattern matches any text string:

```bash
$ dcbor match TEXT '"hello"'
"hello"
```

```bash
$ dcbor match TEXT '"🌎"'
"🌎"
```

Notice that when providing text strings as input to the CLI, you need to include the quotes as part of the dCBOR diagnostic notation. This is the same quoting consideration we discussed in the [basic dcbor CLI chapter](dcbor_cli.md#quoting-input).

#### Byte Strings

The `BSTR` pattern matches any byte string:

```bash
$ dcbor match BSTR "h'68656c6c6f'"
h'68656c6c6f'
```

```bash
$ dcbor match BSTR "h''"
h''
```

Byte strings in CBOR are sequences of raw bytes, distinct from text strings which have UTF-8 character encoding semantics.

#### Booleans and Null

The `BOOL` pattern matches boolean values:

```bash
$ dcbor match BOOL true
true
```

```bash
$ dcbor match BOOL false
false
```

The `NULL` pattern matches the null value:

```bash
$ dcbor match NULL null
null
```

#### The Universal Pattern

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

### Specific Value Matching

Beyond matching types, you can match exact values by providing the specific value inside parentheses.

#### Specific Numbers

```bash
$ dcbor match "NUMBER(42)" 42
42
```

```bash
# This won't match because 43 ≠ 42
$ dcbor match "NUMBER(42)" 43
Error: No match
```

#### Specific Text Strings

```bash
$ dcbor match 'TEXT("hello")' '"hello"'
"hello"
```

```bash
# This won't match because the strings are different
$ dcbor match 'TEXT("hello")' '"world"'
Error: No match
```

#### Specific Byte Strings

```bash
$ dcbor match "BSTR(h'1234')" "h'1234'"
h'1234'
```

#### Specific Boolean Values

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

#### Number Ranges and Comparisons

Numbers can be matched using ranges and comparison operators, which is particularly useful for validating data within acceptable bounds.

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

##### Comparison Operators

Numbers support various comparison operators:

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

##### Special Number Values

You can also match special floating-point values:

```bash
$ dcbor match "NUMBER(NaN)" NaN
NaN
```

These comparison patterns are invaluable for data validation scenarios where you need to ensure numeric values fall within acceptable ranges.

#### Text Regular Expressions

Text strings can be matched using regular expressions, enabling sophisticated pattern matching for string content.

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

- Character classes: `[a-z]`, `[0-9]`, `\d`, `\w`
- Quantifiers: `*`, `+`, `?`, `{n,m}`
- Anchors: `^` (start), `$` (end)
- Groups and alternation: `(pattern)`, `pattern1|pattern2`

#### Byte String Regular Expressions

Byte strings also support regular expression matching, useful for matching binary patterns or encoded data. Binary regexes operate on raw byte content, not on the hex string representation you see in diagnostic notation.

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

:::admonition type="note"

Binary regexes require the `(?s-u)` flags to work correctly:
- `(?s)` enables "dot matches newline" mode, allowing `.` to match across newlines (like byte `0x0a`)
- `(?-u)` disables Unicode mode, allowing `.` to match any byte value instead of just valid UTF-8 sequences
- Use `\x` notation for specific byte values (e.g., `\xFF` for byte 255)

Without these flags, patterns may fail on byte strings containing newlines or invalid UTF-8 sequences.

:::

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

### Pattern Syntax Reference

Here's a complete reference for value patterns:

| Pattern                  | Matches                          | Example Input      | Matches? |
| ------------------------ | -------------------------------- | ------------------ | -------- |
| `NUMBER`                 | Any numeric value                | `42`, `3.14`, `-1` | ✓        |
| `NUMBER(42)`             | Exactly the number 42            | `42`               | ✓        |
| `NUMBER(42)`             | Exactly the number 42            | `43`               | ✗        |
| `NUMBER(1...10)`         | Range 1 to 10 (inclusive)        | `5`, `1`, `10`     | ✓        |
| `NUMBER(>0)`             | Greater than 0                   | `1`, `100`         | ✓        |
| `TEXT`                   | Any text string                  | `"hello"`, `"🌎"`   | ✓        |
| `TEXT("hello")`          | Exactly the string "hello"       | `"hello"`          | ✓        |
| `TEXT("hello")`          | Exactly the string "hello"       | `"world"`          | ✗        |
| `TEXT(/^temp/)`          | Text starting with "temp"        | `"temporary"`      | ✓        |
| `BSTR`                   | Any byte string                  | `h'1234'`, `h''`   | ✓        |
| `BSTR(h'1234')`          | Exactly the bytes `h'1234'`      | `h'1234'`          | ✓        |
| `BSTR(/(?s-u).*\xFF.*/)` | Byte string containing byte 0xFF | `h'FF0102'`        | ✓        |
| `BOOL`                   | Any boolean value                | `true`, `false`    | ✓        |
| `BOOL(true)`             | Exactly `true`                   | `true`             | ✓        |
| `BOOL(true)`             | Exactly `true`                   | `false`            | ✗        |
| `NULL`                   | The null value                   | `null`             | ✓        |
| `ANY`                    | Any CBOR value                   | Anything           | ✓        |

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

#### Element-Specific Patterns

You can specify patterns for array elements using sequence notation with `>`:

```bash
# Match an array with a number followed by text
$ dcbor match "ARRAY(NUMBER > TEXT)" '[42, "hello"]'
[42, "hello"]
```

```bash
# Match specific values in sequence
$ dcbor match "ARRAY(NUMBER(42) > TEXT)" '[42, "hello"]'
[42, "hello"]
```

```bash
# This won't match because the elements are in wrong order
$ dcbor match "ARRAY(NUMBER > TEXT)" '["hello", 42]'
Error: No match
```

#### Array Sequences with `>`

The `>` operator creates sequences within arrays, requiring elements to appear in the specified order:

```bash
# Match array starting with number, then text, then anything else
$ dcbor match "ARRAY(NUMBER > TEXT > ANY)" '[42, "hello", true]'
[42, "hello", true]
```

```bash
# Match exact three-element sequence
$ dcbor match "ARRAY(BOOL > NUMBER > TEXT)" '[true, 42, "world"]'
[true, 42, "world"]
```

### Map Patterns

#### Key-Value Matching

Maps can be matched by specifying key-value patterns using `:` notation:

```bash
# Match map with specific key-value pair
$ dcbor match 'MAP(TEXT("name"): TEXT)' '{"name": "Alice", "age": 30}'
{"age": 30, "name": "Alice"}
```

```bash
# Match map with number key
$ dcbor match 'MAP(NUMBER(1): TEXT)' '{1: "first", 2: "second"}'
{1: "first", 2: "second"}
```

Note that map output is shown in dCBOR's canonical ordering (by key), which may differ from input order.

#### Specific Key Patterns

You can match maps containing specific keys regardless of other content:

```bash
# Match any map that contains a "name" key with text value
$ dcbor match 'MAP(TEXT("name"): TEXT)' '{"name": "Bob", "id": 42, "active": true}'
{"active": true, "id": 42, "name": "Bob"}
```

```bash
# Match map with multiple required key-value pairs
$ dcbor match 'MAP(TEXT("id"): NUMBER, TEXT("name"): TEXT)' '{"id": 1, "name": "Alice", "extra": "data"}'
{"extra": "data", "id": 1, "name": "Alice"}
```

#### Multiple Entry Patterns

Maps can specify multiple key-value requirements using comma-separated patterns:

```bash
# Both key-value pairs must exist
$ dcbor match 'MAP(TEXT("id"): NUMBER(1), TEXT("name"): TEXT("Alice"))' '{"id": 1, "name": "Alice"}'
{"id": 1, "name": "Alice"}
```

### Tagged Value Patterns

CBOR tagged values apply semantic meaning to data. Patterns can match both the tag and the content.

#### Tag Number Matching

```bash
# Match any value with tag 1 (often timestamps) containing a number
$ dcbor match "TAG(1, NUMBER)" "1(42)"
1970-01-01T00:00:42Z
```

Note that the output shows the semantic interpretation (Unix timestamp as ISO date) rather than the raw number.

```bash
# Match tag 100 with any content
$ dcbor match "TAG(100, ANY)" '100("tagged string")'
100("tagged string")
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

The output shows you what matched, which becomes more meaningful when working with search patterns or captures that can match multiple items or nested elements.

### Simple Output Examples

#### Single Value Matches
```bash
$ dcbor match NUMBER 42
42
```

#### Structure Matches
```bash
$ dcbor match "ARRAY(NUMBER)" '[42]'
[42]
```

#### Multiple Matches
When searching structures, you might see multiple matching paths:

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

Each line represents a path to a matching value, showing the context and the matched item.

### Output Options Overview

The `dcbor match` command provides several options for controlling output format:

- `--captures`: Show named capture information (covered in advanced chapter)
- `--no-indent`: Remove indentation from multi-line output
- `--last-only`: Show only the final matched items
- `--in FORMAT` / `--out FORMAT`: Control input/output formats (hex, diag, etc.)

### Simple Getting Started Examples

These examples help you get comfortable with pattern matching before moving to advanced techniques:

#### Finding Known Keys

```bash
# Find maps with specific structure
$ dcbor match 'MAP(TEXT("status"): ANY)' '{"status": "active", "count": 5}'
{"count": 5, "status": "active"}
```

#### Basic Content Discovery

```bash
# Find arrays containing numbers
$ dcbor match "ARRAY(NUMBER)" '[42]'
[42]
```

```bash
# Find tagged timestamps
$ dcbor match "TAG(1, NUMBER)" "1(1640995200)"
2022-01-01
```

#### Experimenting Safely

Start with simple test data and gradually increase complexity:

```bash
# Start simple
$ dcbor match NUMBER 42

# Add structure
$ dcbor match "ARRAY(NUMBER)" '[42]'

# Add constraints
$ dcbor match "ARRAY(NUMBER(42))" '[42]'

# Try variations
$ dcbor match "ARRAY(NUMBER(42))" '[43]'  # Will fail
```

## Preparing for Advanced Patterns

### What We've Covered

By now you should be comfortable with:

- **Value patterns**: Matching specific types (`NUMBER`, `TEXT`, `BSTR`, etc.) and exact values
- **Advanced value patterns**: Ranges, comparisons, and regular expressions
- **Structure patterns**: Arrays, maps, and tagged values with specific element requirements
- **Basic output**: Understanding what successful matches look like
- **Pattern syntax**: The core notation for combining patterns with `>`, `:`, and parentheses

### Understanding Pattern Validation

When patterns don't match, the CLI provides clear error messages:

```bash
$ dcbor match TEXT 42
Error: No match
```

This happens because the input `42` is a number, but the pattern `TEXT` expects a string. Understanding these error messages helps you debug your patterns and verify your data structure assumptions.

### What's Next

In the next chapter, [Advanced Pattern Matching with `dcbor`](dcbor_cli_advanced_matching.md), we'll explore:

- **Search Patterns**: Using `SEARCH` to find data anywhere in complex structures
- **Named Captures**: Using `@name(pattern)` to extract and label specific data
- **Advanced Features**: Logical combinations, sequences, and quantifiers
- **Complex Workflows**: Real-world data processing scenarios

:::admonition type="tip"

Practice these basic patterns first! The advanced features build directly on these fundamentals. Try creating test data files with nested structures and experimenting with different pattern combinations to build your intuition.

:::

## Getting Started with Simple Patterns

## Preparing for Advanced Patterns

### What We've Covered

By now you should be comfortable with:

- **Value patterns**: Matching specific types (`NUMBER`, `TEXT`, `BSTR`, etc.) and exact values
- **Advanced value patterns**: Ranges, comparisons, and regular expressions
- **Structure patterns**: Arrays, maps, and tagged values with specific element requirements
- **Basic output**: Understanding what successful matches look like
- **Pattern syntax**: The core notation for combining patterns with `>`, `:`, and parentheses

### Understanding Pattern Validation

When patterns don't match, the CLI provides clear error messages:

```bash
$ dcbor match TEXT 42
Error: No match
```

This happens because the input `42` is a number, but the pattern `TEXT` expects a string. Understanding these error messages helps you debug your patterns and verify your data structure assumptions.

### What's Next

In the next chapter, [Advanced Pattern Matching with `dcbor`](dcbor_cli_advanced_matching.md), we'll explore:

- **Search Patterns**: Using `SEARCH` to find data anywhere in complex structures
- **Named Captures**: Using `@name(pattern)` to extract and label specific data
- **Advanced Features**: Logical combinations, sequences, and quantifiers
- **Complex Workflows**: Real-world data processing scenarios

:::admonition type="tip"

Practice these basic patterns first! The advanced features build directly on these fundamentals. Try creating test data files with nested structures and experimenting with different pattern combinations to build your intuition.

:::

---

*This chapter has established the fundamentals of pattern matching with the dcbor CLI. With these basic skills, you can already accomplish many useful data validation and extraction tasks. The next chapter will build on this foundation to unlock the full power of the pattern matching system.*
