# Writing Guidelines for Technical Chapters

This document provides style and technical guidelines for writing new chapters in "The CBOR, dCBOR, and Gordian Envelope Book." These guidelines are derived from analyzing existing chapters and represent the standards you should follow to maintain consistency and quality throughout the book.

## Table of Contents

- [Writing Guidelines for Technical Chapters](#writing-guidelines-for-technical-chapters)
  - [Table of Contents](#table-of-contents)
  - [Chapter Structure and Organization](#chapter-structure-and-organization)
    - [Standard Chapter Template](#standard-chapter-template)
    - [Section Organization](#section-organization)
    - [Progressive Complexity](#progressive-complexity)
  - [Writing Style and Voice](#writing-style-and-voice)
    - [Tone and Approach](#tone-and-approach)
    - [Language Patterns](#language-patterns)
    - [Explaining Complex Concepts](#explaining-complex-concepts)
  - [Technical Content Guidelines](#technical-content-guidelines)
    - [Accuracy and Precision](#accuracy-and-precision)
    - [Comparison Tables](#comparison-tables)
    - [Technical Definitions](#technical-definitions)
    - [Error Handling and Edge Cases](#error-handling-and-edge-cases)
  - [Code Examples and Rust Integration](#code-examples-and-rust-integration)
    - [Rust Code Standards](#rust-code-standards)
    - [Code Example Structure](#code-example-structure)
    - [Code Example Guidelines](#code-example-guidelines)
    - [Formatting Code Blocks](#formatting-code-blocks)
  - [Visual Elements and Formatting](#visual-elements-and-formatting)
    - [Admonition Blocks](#admonition-blocks)
    - [Diagrams and Visual Representations](#diagrams-and-visual-representations)
      - [Binary Format Diagrams](#binary-format-diagrams)
      - [Data Flow Illustrations](#data-flow-illustrations)
    - [Links and References](#links-and-references)
  - [Documentation Standards](#documentation-standards)
    - [Section Introductions](#section-introductions)
    - [Cross-References](#cross-references)
    - [Specifications and Standards](#specifications-and-standards)
  - [Testing and Validation](#testing-and-validation)
    - [Code Testing Requirements](#code-testing-requirements)
    - [Content Validation](#content-validation)
    - [Building and Testing](#building-and-testing)
  - [File Organization and Naming](#file-organization-and-naming)
    - [File Structure](#file-structure)
    - [Naming Conventions](#naming-conventions)
    - [Supporting Files](#supporting-files)
  - [Common Patterns and Anti-Patterns](#common-patterns-and-anti-patterns)
    - [Effective Patterns](#effective-patterns)
    - [Anti-Patterns to Avoid](#anti-patterns-to-avoid)
  - [Quality Checklist](#quality-checklist)
    - [Content Quality](#content-quality)
    - [Technical Accuracy](#technical-accuracy)
    - [Writing Quality](#writing-quality)
    - [Integration](#integration)
    - [User Experience](#user-experience)
  - [Final Notes](#final-notes)

## Chapter Structure and Organization

### Standard Chapter Template

Each chapter should follow this general structure:

1. **Engaging Introduction**: Start with a practical scenario or clear problem statement
2. **Background/Context**: Build from what readers already know
3. **Core Concepts**: Introduce new concepts progressively
4. **Practical Examples**: Show concepts in action
5. **Implementation Details**: Deep dive into technical specifics
6. **Advanced Topics**: Cover edge cases and sophisticated usage
7. **Summary/Transition**: Conclude and connect to subsequent chapters

### Section Organization

- Use clear, hierarchical headings (`##`, `###`, `####`)
- Keep sections focused on a single concept
- Aim for 3-7 paragraphs per section (not rigid, but a good guideline)
- Include practical examples in every major section
- Use numbered subsections when covering step-by-step processes

### Progressive Complexity

Follow the established pattern of building complexity gradually:

```markdown
## Simple Scalar Types: Integers, Booleans, and Null
## Strings: Bytes and Text
## Collections: Arrays and Maps
## Advanced Features: Tags and Extensions
```

Start with fundamental concepts and build toward more sophisticated applications.

## Writing Style and Voice

### Tone and Approach

- **Professional but approachable**: Technical precision without academic stuffiness
- **Practical focus**: Always connect concepts to real-world applications
- **Engineering perspective**: Write for working developers, not computer science students
- **Confidence without arrogance**: Be definitive about best practices, humble about trade-offs

### Language Patterns

**Preferred patterns:**
- "We will explore..." (inclusive)
- "This enables..." (benefit-focused)
- "Consider the scenario where..." (practical framing)
- "The key insight is..." (emphasizing understanding)

**Avoid:**
- "Obviously" or "simply" (assumes knowledge)
- "Just" (minimizes complexity)
- "Clearly" (may not be clear to readers)
- Academic jargon without practical context

### Explaining Complex Concepts

1. **Lead with the "why"** before the "how"
2. **Use analogies sparingly** but effectively
3. **Provide multiple perspectives** on the same concept
4. **Connect to familiar technologies** when appropriate
5. **Acknowledge complexity** rather than hiding it

Example of good explanation structure:
```markdown
## The Need for Deterministic Encoding

Consider a scenario in software engineering: comparing two data structures that represent the same logical information... [practical scenario]

This discrepancy arises because... [technical cause]

This phenomenon, where serialization yields inconsistent byte outputs... [naming and defining]

Understanding and controlling this variability... [importance and implications]
```

## Technical Content Guidelines

### Accuracy and Precision

- **RFC references**: Always cite the authoritative specifications
- **Version specificity**: Be clear about which versions/standards you're referencing
- **Caveats and limitations**: Explicitly state when rules have exceptions
- **Trade-offs**: Discuss both benefits and costs of design decisions

### Comparison Tables

Use tables to clarify complex concepts:

```markdown
| JSON | CBOR Diagnostic | CBOR Hex | MT  | AI  | Explanation               |
| ---- | --------------- | -------- | --- | --- | ------------------------- |
| `0`  | `0`             | `00`     | 0   | 0   | Value 0 directly encoded  |
| `10` | `10`            | `0a`     | 0   | 10  | Value 10 directly encoded |
```

### Technical Definitions

- **Bold** key terms on first use
- Provide clear, precise definitions
- Use RFC-style language when appropriate
- Cross-reference related concepts

### Error Handling and Edge Cases

- Cover common failure modes
- Explain error conditions clearly
- Provide debugging guidance
- Discuss security implications

## Code Examples and Rust Integration

### Rust Code Standards

All Rust code must follow these standards:

1. **Idiomatic Rust**: Use standard Rust patterns and conventions
2. **Safety first**: Prefer `Result` and `Option` over panicking (unless writing test modules expected to panic)
3. **Clear types**: Use explicit types where helpful for understanding
4. **Documentation**: Include inline comments for complex logic
5. **Testing**: All code examples must be tested

### Code Example Structure

Use the established pattern with `mdbook` integration:

```rust
{{#rustdoc_include ../../tests/using_dcbor.rs:example_label}}
```

This requires:
1. Adding your code to appropriate test files in `/tests/`
2. Using `ANCHOR` comments to mark code sections
3. Ensuring the code compiles and runs successfully

Example test file structure:
```rust
use dcbor::prelude::*;

// ANCHOR: basic_example
pub fn demonstrate_basic_usage() {
    let value = 42;
    let cbor = value.to_cbor();
    assert_eq!(cbor.diagnostic(), "42");
    assert_eq!(cbor.hex(), "182a");
}
// ANCHOR_END: basic_example

#[test]
fn test_basic_example() {
    demonstrate_basic_usage();
}
```

### Code Example Guidelines

1. **Complete and runnable**: All examples must compile and execute
2. **Progressive complexity**: Start simple, build up
3. **Real-world relevant**: Use meaningful variable names and scenarios
4. **Error handling**: Show both success and failure cases
5. **Performance considerations**: Note when operations are expensive

### Formatting Code Blocks

- Use syntax highlighting: `\`\`\`rust`, `\`\`\`cbor`, `\`\`\`json`
- Include explanatory comments within code
- Show input and expected output
- Use consistent formatting (handled by `rustfmt`)

## Visual Elements and Formatting

### Admonition Blocks

Use the established admonition types appropriately:

```markdown
```admonish note
Use for supplementary information that adds context.
\`\`\`

```admonish tip
Use for practical advice and best practices.
\`\`\`

```admonish warning "The Danger of 'Squatting'"
Use for important cautions with descriptive titles.
\`\`\`

```admonish info
Use for additional details that enrich understanding.
\`\`\`

```admonish wip "Forthcoming..."
Use for placeholder content with poetic flair.
\`\`\`
```

### Diagrams and Visual Representations

#### Binary Format Diagrams

Use ASCII art for byte-level representations:

```
┌──────────────────────┐
│   TAG HEADER BYTE    │   → Major Type 6 + AI (determines length of tag number)
├──────────────────────┤
│   TAG NUMBER BYTES   │   → (0 to 8 bytes depending on AI)
└──────────────────────┘
           ↓
┌──────────────────────┐
│   TAGGED DATA ITEM   │   → Any valid CBOR item (primitive, array, map, etc.)
└──────────────────────┘
```

#### Data Flow Illustrations

Show transformations clearly:

```
JSON → CBOR Diagnostic → CBOR Hex → Binary Bytes
```

### Links and References

- Use descriptive link text, not "click here"
- Link to authoritative sources (RFCs, specifications)
- Include both internal cross-references and external resources
- Test all links before publication

## Documentation Standards

### Section Introductions

Each major section should begin with:
1. Clear statement of what will be covered
2. Connection to previous material
3. Preview of key takeaways

Example:
```markdown
## Extending Semantics with CBOR Tags

Having explored the fundamental mechanics of CBOR encoding, we now turn to one of its most powerful features: the ability to add semantic meaning through tags. This chapter delves into CBOR Tags (Major Type 6), showing how they enable rich data representations while maintaining the format's efficiency and extensibility.
```

### Cross-References

- Keep in mind where the chapter your writing fits into the flow of the chapters. Generally chapters should build on each other progressively, and can presume the reader's knowledge of previous chapters
- Reference other chapters by name, not number: "As we saw in [Chapter Name](relative_chapter_link.md)..."
- Use consistent terminology throughout the book
- Maintain a glossary mindset for technical terms
- Forward-reference upcoming concepts when appropriate

### Specifications and Standards

- Link to authoritative sources, but only once when introducing the reference
- Quote specifications appropriately
- Clarify when the book differs from or simplifies the spec
- But keep in mind that we want the book to stand alone as everything the reader needs

## Testing and Validation

### Code Testing Requirements

Every Rust code example must:

1. **Compile successfully** with the project's Rust version
2. **Execute without errors** in the test suite
3. **Demonstrate the stated concept** accurately
4. **Include appropriate assertions** to verify behavior

### Content Validation

Before submitting a chapter:

1. **Technical accuracy**: Verify all claims against specifications
2. **Link validation**: Use `mdbook-linkcheck` to verify external links
3. **Code execution**: Run `cargo test` to ensure all examples work
4. **Cross-references**: Verify internal links and references are accurate

### Building and Testing

Test your chapter by running:

```bash
cd cbor-book
mdbook build
mdbook serve --open
```

LLM agents can examine the `book/` directory rather than a web browser.

Verify code examples with:
```bash
cargo test
```

## File Organization and Naming

### File Structure

Follow the established pattern:
```
src/
├── part_1/
│   ├── practical_introduction_to_cbor.md
│   ├── cbor_tags.md
│   └── indefinite_length_items.md
├── part_2/
│   ├── determinism.md
│   ├── using_dcbor.md
│   └── dcbor_tags.md
└── part_3/
    ├── introducing_gordian_envelope.md
    └── envelope_encoding_and_processing.md
```

### Naming Conventions

- Use lowercase with underscores: `deterministic_encoding.md`
- Choose descriptive, not clever names
- Match chapter titles closely
- Keep names reasonably short but unambiguous

### Supporting Files

Place test code in `/tests/` with descriptive names:
```
tests/
├── using_dcbor.rs
├── dcbor_tags.rs
└── your_new_feature.rs
```

## Common Patterns and Anti-Patterns

### Effective Patterns

1. **Problem → Solution → Implementation**: Start with the problem, explain the solution conceptually, then show implementation

2. **Compare and Contrast**: Use tables and side-by-side examples to clarify differences

3. **Progressive Revelation**: Build up complex examples from simple components

4. **Multiple Perspectives**: Show the same concept from different angles (hex, diagnostic, code)

### Anti-Patterns to Avoid

1. **Diving into implementation without context**: Always establish the "why" before the "how"

2. **Overwhelming tables**: Break large tables into focused, digestible pieces

3. **Unexplained magic**: Don't show code that "just works" without explaining the underlying principles

4. **Inconsistent terminology**: Use the same terms for the same concepts throughout

5. **Missing error cases**: Don't only show the happy path

## Quality Checklist

Before considering a chapter complete, verify:

### Content Quality
- [ ] Clear learning objectives established
- [ ] Concepts build logically from simple to complex
- [ ] Real-world applications explained
- [ ] Trade-offs and limitations discussed
- [ ] Key insights highlighted appropriately

### Technical Accuracy
- [ ] All code examples compile and run
- [ ] Claims verified against authoritative sources
- [ ] Version numbers and specifications cited
- [ ] Error conditions properly handled
- [ ] Security implications addressed where relevant

### Writing Quality
- [ ] Consistent tone and voice
- [ ] Clear, jargon-free explanations
- [ ] Smooth transitions between sections
- [ ] Proper grammar and spelling
- [ ] Appropriate use of formatting and emphasis

### Integration
- [ ] Fits well with surrounding chapters
- [ ] Cross-references are accurate
- [ ] Terminology consistent with rest of book
- [ ] Examples use established patterns
- [ ] Links tested and working

### User Experience
- [ ] Information is findable and scannable
- [ ] Code examples are copy-pasteable
- [ ] Visual elements enhance understanding
- [ ] Length appropriate for content depth
- [ ] Leaves readers prepared for next chapter

---

## Final Notes

This book represents a high standard of technical writing that aims for practical applicability without sacrificing theoretical rigor. Your chapters should educate, enable, and inspire developers to build better systems using these powerful tools.

Remember that you're writing for working engineers who need to make informed decisions about technology choices. Provide them with the understanding, tools, and confidence to succeed.

When in doubt, look to existing chapters as models, particularly:
- "A Practical Introduction to CBOR" for progressive complexity
- "Determinism: Why Consistent Encodings Matter" for thorough conceptual grounding
- "Using dCBOR" for practical implementation guidance

Happy writing!
