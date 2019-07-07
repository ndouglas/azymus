use std::fs;

fn main() {
    fs::copy("lib/libBearLibTerminal.dylib", "target/debug/deps/libBearLibTerminal.dylib").unwrap();
    fs::copy("lib/libBearLibTerminal.dylib", "target/release/deps/libBearLibTerminal.dylib").unwrap();
}
