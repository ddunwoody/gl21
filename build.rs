/*
 * Copyright © 2023 David Dunwoody.
 *
 * All rights reserved.
 */

fn main() {
    configure();
    #[cfg(feature = "generate-bindings")]
    generate_bindings();
}

fn configure() {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target == "macos" {
        println!("cargo:rustc-link-lib=framework=OpenGL");
    } else if target == "linux" {
        println!("cargo:rustc-link-lib=GL");
    }
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings() {
    use gl_generator::{Api, Fallbacks, Profile, Registry, StaticGenerator};
    use std::fs::File;

    let mut file = File::create("src/bindings.rs").unwrap();

    Registry::new(Api::Gl, (2, 1), Profile::Compatibility, Fallbacks::None, [])
        .write_bindings(StaticGenerator, &mut file)
        .unwrap();
}
