use winit::{event_loop::EventLoop, window::Window};

mod cube;
mod triangle;
mod quad;

pub use cube::*;
pub use triangle::*;
pub use quad::*;

pub struct Graphics {
    adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    surface: wgpu::Surface,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,

    hidpi_factor: f64,
    size: winit::dpi::PhysicalSize,
}

impl Graphics {
    pub fn windowed<T: 'static>(title: &str, event_loop: &EventLoop<T>) -> (Self, Window) {
        // todo: add gl support
        // #[cfg(not(feature = "gl"))]
        let (window, hidpi_factor, size, instance, surface) = {
            let window = winit::window::Window::new(event_loop).unwrap();
            window.set_title(title);
            let hidpi_factor = window.hidpi_factor();
            let size = window.inner_size().to_physical(hidpi_factor);
            let instance = wgpu::Instance::new();
            
            use raw_window_handle::HasRawWindowHandle as _;
            let surface = instance.create_surface(window.raw_window_handle());
            // let surface = wgpu::Surface::create(&window);
            (window, hidpi_factor, size, instance, surface)
        };

        // #[cfg(feature = "gl")]
        // let (window, instance, hidpi_factor, size, surface) = {
        //     let wb = winit::window::WindowBuilder::new();
        //     let cb = wgn::glutin::ContextBuilder::new().with_vsync(true);
        //     let context = cb.build_windowed(wb, &event_loop).unwrap();
        //     context.window().set_title(title);

        //     let hidpi_factor = context.window().hidpi_factor();
        //     let size = context.window().get_inner_size().unwrap().to_physical(hidpi_factor);

        //     let (context, window) = unsafe { context.make_current().unwrap().split() };
        //     let instance = wgpu::Instance::new(context);
        //     let surface = instance.get_surface();

        //     (window, instance, hidpi_factor, size, surface)
        // };

        // let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        //     power_preference: wgpu::PowerPreference::LowPower,
        //     backends: wgpu::BackendBit::PRIMARY,
        // }).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
        });

        let device = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width.round() as u32,
            height: size.height.round() as u32,
            present_mode: wgpu::PresentMode::Vsync,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        (
            Self {
                adapter,
                device,
                sc_desc,
                swap_chain,
                surface,
                size,
                hidpi_factor,
            }, 
            window,
        )
    }

    pub fn resize(&mut self, size: winit::dpi::LogicalSize) {
        let physical = size.to_physical(self.hidpi_factor);
        info!("Resising to {:?}", physical);
        self.sc_desc.width = physical.width.round() as u32;
        self.sc_desc.height = physical.height.round() as u32;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.sc_desc.width as f32 / self.sc_desc.height as f32
    }
}