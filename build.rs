use std::fs;

fn main() {
    let mut build = cxx_build::bridge("src/geo/types.rs");
    build.flag_if_supported("-std=c++17");

    build
        .file("src/geo/lart.cpp")
        .file("Clipper2/CPP/Clipper2Lib/clipper.h")
        .file("Clipper2/CPP/Clipper2Lib/clipper.engine.cpp")
        .file("Clipper2/CPP/Clipper2Lib/clipper.offset.cpp");

    build.compile("lart");

    println!("cargo:rerun-if-changed=src/geo/types.rs");
    println!("cargo:rerun-if-changed=include/lart.h");
    println!("cargo:rerun-if-changed=src/geo/lart.cpp");
    for f in fs::read_dir("Clipper2/CPP/Clipper2Lib").unwrap() {
        let f = f.unwrap().path();
        println!("cargo:rerun-if-changed={}", f.as_os_str().to_str().unwrap());
    }
}
