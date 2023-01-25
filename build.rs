use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/actions/mute_write.asm");

    // Compile `mute_write.asm` into a binary file.
    Command::new("nasm")
        .arg("-f")
        .arg("bin")
        .arg("src/actions/mute_write.asm")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
