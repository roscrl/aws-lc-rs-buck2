#!/usr/bin/env bash
# Create a release tarball
set -euo pipefail

VERSION=$(cat VERSION)
RELEASE_NAME="aws-lc-rs-buck2-${VERSION}"
RELEASE_DIR="releases/${RELEASE_NAME}"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           Creating Release: ${RELEASE_NAME}                  ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Clean previous releases
rm -rf releases/
mkdir -p "${RELEASE_DIR}"

# Copy essential files
echo "Copying files..."
cp -r \
    .buckconfig \
    .buckroot \
    BUCK \
    LICENSE \
    README.md \
    CHANGELOG.md \
    CONTRIBUTING.md \
    VERSION \
    Makefile \
    flake.nix \
    flake.lock \
    platforms \
    toolchains \
    none \
    examples \
    test \
    third-party \
    scripts \
    docs \
    "${RELEASE_DIR}/"

# Copy aws-lc-rs BUCK files (not full source - that's a submodule)
mkdir -p "${RELEASE_DIR}/aws-lc-rs/aws-lc-sys"
mkdir -p "${RELEASE_DIR}/aws-lc-rs/aws-lc-rs"
cp aws-lc-rs/aws-lc-sys/BUCK "${RELEASE_DIR}/aws-lc-rs/aws-lc-sys/"
cp aws-lc-rs/aws-lc-sys/sources.bzl "${RELEASE_DIR}/aws-lc-rs/aws-lc-sys/"
cp aws-lc-rs/aws-lc-sys/defines.bzl "${RELEASE_DIR}/aws-lc-rs/aws-lc-sys/"
cp aws-lc-rs/aws-lc-rs/BUCK "${RELEASE_DIR}/aws-lc-rs/aws-lc-rs/"

# Copy prelude (required for build)
echo "Copying prelude..."
cp -r prelude "${RELEASE_DIR}/"

# Create setup instructions
cat > "${RELEASE_DIR}/SETUP.md" << 'EOF'
# Setup Instructions

This release contains the Buck2 build configuration for aws-lc-rs.

## Requirements

You need to clone the aws-lc-rs source separately:

```bash
# Clone aws-lc-rs into this directory
git clone --recursive https://github.com/aws/aws-lc-rs.git aws-lc-rs-src

# Link or copy the source
ln -s aws-lc-rs-src/aws-lc-sys/aws-lc aws-lc-rs/aws-lc-sys/aws-lc
ln -s aws-lc-rs-src/aws-lc-sys/src aws-lc-rs/aws-lc-sys/src
ln -s aws-lc-rs-src/aws-lc-rs/src aws-lc-rs/aws-lc-rs/src
```

## Build

```bash
nix develop  # or install buck2, rust, clang manually
make test
```
EOF

# Create tarball
echo "Creating tarball..."
cd releases
tar -czf "${RELEASE_NAME}.tar.gz" "${RELEASE_NAME}"
cd ..

# Calculate checksums
echo "Calculating checksums..."
cd releases
shasum -a 256 "${RELEASE_NAME}.tar.gz" > "${RELEASE_NAME}.tar.gz.sha256"
cd ..

# Show results
echo ""
echo "Release created:"
ls -lh "releases/${RELEASE_NAME}.tar.gz"
cat "releases/${RELEASE_NAME}.tar.gz.sha256"
echo ""
echo "✅ Release complete!"
