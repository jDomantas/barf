use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
        mpsc::{Receiver, Sender, channel},
    },
    thread::JoinHandle,
};
use crate::{
    TextureProcessor,
    texture::{BindableTexture, Texture},
};

#[derive(Clone)]
pub struct JobQueue {
    inner: Arc<JobQueueInner>,
}

impl JobQueue {
    pub fn load_texture_from_file(
        &self,
        path: PathBuf,
        processors: &'static [&'static dyn TextureProcessor],
    ) -> JobToken {
        let token = self.inner.generate_token();
        self.send(JobRequest::LoadTextureFromFile(token, path, processors));
        token
    }

    pub fn load_texture_from_rgba(
        &self,
        rgba: Vec<u8>,
        width: u32,
        height: u32,
        processors: &'static [&'static dyn TextureProcessor],
    ) -> JobToken {
        assert_eq!((width * height * 4) as usize, rgba.len());
        let token = self.inner.generate_token();
        self.send(JobRequest::LoadTextureFromRgba(token, rgba, width, height, processors));
        token
    }

    pub fn quit(&self) {
        self.send(JobRequest::Quit);
    }

    fn send(&self, request: JobRequest) {
        self.inner.sender.send(request).ok();
    }
}

struct JobQueueInner {
    sender: Sender<JobRequest>,
    token_generator: AtomicU64,
}

impl JobQueueInner {
    fn generate_token(&self) -> JobToken {
        let token = self.token_generator.fetch_add(1, Ordering::Relaxed);
        JobToken(token)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct JobToken(u64);

enum JobRequest {
    Quit,
    LoadTextureFromFile(JobToken, PathBuf, &'static [&'static dyn TextureProcessor]),
    LoadTextureFromRgba(JobToken, Vec<u8>, u32, u32, &'static [&'static dyn TextureProcessor]),
}

pub(crate) enum FinishedJob {
    TextureLoaded {
        token: JobToken,
        texture: BindableTexture,
    },
    Quit,
}

pub(crate) fn start_job_thread(
    gpu_device: Arc<wgpu::Device>,
    gpu_queue: Arc<wgpu::Queue>,
    texture_bind_group_layout: Arc<wgpu::BindGroupLayout>,
) -> (JobQueue, JoinHandle<()>, Receiver<FinishedJob>) {
    let (job_tx, job_rx) = channel();
    let (result_tx, result_rx) = channel();
    let mut runner = JobRunner {
        result_sender: result_tx,
        gpu_device,
        gpu_queue,
        texture_bind_group_layout,
    };
    let handle = std::thread::spawn(move || {
        for job in job_rx {
            match job {
                JobRequest::Quit => {
                    runner.quit();
                    break;
                }
                JobRequest::LoadTextureFromFile(token, path, processors) => {
                    runner.load_texture_from_file(token, &path, processors);
                }
                JobRequest::LoadTextureFromRgba(token, rgba, width, height, processors) => {
                    runner.load_texture_from_rgba(token, rgba, width, height, processors);
                }
            }
        }
    });
    let queue = JobQueue {
        inner: Arc::new(JobQueueInner {
            sender: job_tx,
            token_generator: AtomicU64::new(0),
        }),
    };
    (queue, handle, result_rx)
}

struct JobRunner {
    result_sender: Sender<FinishedJob>,
    gpu_device: Arc<wgpu::Device>,
    gpu_queue: Arc<wgpu::Queue>,
    texture_bind_group_layout: Arc<wgpu::BindGroupLayout>,
}

impl JobRunner {
    fn quit(self) {
        self.result_sender.send(FinishedJob::Quit).unwrap();
    }

    fn load_texture_from_file(
        &mut self,
        token: JobToken,
        path: &Path,
        processors: &[&dyn TextureProcessor],
    ) {
        let texture = Texture::from_file(&self.gpu_device, &self.gpu_queue, path, processors);
        let texture = BindableTexture::from_texture(texture, &self.gpu_device, &self.texture_bind_group_layout);
        self.result_sender.send(FinishedJob::TextureLoaded { token, texture }).unwrap();
    }

    fn load_texture_from_rgba(
        &mut self,
        token: JobToken,
        rgba: Vec<u8>,
        width: u32,
        height: u32,
        processors: &[&dyn TextureProcessor],
    ) {
        let texture = Texture::from_rgba(&self.gpu_device, &self.gpu_queue, rgba, width, height, None, processors);
        let texture = BindableTexture::from_texture(texture, &self.gpu_device, &self.texture_bind_group_layout);
        self.result_sender.send(FinishedJob::TextureLoaded { token, texture }).unwrap();
    }
}
