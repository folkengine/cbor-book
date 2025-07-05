This works:

```json
{
    "firstName": "Wolf",
    "lastName": "McNally",
}
```

The inline highlighting using `mdbook-inline-highlighting` also works:

Here is some JSON: `[json] {"firstName": "Wolf", "lastName": "McNally"}`

Our custom syntax also works:

```dcbor
{
    "firstName": "Wolf",
    "lastName": "McNally",
}
```

But it doesn't work for our custom syntax:

Here is some dCBOR: `[dcbor] {"firstName": "Wolf", "lastName": "McNally"}`
