// Explorer and shortcuts read the icon from the .exe itself; the window icon is set at runtime in main.rs.
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=logo.ico");
    println!("cargo:rerun-if-env-changed=MUNDWERK_VERSION");
    // Cargo.toml's version is never bumped; releases get theirs from CI, like the footer
    let version = std::env::var("MUNDWERK_VERSION").ok().filter(|v| !v.is_empty());
    let label = version.as_deref().unwrap_or("dev");
    let number = version.as_deref().map_or(0, version_number);
    let mut res = winresource::WindowsResource::new();
    res.set_icon("logo.ico")
        .set("ProductName", "Mundwerk")
        .set("FileDescription", "Mundwerk")
        .set("FileVersion", label)
        .set("ProductVersion", label)
        .set_version_info(winresource::VersionInfo::FILEVERSION, number)
        .set_version_info(winresource::VersionInfo::PRODUCTVERSION, number);
    res.compile().expect("embedding the icon in the .exe failed");
}

/// "0.12.3" as Windows' four 16-bit words: major, minor, patch, 0.
fn version_number(version: &str) -> u64 {
    version.split('.').take(3).zip([48, 32, 16]).map(|(part, shift)| part.parse::<u64>().unwrap_or(0) << shift).sum()
}
