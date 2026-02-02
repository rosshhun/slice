# slice

Slice text files by lines, bytes, or ranges — a `head`/`tail`-style CLI with extras.

[![Repology](https://repology.org/badge/latest-versions/slice.svg)](https://repology.org/project/slice/versions)

## Install

**From source (cargo):**

```bash
cargo install slice
```

**From crates.io** (after publishing): the same — Repology tracks [crates.io](https://repology.org/repository/crates_io), so your package will appear on [Repology](https://repology.org/project/slice/versions) once published there.

## Usage

```bash
slice --head 20 file.log
slice --tail 10 file.log
slice --lines 50-100 file.log
slice --help
```

## License

Apache-2.0
