use std::fs::create_dir_all;
use std::path::Path;

use libbpf_cargo::SkeletonBuilder;

const SRC: &str = "../../src/bpf/hello.bpf.c";

fn main() {
    // It's unfortunate we cannot use `OUT_DIR` to store the generated skeleton.
    // Reasons are because the generated skeleton contains compiler attributes
    // that cannot be `include!()`ed via macro. And we cannot use the `#[path = "..."]`
    // trick either because you cannot yet `concat!(env!("OUT_DIR"), "/skel.rs")` inside
    // the path attribute either (see https://github.com/rust-lang/rust/pull/83366).
    //
    // However, there is hope! When the above feature stabilizes we can clean this
    // all up.
    create_dir_all("./.output").unwrap();
    let skel = Path::new("./.output/hello.skel.rs");
    SkeletonBuilder::new(SRC).generate(&skel).unwrap();
    println!("cargo:rerun-if-changed={}", SRC);
}
