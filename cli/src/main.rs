use std::env;
fn main() {
    let mut args = env::args();

    let production_mode = args.any(|arg| arg.contains("--production-mode"));

    if production_mode {
        println!("using production mode");
    } else {
        println!("development-mode");
    }
}
