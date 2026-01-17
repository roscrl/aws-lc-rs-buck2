# Contributing

## Development Setup

1. Install [Nix](https://nixos.org/download.html)
2. Clone with submodules:
   ```bash
   git clone --recursive <repo>
   cd ppppp
   ```
3. Enter development shell:
   ```bash
   nix develop
   ```

## Building

```bash
# Build the library
buck2 build //aws-lc-rs/aws-lc-rs:aws-lc-rs --target-platforms root//platforms:default

# Run tests
buck2 run //test/crypto:test_crypto --target-platforms root//platforms:default

# Clean build
buck2 clean
```

## Project Structure

```
aws-lc-rs/aws-lc-sys/
├── BUCK           # cxx_library (C) + rust_library (FFI)
├── sources.bzl    # C/asm source file lists per platform
├── defines.bzl    # Preprocessor defines
└── aws-lc/        # C source (git submodule)

aws-lc-rs/aws-lc-rs/
└── BUCK           # rust_library depending on aws-lc-sys

third-party/rust/
├── BUCK           # Third-party Rust dependencies
├── zeroize-*/     # Vendored zeroize crate
└── untrusted-*/   # Vendored untrusted crate
```

## Adding New Platforms

1. Add assembly sources to `sources.bzl`:
   ```python
   NEW_PLATFORM_ASM = [
       "aws-lc/generated-src/new-platform/...",
   ]
   ```

2. Update `select()` in `BUCK`:
   ```python
   ASM_SRCS = select({
       "prelude//os:newos": NEW_PLATFORM_ASM,
       ...
   })
   ```

3. Add rustc cfg for bindings:
   ```python
   rustc_flags = select({
       "prelude//os:newos": ["--cfg=new_platform_bindings"],
       ...
   })
   ```

## Adding New Tests

Edit `test/crypto/main.rs` and add a new test function:

```rust
fn test_new_feature() -> bool {
    // Your test code
    true
}
```

Then call it from `main()`:
```rust
print!("XX. New Feature....................");
if test_new_feature() { println!(" ✓"); passed += 1; } 
else { println!(" ✗"); failed += 1; }
```

## Updating Dependencies

### Updating aws-lc-rs

1. Update the submodule:
   ```bash
   cd aws-lc-rs
   git fetch origin
   git checkout <new-version>
   cd ..
   ```

2. Regenerate source lists:
   ```bash
   python3 extract_sources.py > aws-lc-rs/aws-lc-sys/sources.bzl
   ```

3. Test the build

### Updating Third-Party Crates

1. Download new version:
   ```bash
   cd third-party/rust
   curl -sL https://static.crates.io/crates/<crate>/<crate>-<version>.crate | tar xz
   ```

2. Update BUCK file with new paths

## Code Style

- Follow existing patterns in BUCK files
- Use `select()` for platform-specific configuration
- Keep source lists in separate `.bzl` files
- Document non-obvious build flags

## Testing

Run all tests before submitting:
```bash
buck2 run //test/crypto:test_crypto --target-platforms root//platforms:default
```

All 18 tests must pass.
