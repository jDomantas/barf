use crate::{
    Rect, Texture,
    texture::BindableTexture,
    wgpu_render::{Instance, RawInstance, WgpuState},
};

pub struct Renderer<'a> {
    wgpu_state: &'a mut WgpuState,
    screen_size: (u32, u32),
    view: Rect,
}

impl<'a> Renderer<'a> {
    pub(crate) fn new(wgpu_state: &'a mut WgpuState) -> Renderer<'a> {
        wgpu_state.instances.clear();
        let screen_size = (wgpu_state.size.width, wgpu_state.size.height);
        let view = Rect {
            x: 0.0,
            y: 0.0,
            w: screen_size.0 as f32,
            h: screen_size.1 as f32,
        };
        Renderer {
            wgpu_state,
            screen_size,
            view,
        }
    }

    pub fn draw(&mut self, texture: Texture, dest: Rect) -> DrawBuilder<'_> {
        let (pos, size) = {
            let Rect { x, y, w, h } = dest;
            let w = w / self.view.w * 2.0;
            let h = h / self.view.h * 2.0;
            let x = (x - self.view.x) / self.view.w * 2.0 - 1.0;
            let y = (y - self.view.y) / self.view.h * 2.0 - 1.0;
            ([x, y, 0.0, 0.0], [w, h])
        };
        self.wgpu_state.instances.push(Instance {
            texture,
            raw: RawInstance {
                pos,
                size,
                tex_source_pos: [0.0, 0.0],
                tex_source_size_x: [1.0, 0.0],
                tex_source_size_y: [0.0, 1.0],
                tex_color: [1.0, 1.0, 1.0, 0.0],
            },
        });
        let texture = &self.wgpu_state.textures[texture.index];
        let instance = self.wgpu_state.instances.last_mut().unwrap();
        DrawBuilder {
            texture,
            instance,
            src: Rect {
                x: 0.0,
                y: 0.0,
                w: 1.0,
                h: 1.0,
            },
            rotate: Rotate::None,
        }
    }

    pub fn set_view(&mut self, view: Rect) {
        self.view = view;
    }

    pub fn screen_size(&self) -> (u32, u32) {
        self.screen_size
    }
}

pub struct DrawBuilder<'a> {
    texture: &'a BindableTexture,
    instance: &'a mut Instance,
    src: Rect,
    rotate: Rotate,
}

impl DrawBuilder<'_> {
    pub fn src(mut self, src: Rect) -> Self {
        self.src = src;
        self
    }

    pub fn rotate(mut self, rotate: Rotate) -> Self {
        self.rotate = rotate;
        self
    }

    pub fn color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.instance.raw.tex_color = [
            (r as f32) / 255.0,
            (g as f32) / 255.0,
            (b as f32) / 255.0,
            0.0,
        ];
        self
    }
}

impl Drop for DrawBuilder<'_> {
    fn drop(&mut self) {
        let (pos, x, y) = match self.rotate {
            Rotate::None => (
                [self.src.x, self.src.y],
                [self.src.w, 0.0],
                [0.0, self.src.h],
            ),
            Rotate::R90 => (
                [self.src.x, self.src.y + self.src.h],
                [0.0, -self.src.h],
                [self.src.w, 0.0],
            ),
            Rotate::R180 => (
                [self.src.x + self.src.w, self.src.y + self.src.h],
                [-self.src.w, 0.0],
                [0.0, -self.src.h],
            ),
            Rotate::R270 => (
                [self.src.x + self.src.w, self.src.y],
                [0.0, self.src.h],
                [-self.src.w, 0.0],
            ),
        };
        let pos = [
            pos[0] / (self.texture.texture.width as f32),
            pos[1] / (self.texture.texture.height as f32),
        ];
        let x = [
            x[0] / (self.texture.texture.width as f32),
            x[1] / (self.texture.texture.height as f32),
        ];
        let y = [
            y[0] / (self.texture.texture.width as f32),
            y[1] / (self.texture.texture.height as f32),
        ];
        self.instance.raw.tex_source_pos = pos;
        self.instance.raw.tex_source_size_x = x;
        self.instance.raw.tex_source_size_y = y;
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Rotate {
    None,
    R90,
    R180,
    R270,
}