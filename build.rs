fn main() {
    let dst = cmake::Config::new("waffle").build();
    eprintln!("{}", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=dylib=waffle-1");
    /*let binds = bindgen::Builder::default()
        .header("waffle/include/waffle-1/waffle.h")
        .header("waffle/include/waffle-1/waffle_gbm.h")
        .whitelist_type("waffle.*")
        .whitelist_function("waffle.*")
        .whitelist_type("gbm.*")
        .whitelist_function("gbm.*")
        .rustified_enum("waffle_error")
        .generate()
        .unwrap();
    binds.write_to_file("src/waffle-sys.rs").unwrap();*/
}
