use wgpu::{CreateSurfaceError, RequestDeviceError, SurfaceError};
use winit::error::OsError;

pub enum GuiError {
    InitializeWindow(OsError),
    CreateSurface(CreateSurfaceError),
    RequestAdapter,
    RequestDevice(RequestDeviceError),
    Render(SurfaceError),
}
