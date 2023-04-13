use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::error::GuiError;

pub fn run() -> Result<(), GuiError> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = build_window(&event_loop)?;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}

fn build_window(event_loop: &EventLoop<()>) -> Result<Window, GuiError> {
    WindowBuilder::new()
        .build(event_loop)
        .or_else(|e| Err(GuiError::WindowInitialization(e)))
}
