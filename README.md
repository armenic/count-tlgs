# Search a directory for TLGs

- Utility to recursively search the supplied directory for `data_analysis/*csr*/prod/output`
- In the found directories search for TLGs (`t_`, `l_`, `g_`) and count them

## Tests

Run tets in one thread to that dummy file tree is setup:

```
rm -rf zzz; cargo test -- --test-threads=1
```
