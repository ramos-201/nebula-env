pub mod command_history_handler;

fn main() {
    if let Err(err) = command_history_handler::run_handler() {
        eprintln!("Error: {}", err);
    }
}
