use wgpu::SurfaceError;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{error::GuiError, state::GuiState};

pub async fn run() -> Result<(), GuiError> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = build_window(&event_loop)?;
    let mut state = GuiState::new(window).await?;

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == state.window.id() => {
            state.update();
            match state.render() {
                Err(GuiError::Render(SurfaceError::Lost)) => state.resize(state.size),
                Err(GuiError::Render(SurfaceError::OutOfMemory)) => {
                    *control_flow = ControlFlow::Exit;
                }
                Err(GuiError::Render(e)) => eprintln!("{:?}", e),
                Err(_) => {}
                Ok(_) => {}
            }
        }
        Event::MainEventsCleared => state.window.request_redraw(),
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() && !state.input(event) => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => state.resize(*physical_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size);
            }
            _ => {}
        },
        _ => {}
    });
}

fn build_window(event_loop: &EventLoop<()>) -> Result<Window, GuiError> {
    WindowBuilder::new()
        .build(event_loop)
        .or_else(|e| Err(GuiError::InitializeWindow(e)))
}
