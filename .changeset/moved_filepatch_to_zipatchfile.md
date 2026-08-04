---
ironworks: major
---

# Moved `file::patch` to `zipatch::file`

`ZiPatch` and `ChunkIterator` are now generic over the reader's type.

The `read` method provided by the `impl File` has been replaced by the `from_reader` associated function.

The existing `zipatch` feature is required to utilise the moved module. The previous `patch` feature has been removed.
