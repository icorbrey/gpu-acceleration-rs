use std::iter;

use wgpu::{
    Adapter, Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features,
    Instance, InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, Surface,
    SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use crate::error::GuiError;

pub struct GuiState {
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    pub surface: Surface,
    pub device: Device,
    pub window: Window,
    pub queue: Queue,
}

impl GuiState {
    pub async fn new(window: Window) -> Result<Self, GuiError> {
        let size = window.inner_size();
        let instance = generate_instance();
        let surface = create_surface(&instance, &window)?;
        let adapter = request_adapter(&instance, &surface).await?;
        let (device, queue) = request_device(&adapter).await?;
        let config = generate_config(&surface, &adapter, &size);

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            config,
            device,
            window,
            queue,
            size,
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if 0 < size.width && 0 < size.height {
            self.config.height = size.height;
            self.config.width = size.width;
            self.size = size;

            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), GuiError> {
        let output = self
            .surface
            .get_current_texture()
            .or_else(|e| Err(GuiError::Render(e)))?;

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        drop(encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        }));

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn generate_instance() -> Instance {
    Instance::new(InstanceDescriptor {
        dx12_shader_compiler: Default::default(),
        backends: Backends::all(),
    })
}

fn create_surface(instance: &Instance, window: &Window) -> Result<Surface, GuiError> {
    unsafe { instance.create_surface(window) }.or_else(|e| Err(GuiError::CreateSurface(e)))
}

async fn request_adapter(instance: &Instance, surface: &Surface) -> Result<Adapter, GuiError> {
    instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or(GuiError::RequestAdapter)
}

async fn request_device(adapter: &Adapter) -> Result<(Device, Queue), GuiError> {
    adapter
        .request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .or_else(|e| Err(GuiError::RequestDevice(e)))
}

fn generate_config(
    surface: &Surface,
    adapter: &Adapter,
    size: &PhysicalSize<u32>,
) -> SurfaceConfiguration {
    let capabilities = surface.get_capabilities(adapter);

    let format = capabilities
        .formats
        .iter()
        .copied()
        .filter(|f| f.describe().srgb)
        .next()
        .unwrap_or(capabilities.formats[0]);

    SurfaceConfiguration {
        present_mode: capabilities.present_modes[0],
        alpha_mode: capabilities.alpha_modes[0],
        usage: TextureUsages::RENDER_ATTACHMENT,
        view_formats: vec![],
        height: size.height,
        width: size.width,
        format,
    }
}
