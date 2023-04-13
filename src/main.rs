use pollster::block_on;

mod error;
mod gui;
mod state;

fn main() {
    match block_on(gui::run()) {
        _ => {}
    }
}
