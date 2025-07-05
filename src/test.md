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

This also works: `[cbor] [1, "foo", true]`

This looks good: `[json] [_, "a", "b"]`

But this is not right: `[cbor] [_ "a", "b"]`
