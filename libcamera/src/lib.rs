#![warn(rust_2018_idioms)]

pub mod camera;
pub mod camera_manager;
pub mod control;
pub mod control_value;
pub mod framebuffer;
pub mod framebuffer_allocator;
pub mod framebuffer_map;
pub mod geometry;
pub mod logging;
pub mod pixel_format;
pub mod request;
pub mod stream;
pub mod utils;

mod generated;
pub use generated::*;

// Add conditional modules or functions for 0.4.0 compatibility
#[cfg(libcamera_0_4)]
mod compat_0_4;

// Example of a compatibility wrapper around a changed API
#[cfg(libcamera_0_4)]
pub struct FrameBufferAllocator {
    // Updated implementation for 0.4.0
}

#[cfg(not(libcamera_0_4))]
pub struct FrameBufferAllocator {
    // Original implementation
}

// For each API component that changed in 0.4.0, add conditional implementations
// Example:
#[cfg(libcamera_0_4)]
impl FrameBufferAllocator {
    // 0.4.0 compatible implementation
    pub fn new(camera: &Camera) -> Result<Self, Error> {
        // Implementation for 0.4.0
    }
}

#[cfg(not(libcamera_0_4))]
impl FrameBufferAllocator {
    // Original implementation
    pub fn new(camera: &Camera) -> Result<Self, Error> {
        // Original implementation
    }
}
