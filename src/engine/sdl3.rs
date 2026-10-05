use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::thread::ThreadId;

use anyhow::Context;

use crate::sys;

use super::core::SmgnEvent;
use super::core::SmgnEventProvider;

static SDL3_MAIN_THREAD: OnceLock<ThreadId> = OnceLock::new();
static SDL3_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct SDL3Wrapper {
    _block_send_sync: PhantomData<*mut ()>,
}

impl SDL3Wrapper {
    pub fn init() -> anyhow::Result<Self> {
        let thread_id = thread::current().id();

        let sdl3_main_thread_id = SDL3_MAIN_THREAD.get_or_init(|| thread_id);

        if *sdl3_main_thread_id != thread_id {
            anyhow::bail!("Attempt to create SDL3 from multiple threads");
        }

        let was_initialized = SDL3_INITIALIZED.swap(true, Ordering::Acquire);

        if was_initialized {
            anyhow::bail!("Attempt to create SDL3 twice");
        }

        let init_result = unsafe { sys::smgn_sdl_init() };

        if !init_result {
            SDL3_INITIALIZED.store(false, Ordering::Release);

            anyhow::bail!("Failed to initialize SDL3");
        }

        Ok(SDL3Wrapper {
            _block_send_sync: PhantomData,
        })
    }

    pub fn create_window<'sdl>(&'sdl self) -> anyhow::Result<Window<'sdl>> {
        let Some(window_ptr) = NonNull::new(unsafe { sys::smgn_sdl_window_create() }) else {
            anyhow::bail!("Failed to create SDL3 window");
        };

        Ok(Window {
            window_ptr,
            _sdl: PhantomData,
        })
    }

    pub fn get_event_provider<'sdl>(&'sdl self) -> SDL3EventProvider<'sdl> {
        SDL3EventProvider {
            _sdl: PhantomData,
            event_buf: Default::default(),
        }
    }
}

impl Drop for SDL3Wrapper {
    fn drop(&mut self) {
        unsafe { sys::smgn_sdl_quit() };

        SDL3_INITIALIZED.store(false, Ordering::Release);
    }
}

#[derive(Debug)]
pub struct Window<'sdl> {
    window_ptr: NonNull<sys::SDL_Window>,
    _sdl: PhantomData<&'sdl SDL3Wrapper>,
}

impl<'sdl> Window<'sdl> {
    pub fn create_renderer<'window>(&'window self) -> anyhow::Result<Renderer<'sdl, 'window>> {
        let Some(renderer_ptr) =
            NonNull::new(unsafe { sys::smgn_sdl_renderer_create(self.window_ptr.as_ptr()) })
        else {
            anyhow::bail!(
                "Failed to crate SDL3 renderer for window {:?}",
                self.window_ptr
            );
        };

        Ok(Renderer {
            renderer_ptr,
            _window: PhantomData,
        })
    }
}

impl Drop for Window<'_> {
    fn drop(&mut self) {
        unsafe { sys::smgn_sdl_window_nuke(self.window_ptr.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct Renderer<'sdl, 'window> {
    renderer_ptr: NonNull<sys::SDL_Renderer>,
    _window: PhantomData<&'window Window<'sdl>>,
}

impl<'sdl, 'window> Renderer<'sdl, 'window> {
    pub fn render_start(&self, r: u8, g: u8, b: u8) -> anyhow::Result<()> {
        let res = unsafe { sys::smgn_sdl_renderer_start(self.renderer_ptr.as_ptr(), r, g, b) };

        anyhow::ensure!(res, "Failed to start SDL3 rendering");

        Ok(())
    }

    pub fn render_finish(&self) -> anyhow::Result<()> {
        let res = unsafe { sys::smgn_sdl_renderer_finish(self.renderer_ptr.as_ptr()) };

        anyhow::ensure!(res, "Failed to finish SDL3 rendering");

        Ok(())
    }

    pub fn render_texture(&self, texture: &Texture) -> anyhow::Result<()> {
        let res = unsafe {
            sys::smgn_sdl_renderer_render_texture(
                self.renderer_ptr.as_ptr(),
                texture.texture_ptr.as_ptr(),
            )
        };

        anyhow::ensure!(res, "Failed to render SDL3 texture: {texture:#?}");

        Ok(())
    }

    pub fn load_texture<'renderer>(
        &'renderer self,
        path: &str,
    ) -> anyhow::Result<Texture<'sdl, 'window, 'renderer>> {
        let c_path =
            CString::new(path).context("Failed to convert texture file path to C-String")?;

        let Some(texture_ptr) = NonNull::new(unsafe {
            sys::smgn_sdl_load_texture(self.renderer_ptr.as_ptr(), c_path.as_ptr())
        }) else {
            anyhow::bail!("Failed to load SDL3 texture from file {path:?}");
        };

        Ok(Texture {
            texture_ptr,
            _renderer: PhantomData,
        })
    }
}

impl Drop for Renderer<'_, '_> {
    fn drop(&mut self) {
        unsafe { sys::smgn_sdl_renderer_nuke(self.renderer_ptr.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct SDL3EventProvider<'sdl> {
    _sdl: PhantomData<&'sdl SDL3Wrapper>,
    event_buf: Vec<SmgnEvent>,
}

impl SmgnEventProvider for SDL3EventProvider<'_> {
    fn get_event(&mut self) -> Option<SmgnEvent> {
        if let Some(event) = self.event_buf.pop() {
            Some(event)
        } else {
            loop {
                let mut sdl_event = MaybeUninit::uninit();

                let has_event = unsafe { sys::smgn_sdl_events_poll(sdl_event.as_mut_ptr()) };

                if !has_event {
                    break;
                }

                let sdl_event = unsafe { sdl_event.assume_init() };

                if let Some(smgn_event) = map_sdl_event(sdl_event) {
                    self.event_buf.push(smgn_event);
                }
            }

            self.event_buf.pop()
        }
    }
}

fn map_sdl_event(sdl_event: sys::SDL_Event) -> Option<SmgnEvent> {
    let event_type = unsafe { sdl_event.type_ };

    use sys::SDL_EventType::*;

    match event_type {
        SDL_EVENT_QUIT => Some(SmgnEvent::Quit),
        _other => None,
    }
}

#[derive(Debug)]
pub struct Texture<'sdl, 'window, 'renderer> {
    texture_ptr: NonNull<sys::SDL_Texture>,
    _renderer: PhantomData<&'renderer Renderer<'sdl, 'window>>,
}

impl Drop for Texture<'_, '_, '_> {
    fn drop(&mut self) {
        unsafe { sys::smgn_sdl_renderer_nuke_texture(self.texture_ptr.as_ptr()) };
    }
}
