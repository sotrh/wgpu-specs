#[macro_use]
extern crate log;
#[macro_use]
extern crate specs_derive;

mod lib;

use lib::{
    graphics::{self, CubeRenderer, Graphics, TriangleRenderer},
    camera,
};
use specs::prelude::*;
use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
};

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let (mut graphics, _window) = Graphics::windowed("wgpu-specs", &event_loop);

    let mut camera = camera::LookAtCamera::new(
        graphics.aspect_ratio(), 
        45f32, 
        (1.5, -5.0, 3.0).into(), 
        (0.0, 0.0, 0.0).into(),
    );
    info!("aspect_ratio = {}", graphics.aspect_ratio());
    let mut cube_renderer = CubeRenderer::new(&mut graphics, &camera);
    // use cgmath::SquareMatrix;
    // cube_renderer.update_matrix(&mut graphics, &cgmath::Matrix4::identity());
    let triangle_renderer = TriangleRenderer::new(&graphics);

    // let mut world = World::new();
    // let mut render_dispatcher = DispatcherBuilder::new()
    //     .with(graphics::model::ModelRenderSystem, "model_render_system", &[])
    //     .build();

    event_loop.run(move |event, _, control_flow| {
        * control_flow = if cfg!(feature = "metal-auto-capture") { 
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };

        match event {
            event::Event::WindowEvent {
                event: event::WindowEvent::Resized(size), ..
            } => {
                graphics.resize(size);
                camera.aspect_ratio = graphics.aspect_ratio();
                cube_renderer.update_matrix(&mut graphics, &camera.generate_matrix());
                // use cgmath::SquareMatrix;
                // cube_renderer.update_matrix(&mut graphics, &cgmath::Matrix4::identity())
            }
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::KeyboardInput {
                    input: event::KeyboardInput {
                        virtual_keycode: Some(event::VirtualKeyCode::Escape),
                        state: event::ElementState::Pressed,
                        ..
                    },
                    ..
                } | event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {
                    // todo: pass the event to the world
                }
            }
            event::Event::EventsCleared => {
                let frame = graphics.swap_chain.get_next_texture();
                let mut encoder = graphics.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    // triangle_renderer.draw(&mut rpass);
                    cube_renderer.draw(&mut rpass);
                }
                graphics.device.get_queue().submit(&[encoder.finish()]);
            }
            _ => (),
        }
    })
}