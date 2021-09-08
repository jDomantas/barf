mod input;
mod queue;
mod renderer;
mod texture;
mod timer;
mod wgpu_render;

use std::{
    sync::mpsc::{Receiver, TryRecvError},
    thread::JoinHandle,
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use crate::{
    wgpu_render::WgpuState,
    timer::Timer,
    queue::FinishedJob,
};
pub use crate::{
    input::{Input, Key},
    queue::{JobQueue, JobToken},
    renderer::{Rotate, Renderer},
    texture::{MakeTransparent, TextureProcessor},
};

pub trait Game {
    fn on_start(&mut self, ctx: &mut Ctx);
    fn on_texture_loaded(&mut self, ctx: &mut Ctx, job: JobToken, texture: Texture);
    fn update(&mut self, ctx: &mut Ctx);
    fn draw(&mut self, ctx: &mut Ctx, renderer: &mut Renderer<'_>);
}

pub struct Ctx {
    job_queue: JobQueue,
    input: Input,
    screen_size: (u32, u32),
}

impl Ctx {
    pub fn job_queue(&mut self) -> &mut JobQueue {
        &mut self.job_queue
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn screen_size(&self) -> (u32, u32) {
        self.screen_size
    }
}

struct GameRunner {
    game: Box<dyn Game>,
    wgpu: WgpuState,
    window: Window,
    timer: Timer,
    ctx: Ctx,
    job_thread: Option<JoinHandle<()>>,
    job_results: Receiver<FinishedJob>,
    should_exit: bool,
}

impl GameRunner {
    fn event(
        &mut self,
        event: Event<()>,
        control_flow: &mut ControlFlow,
    ) {
        if self.should_exit {
            *control_flow = ControlFlow::Exit;
            return;
        }
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event,
                window_id,
            } if window_id == self.window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(key),
                        ..
                    } => {
                        self.ctx.input.press_key(key.into());
                    }
                    KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(key),
                        ..
                    } => {
                        self.ctx.input.release_key(key.into());
                    }
                    _ => {}
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    match (state, button) {
                        (ElementState::Pressed, MouseButton::Left) => {
                            self.ctx.input.set_mouse_left_press(true);
                        }
                        (ElementState::Released, MouseButton::Left) => {
                            self.ctx.input.set_mouse_left_press(false);
                        }
                        (ElementState::Pressed, MouseButton::Right) => {
                            self.ctx.input.set_mouse_right_press(true);
                        }
                        (ElementState::Released, MouseButton::Right) => {
                            self.ctx.input.set_mouse_right_press(false);
                        }
                        (ElementState::Pressed, MouseButton::Middle) |
                        (ElementState::Pressed, MouseButton::Other(_)) |
                        (ElementState::Released, MouseButton::Middle) |
                        (ElementState::Released, MouseButton::Other(_)) => {}
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    match delta {
                        MouseScrollDelta::LineDelta(_, y) if y < 0.0 => {
                            self.ctx.input.add_mouse_scroll(-1.0);
                        }
                        MouseScrollDelta::LineDelta(_, y) if y > 0.0 => {
                            self.ctx.input.add_mouse_scroll(1.0);
                        }
                        MouseScrollDelta::LineDelta(_, _) => {}
                        MouseScrollDelta::PixelDelta(px) if px.y < 0.0 => {
                            self.ctx.input.add_mouse_scroll(-1.0);
                        }
                        MouseScrollDelta::PixelDelta(px) if px.y > 0.0 => {
                            self.ctx.input.add_mouse_scroll(1.0);
                        }
                        MouseScrollDelta::PixelDelta(_) => {}
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    self.ctx.input.set_mouse_pos(Some((position.x as u32, position.y as u32)));
                }
                WindowEvent::CursorLeft { .. } => {
                    self.ctx.input.set_mouse_pos(None);
                }
                WindowEvent::Focused(false) => {
                    self.ctx.input.release_all();
                }
                WindowEvent::Resized(physical_size) => {
                    self.wgpu.resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.wgpu.resize(*new_inner_size);
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                match self.wgpu.render() {
                    Ok(_) => {}
                    Err(wgpu::SwapChainError::Lost) => self.wgpu.recreate_swap_chain(),
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                self.process_job_results();
                let mut updated = false;
                self.timer.tick();
                self.ctx.screen_size = (self.wgpu.size.width, self.wgpu.size.height);
                while self.timer.should_update() {
                    self.game.update(&mut self.ctx);
                    updated = true;
                }
                if updated {
                    let mut renderer = Renderer::new(&mut self.wgpu);
                    self.game.draw(&mut self.ctx, &mut renderer);
                }
                self.window.request_redraw();
            }
            _ => {}
        }
    }

    fn process_job_results(&mut self) {
        while !self.should_exit {
            match self.job_results.try_recv() {
                Ok(result) => self.process_job_result(result),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("job thread died"),
            }
        }
    }

    fn process_job_result(&mut self, result: FinishedJob) {
        match result {
            FinishedJob::TextureLoaded { token, texture } => {
                self.wgpu.textures.push(texture);
                let index = Texture { index: self.wgpu.textures.len() - 1 };
                self.game.on_texture_loaded(&mut self.ctx, token, index);
            }
            FinishedJob::Quit => {
                self.job_thread.take().unwrap().join().unwrap();
                self.should_exit = true;
            }
        }
    }
}

pub fn run(game: Box<dyn Game>) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let wgpu = futures::executor::block_on(WgpuState::new(&window));
    let (job_queue, job_thread, job_results) = queue::start_job_thread(
        wgpu.device.clone(),
        wgpu.queue.clone(),
        wgpu.texture_bind_group_layout.clone(),
    );
    let screen_size = window.inner_size();
    let mut runner = GameRunner {
        game,
        wgpu,
        window,
        timer: Timer::new(),
        job_thread: Some(job_thread),
        job_results,
        ctx: Ctx {
            job_queue,
            input: Input::default(),
            screen_size: (screen_size.width, screen_size.height),
        },
        should_exit: false,
    };
    runner.game.on_start(&mut runner.ctx);

    event_loop.run(move |event, _, control_flow| runner.event(event, control_flow));
}

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
