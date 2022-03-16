// this project has been generated using
// cargo new hello_cargo

// it didn't initialize a git repository because it has ben run inside an existing project
// it generated the src directory, the main file, the Cargo.toml file, and the gitignore file
fn main() {
    println!("Hello, world!");
}

// the command cargo build will compile the program and put the executable inside target folder (gitignored)
// run from ./target/debug/hello_cargo

// the command cargo check will try to compile the program without producing an executable
// it's MUCH faster than cargo build and can help you find errors


// cargo build --release will produce an executable that is optimized for speed