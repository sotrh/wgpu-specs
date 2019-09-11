#[macro_use]
extern crate log;
#[macro_use]
extern crate specs_derive;

mod lib;

use lib::{
    graphics::{self, *},
    camera,
    util,
};
use specs::prelude::*;
use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
};

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
struct Position(cgmath::Vector2<f32>);

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
struct Velocity(cgmath::Vector2<f32>);

#[derive(Debug, Clone, Copy)]
struct Bounds {
    min: cgmath::Vector2<f32>,
    max: cgmath::Vector2<f32>,
}

const MS_PER_UPDATE: std::time::Duration = std::time::Duration::from_millis(20);
const NANOS_PER_SEC: u64 = 1_000_000_000;
const DT_PER_UPDATE: f32 = (MS_PER_UPDATE.as_secs() as f32) + (MS_PER_UPDATE.as_nanos() as f32) / (NANOS_PER_SEC as f32);

struct MovementSystem;
impl <'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadExpect<'a, Bounds>,
    );

    fn run(&mut self, (mut w_pos, mut w_vel, r_bounds): Self::SystemData) {
        for (pos, vel) in (&mut w_pos, &mut w_vel).join() {
            pos.0 += vel.0 * DT_PER_UPDATE;

            if pos.0.x < r_bounds.min.x {
                pos.0.x = r_bounds.min.x;
                vel.0.x *= -1.0;
            } else if pos.0.x > r_bounds.max.x {
                pos.0.x = r_bounds.max.x;
                vel.0.x *= -1.0;
            }

            if pos.0.y < r_bounds.min.y {
                pos.0.y = r_bounds.min.y;
                vel.0.y *= -1.0;
            } else if pos.0.y > r_bounds.max.y {
                pos.0.y = r_bounds.max.y;
                vel.0.y *= -1.0;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Component)]
#[storage(VecStorage)]
struct Appearance {
    color: cgmath::Vector3<f32>,
    origin: cgmath::Vector2<f32>,
    scale: cgmath::Vector2<f32>,
    rotation: f32,
}

struct InstanceUpdateSystem;
impl <'a> System<'a> for InstanceUpdateSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Appearance>,
        WriteExpect<'a, Vec<Instance>>,
    );

    fn run(&mut self, (r_pos, r_appearance, mut instances): Self::SystemData) {
        instances.clear();
        instances.extend((&r_pos, &r_appearance).join().map(|(pos, appearance)| {
            Instance {
                offset: pos.0,
                origin: appearance.origin,
                scale: appearance.scale,
                rotation: appearance.rotation,
                color: appearance.color,
            }
        }));
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let (mut graphics, _window) = Graphics::windowed("wgpu-specs", &event_loop);
    
    let mut quad_renderer = QuadRenderer::new(&mut graphics, 100);

    let mut world = World::new();
    world.insert(Vec::<Instance>::with_capacity(100));
    world.insert(Bounds { min: (-1.0, -1.0).into(), max: (1.0, 1.0).into()});

    let mut dispatcher = DispatcherBuilder::new()
        .with(MovementSystem, "movement_system", &[])
        .with(InstanceUpdateSystem, "instance_update_sytem", &["movement_system"])
        .build();
    dispatcher.setup(&mut world);

    for _ in 0..10 {
        let rotation = util::rand(0.0, 2.0 * 3.1415);
        world.create_entity()
            .with(Position(cgmath::Zero::zero()))
            .with(Velocity(util::angle_to_vec2(rotation) * 0.2))
            .with(Appearance {
                scale: util::rand_vec2(0.1, 0.2),
                origin: cgmath::Zero::zero(),
                rotation,
                color: util::rand_vec3(0.0, 1.0),
            })
            .build();
    }
    world.maintain();

    let mut time = std::time::Instant::now();
    let mut lag = std::time::Duration::from_millis(0);

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
                let mut should_update = false;
                let elapsed = time.elapsed();
                time = std::time::Instant::now();
                lag += elapsed;

                // _window.set_title(&format!("elapsed = {:?}", elapsed));

                while lag >= MS_PER_UPDATE {
                    dispatcher.dispatch(&world);
                    world.maintain();
                    lag -= MS_PER_UPDATE;
                    should_update = true;
                }
                if should_update {
                    quad_renderer.update(&mut graphics, &world.read_resource::<Vec<Instance>>());
                }

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
                    quad_renderer.draw(&mut rpass);
                }
                
                graphics.device.get_queue().submit(&[encoder.finish()]);
            }
            _ => (),
        }
    })
}