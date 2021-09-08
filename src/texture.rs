use std::path::Path;
use image::GenericImageView;

pub trait TextureProcessor: Send + Sync {
    fn process_texture(&self, rgba: &mut [u8]);
}

pub struct MakeTransparent(pub u8, pub u8, pub u8);

impl TextureProcessor for MakeTransparent {
    fn process_texture(&self, rgba: &mut [u8]) {
        for pixel in rgba.chunks_mut(4) {
            match *pixel {
                [r, g, b, 255] if r == self.0 && g == self.1 && b == self.2 => {
                    pixel.copy_from_slice(&[0, 0, 0, 0]);
                }
                [_, _, _, _] => {}
                _ => unreachable!(),
            }
        }
    }
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_file(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &Path,
        processors: &[&dyn TextureProcessor],
    ) -> Texture {
        let image = image::io::Reader::open(path)
            .unwrap_or_else(|e| panic!("failed to open {}: {}", path.display(), e))
            .decode()
            .unwrap_or_else(|e| panic!("failed to load {}: {}", path.display(), e));
        let label = &path.display().to_string();
        Self::from_image(device, queue, &image, label, processors)
    }

    // pub fn from_bytes(
    //     device: &wgpu::Device,
    //     queue: &wgpu::Queue,
    //     bytes: &[u8],
    //     label: &str,
    //     processors: &[&dyn TextureProcessor],
    // ) -> Texture {
    //     let img = image::load_from_memory(bytes).unwrap();
    //     Self::from_image(device, queue, &img, label, processors)
    // }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &image::DynamicImage,
        label: &str,
        processors: &[&dyn TextureProcessor],
    ) -> Texture {
        let (width, height) = image.dimensions();
        let rgba = image.to_rgba8().into_raw();
        Self::from_rgba(device, queue, rgba, width, height, Some(label), processors)
    }

    pub fn from_rgba(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mut rgba: Vec<u8>,
        width: u32,
        height: u32,
        label: Option<&str>,
        processors: &[&dyn TextureProcessor],
    ) -> Texture {
        assert_eq!((width * height * 4) as usize, rgba.len());
        for processor in processors {
            processor.process_texture(&mut rgba);
        }
        let rgba = rgba.as_slice();
        let size = wgpu::Extent3d {
            width,
            height,
            depth: 1,
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            }
        );

        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * width,
                rows_per_image: height,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        Self { width, height, texture, view, sampler }
    }

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
    
    pub fn create_depth_texture(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor, label: &str) -> Self {
        let width = sc_desc.width;
        let height = sc_desc.height;
        let size = wgpu::Extent3d {
            width,
            height,
            depth: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT
                | wgpu::TextureUsage::SAMPLED,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::Less),
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        Self { width, height, texture, view, sampler }
    }
}

pub struct BindableTexture {
    pub texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

impl BindableTexture {
    pub fn from_texture(
        texture: Texture,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> BindableTexture {
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        BindableTexture { texture, bind_group }
    }
}
