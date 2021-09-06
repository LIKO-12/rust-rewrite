use raylib::prelude::*;
use raylib::ffi;

/// An owned texture mode, so that it can be stored in the V8 isolate.
/// Could be useful for other uses under the ownership pattern.
///
/// It owns most of the raylib handles, to promise a texture rendering mode that's available
/// as long as it exists.
///
/// Calls `begin_texture_mode` for once when it's constructed,
/// and `end_texture_mode` for once when it's destructed.
///
/// And lives as an owned value.
///
/// Instead of dropping it,
/// you'll want to extract the RaylibHandle, RaylibThread and RenderTexture2D out of it.
pub struct OwnedTextureMode {
    handle: RaylibHandle,
    thread: RaylibThread,
    texture: RenderTexture2D,
}

impl OwnedTextureMode {
    pub fn begin(handle: RaylibHandle, thread: RaylibThread, mut texture: RenderTexture2D) -> Self {
        {
            let framebuffer: &mut ffi::RenderTexture2D = &mut texture;
            unsafe { ffi::BeginTextureMode(*framebuffer) }
        }

        Self { handle, thread, texture }
    }

    pub fn end(self) -> (RaylibHandle, RaylibThread, RenderTexture2D) {
        let Self {handle, thread, texture} = self;
        (handle, thread, texture)
    }
}

impl RaylibDraw for OwnedTextureMode {}

impl Drop for OwnedTextureMode {
    fn drop(&mut self) {
        unsafe { ffi::EndTextureMode() }
    }
}