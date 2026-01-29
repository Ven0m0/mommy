mod affirmations;
mod color;
mod config;
mod mommy;
mod utils;

fn main() {
    // Changed the logic here to pass the exit code of the program
    // as exit code of mommy itself, so the exit code preserved for debug purposes:
    match mommy::mommy() {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
