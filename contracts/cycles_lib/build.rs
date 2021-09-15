fn main() {
    cc::Build::new()
        .file("src/test.c")
        .static_flag(true)
        .flag("-O3")
        .flag("-fvisibility=hidden")
        .flag("-fdata-sections")
        .flag("-ffunction-sections")
        .include("../../deps/sparse-merkle-tree/c/rust-tests/src/tests")
        .include("../../deps/sparse-merkle-tree/c/")
        .include("../../deps/sparse-merkle-tree/c/deps/ckb-c-stdlib")
        .flag("-Wall")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-nonnull")
        .define("__SHARED_LIBRARY__", None)
        .define("CKB_STDLIB_NO_SYSCALL_IMPL", None)
        .compile("smt-test-c-impl");
}
