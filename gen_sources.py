#!/usr/bin/env python3
"""Extract source lists from aws-lc-sys cc_builder rust files."""

import re

base = "aws-lc-rs/aws-lc-sys"

def extract_all_sources(filename):
    """Extract all sources from a cc_builder .rs file."""
    with open(filename) as f:
        content = f.read()
    
    match = re.search(r'sources:\s*&\[(.*?)\],', content, re.DOTALL)
    if not match:
        return [], []
    
    sources_text = match.group(1)
    all_sources = re.findall(r'"([^"]+)"', sources_text)
    
    c_sources = [f"aws-lc/{s}" for s in all_sources if s.endswith('.c')]
    asm_sources = [f"aws-lc/{s}" for s in all_sources if s.endswith('.S')]
    
    return c_sources, asm_sources

# Extract from each platform
apple_aarch64 = f"{base}/builder/cc_builder/aarch64_apple_darwin.rs"
apple_x86_64 = f"{base}/builder/cc_builder/x86_64_apple_darwin.rs"
linux_aarch64 = f"{base}/builder/cc_builder/aarch64_unknown_linux_gnu.rs"
linux_x86_64 = f"{base}/builder/cc_builder/x86_64_unknown_linux_gnu.rs"

# Get common C sources (they should be the same across platforms)
c_sources, apple_aarch64_asm = extract_all_sources(apple_aarch64)
_, apple_x86_64_asm = extract_all_sources(apple_x86_64)
_, linux_aarch64_asm = extract_all_sources(linux_aarch64)
_, linux_x86_64_asm = extract_all_sources(linux_x86_64)

print("# Auto-generated from aws-lc-sys cc_builder")
print()
print("UNIVERSAL_SRCS = [")
for s in c_sources:
    print(f'    "{s}",')
print("]")
print()

print("APPLE_AARCH64_ASM = [")
for s in apple_aarch64_asm:
    print(f'    "{s}",')
print("]")
print()

print("APPLE_X86_64_ASM = [")
for s in apple_x86_64_asm:
    print(f'    "{s}",')
print("]")
print()

print("LINUX_AARCH64_ASM = [")
for s in linux_aarch64_asm:
    print(f'    "{s}",')
print("]")
print()

print("LINUX_X86_64_ASM = [")
for s in linux_x86_64_asm:
    print(f'    "{s}",')
print("]")
print()

# Empty lists for other platforms
for name in ["LINUX_X86_ASM", "LINUX_ARM_ASM", "WIN_X86_64_ASM", "WIN_AARCH64_ASM", "WIN_X86_ASM"]:
    print(f"{name} = []")
    print()
