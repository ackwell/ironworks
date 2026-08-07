---
ironworks: major
---

# Generalised `.file` method and traits

To facilitate future file format write capabilities (serialization), a number of
changes have been made to the traits in the `file::` module and the
corresponding top-level `.file` method.

- The top-level `.file` method now returns an intermediary `Read + Seek` value
- The `File` trait and `read` function have been replaced with `FormatRead` and
  its `from_reader`.
- `FileReaderExt` and the top-level `read_format` method are provided to improve
  the ergonomics of this change.

```rs
// Before
ironworks.file::<exl::ExcelList>("exd/root.exl")?;

// After (all are equivalent)
ironworks.read_format::<exl::ExcelList>("exd/root.exl")?
ironworks.file("exd/root.exl")?.read_format::<exl::ExcelList>()?
exl::ExcelList::from_reader(ironworks.file("exd/root.exl")?)?;
```
