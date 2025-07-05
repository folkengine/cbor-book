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

This also looks good: `[cbor] [_, "a", "b"]`

```patex
/regex/
h'/regex/'
```

Works here: `[patex] /regex/`

And here: `[patex] h'/regex/'`

This works: `*`

So does this: `[json] *`

This works: `[patex] *`
