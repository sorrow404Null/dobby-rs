# Contributing

## Development

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
```

## Guidelines

- Keep unsafe blocks small and well-contained.
- Prefer deterministic, minimal patches for codegen/patching.
- Add tests for relocation/encoding when touching instruction logic.
