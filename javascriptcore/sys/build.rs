// Generated by gir (https://github.com/gtk-rs/gir @ b3147f2b6043)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 7fa401e3ee5d)
// from webkit2gtk-gir-files
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={}", s);
        process::exit(1);
    }
}
