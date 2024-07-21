
fn main() {
    let qt_lib_path = env!("MME_QT_LIB");
    cc::Build::new()
        .cpp(true)
        .file("src/main.cpp")
        .flag("-w")
        .compile("mme");
    println!("cargo:rustc-link-search={}", qt_lib_path);
    println!("cargo:rustc-link-lib=Qt5Quick");
    println!("cargo:rustc-link-lib=Qt5PrintSupport ");
    println!("cargo:rustc-link-lib=Qt5Qml ");
    println!("cargo:rustc-link-lib=Qt5Network ");
    println!("cargo:rustc-link-lib=Qt5Widgets ");
    println!("cargo:rustc-link-lib=Qt5Gui ");
    println!("cargo:rustc-link-lib=Qt5Core");

    println!("cargo:rerun-if-changed=src/main.cpp");
}
