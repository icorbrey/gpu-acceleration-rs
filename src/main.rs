mod error;
mod gui;

fn main() {
    match gui::run() {
        _ => {}
    }
}
