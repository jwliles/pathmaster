fn main() {
    // Tell Cargo this script should run again if the man page changes
    println!("cargo:rerun-if-changed=pathmaster.1");

    // Supply the complete path to the man page for installation hooks
    println!(
        "cargo:rustc-env=PATHMASTER_MAN_PAGE={}",
        std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("pathmaster.1")
            .display()
    );
}
