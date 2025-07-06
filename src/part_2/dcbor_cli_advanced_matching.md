# Advanced Pattern Matching with `dcbor`

Building on the foundational pattern matching concepts from the previous chapter, we now explore the more sophisticated features of the `dcbor match` subcommand. This chapter focuses on recursive search patterns, named captures, and advanced pattern composition techniques that enable complex data extraction and analysis workflows.

```admonish tip
This chapter assumes familiarity with basic pattern matching concepts covered in [Matching using the `dcbor` CLI](dcbor_cli_matching.md). Make sure you understand value patterns and structure patterns before proceeding with these advanced techniques.
```

## Chapter Overview

This chapter covers the advanced pattern matching features:

1. **Repeated Patterns** - Patterns that can match multiple items or structures
1. **Search Patterns** - Recursive searching through data structures
2. **Named Captures** - Extracting and labeling matched data
3. **Advanced Pattern Features** - Sequences, quantifiers, and logical combinations
4. **Output Formats and Options** - Understanding match results and formatting options
5. **Practical Examples** - Real-world scenarios and complex workflows
6. **Error Handling and Debugging** - Advanced troubleshooting techniques
7. **Best Practices** - Guidelines for complex pattern design

## Search Patterns

### Recursive Search with `SEARCH`

#### Finding Values Anywhere
#### Search with Constraints
#### Multiple Match Handling

### Deep Structure Navigation

#### Nested Array Search
#### Nested Map Search
#### Mixed Structure Search

## Named Captures

### Capture Syntax

#### Basic Capture: `@name(pattern)`
#### Multiple Captures
#### Nested Captures

### Working with Capture Output

#### Understanding Capture Paths
#### Capture Formatting Options
#### Using `--captures` Flag

## Advanced Pattern Features

### Logical Combinations

#### OR Patterns: `pattern1 | pattern2`
#### AND Patterns: `pattern1 & pattern2`
#### NOT Patterns: `NOT(pattern)`

### Sequence Patterns

#### Array Sequences: `element1 > element2`
#### Map Entry Sequences
#### Mixed Sequences

### Quantifiers

#### Optional: `pattern?`
#### Zero or More: `pattern*`
#### One or More: `pattern+`
#### Range: `pattern{n,m}`

## Output Formats and Options

### Output Format Types

#### Path Output (Default)
#### Diagnostic Notation Output
#### Hexadecimal Output
#### Binary Output

### Formatting Options

#### Indentation Control: `--no-indent`
#### Path Simplification: `--last-only`
#### Annotations: `--annotate`
#### Capture Display: `--captures`

### Understanding Match Output

#### Path Representation
#### Hierarchical Structure
#### Capture Annotations

## Practical Examples

### Data Validation Scenarios

#### Schema Validation
#### Type Checking
#### Structure Verification

### Data Extraction Use Cases

#### Configuration Processing
#### Log Analysis
#### Protocol Message Parsing

### Complex Search Patterns

#### Multi-Level Data Mining
#### Conditional Extraction
#### Pattern Composition

---

*This chapter completes the coverage of the dcbor pattern matching system, equipping you with the advanced techniques needed for sophisticated data processing workflows. With both basic and advanced pattern matching skills, you're ready to tackle complex dCBOR analysis challenges and prepare for working with Gordian Envelope structures.*
