OPENSSL_DEFINES = []

BORINGSSL_DEFINES = [
    "-DBORINGSSL_IMPLEMENTATION",
    "-DAWS_LC_STUBS",
    "-DBORINGSSL_PREFIX=aws_lc_0_22_0",
]

PREFIX_INCLUDE = []

LINUX_DEFINES = ["-D_POSIX_C_SOURCE=200112L", "-D_XOPEN_SOURCE=700"]
MACOS_DEFINES = []

COMMON_CFLAGS_LINUX = ["-std=c11", "-fPIC", "-fvisibility=hidden", "-Wno-unused-parameter"]
COMMON_CFLAGS_MACOS = ["-std=c11", "-fPIC", "-fvisibility=hidden", "-Wno-unused-parameter"]
