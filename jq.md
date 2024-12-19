example of displaying only one field:

```bash
jq ".[].id"
```

display two fields:
```bash
jq ".[] | {id: .id, type: .type}"
```

counting number of elements:
```bash
jq "length"
```
