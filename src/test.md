Block JSON Highlighting:

```json
{
    "firstName": "Wolf",
    "lastName": "McNally",
}
```

Inline highlighting using `mdbook-inline-highlighting`:

Here is some JSON: `[json] {"firstName": "Wolf", "lastName": "McNally"}`

Our custom syntax in a block:

```dcbor
{
    "firstName": "Wolf",
    "lastName": "McNally",
}
```

Out custom syntax inline:

Here is some dCBOR: `[dcbor] {"firstName": "Wolf", "lastName": "McNally"}`

This highlights correctly: `[cbor] 1, "foo", true`

But this doesn't highlight at all: `[cbor] [1, "foo", true]`
