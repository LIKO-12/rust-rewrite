use raylib::prelude::*;
use raylib::ffi;
use std::ops::Deref;

// TODO: Others might benefit of this by contributing it to a binding.
// TODO: Or maybe create an ownership based bindings of raylib-rs.

/// An owned texture mode, so that it can be stored in the V8 isolate.
/// Could be useful for other usage under the ownership pattern.
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
/// you'll want to extract the RaylibHandle, RaylibThread and RenderTexture2D out of it
/// using the [end] method.
///
/// ## Example
///
/// ```
/// // Begin the texture rendering mode.
/// let mut d = OwnedTextureMode::begin(rl, thread, screen_texture);
///
/// // Do any drawing operations on the texture.
/// d.clear_background(Color::BLACK);
///
/// // Extract the values out of it.
/// let (mut rl, thread, screen_texture) = d.end();
/// ```
pub struct OwnedTextureMode {
    handle: RaylibHandle,
    thread: RaylibThread,
    texture: RenderTexture2D,
}

impl OwnedTextureMode {
    /// Begin the texture rendering mode, giving your resources as a pledge for safety.
    pub fn begin(handle: RaylibHandle, thread: RaylibThread, mut texture: RenderTexture2D) -> Self {
        {
            let framebuffer: &mut ffi::RenderTexture2D = &mut texture;
            unsafe { ffi::BeginTextureMode(*framebuffer) }
        }

        Self { handle, thread, texture }
    }

    /// End the texture rendering mode and retrieve resources back.
    pub fn end(self) -> (RaylibHandle, RaylibThread, RenderTexture2D) {
        unsafe { ffi::EndTextureMode() }
        (self.handle, self.thread, self.texture)
    }
}

impl RaylibDraw for OwnedTextureMode {}
impl Deref for OwnedTextureMode {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}