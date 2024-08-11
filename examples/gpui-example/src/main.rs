#[cfg(not(target_os = "macos"))]
fn main() {
    panic!("Unavailable")
}

#[cfg(target_os = "macos")]
mod main_macos;
fn main() {
    main_macos::main()
}
