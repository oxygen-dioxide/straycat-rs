use std::env;
fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        use std::{fs::write, path::PathBuf, process::Command};

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        write(manifest_dir.join("icon.rc"), "icon ICON favicon.ico").unwrap();
        Command::new(r"rc.exe")
            .current_dir(&manifest_dir)
            .arg("icon.rc")
            .spawn()
            .unwrap();
        println!("cargo:rustc-link-arg=icon.res");
    }
}
