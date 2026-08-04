---
ironworks: major
---

# Replaced `file::File` with `file::FormatRead`

To facilitate future file format write capabilities (serialization), `File` has been replaced with `FormatRead`. The `from_reader<R: Read + Seek>(reader: R)` method replaces the previous `read`.
