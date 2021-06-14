// build.rs

use std::env;
use std::path::Path;

fn main() {
    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("java_wrapper/target/suntan-1.0-SNAPSHOT.jar");
    dbg!(&input_path);
    let output_path =
        Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("jassets/suntan.jar");
    dbg!(&output_path);
    let res = std::fs::copy(input_path, output_path).expect("must copy jar");
    dbg!(res);
}
