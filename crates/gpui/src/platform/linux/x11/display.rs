use anyhow::Result;
use uuid::Uuid;
use x11rb::{connection::Connection as _, xcb_ffi::XCBConnection};

use crate::{Bounds, DisplayId, GlobalPixels, PlatformDisplay, Size};

#[derive(Debug)]
pub(crate) struct X11Display {
    x_screen_index: usize,
    bounds: Bounds<GlobalPixels>,
    uuid: Uuid,
}

impl X11Display {
    pub(crate) fn new(xc: &XCBConnection, x_screen_index: usize) -> Option<Self> {
        let screen = xc.setup().roots.get(x_screen_index).unwrap();
        Some(Self {
            x_screen_index: x_screen_index,
            bounds: Bounds {
                origin: Default::default(),
                size: Size {
                    width: GlobalPixels(screen.width_in_pixels as f32),
                    height: GlobalPixels(screen.height_in_pixels as f32),
                },
            },
            uuid: Uuid::from_bytes([0; 16]),
        })
    }
}

impl PlatformDisplay for X11Display {
    fn id(&self) -> DisplayId {
        DisplayId(self.x_screen_index as u32)
    }

    fn uuid(&self) -> Result<Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<GlobalPixels> {
        self.bounds
    }
}
