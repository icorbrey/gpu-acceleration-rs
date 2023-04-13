use winit::error::OsError;

pub enum GuiError {
    WindowInitialization(OsError),
}
