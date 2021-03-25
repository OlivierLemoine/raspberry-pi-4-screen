#![no_std]
use hal::mailbox::*;

pub struct Screen {
    ptr: *mut u8,
    size: tag_res::Size,
}
impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let width = width as u32;
        let height = height as u32;
        let message = Message::new()
            .with_tag(tag::SetPhysicalSize { width, height })
            .with_tag(tag::SetVirtualSize { width, height })
            .with_tag(tag::SetDepth(16))
            .with_tag(tag::AllocateBuffer)
            .commit()
            .unwrap();
        let ptr = message.get::<tag::AllocateBuffer>().unwrap();

        let message = Message::new()
            .with_tag(tag::GetPitch)
            .with_tag(tag::GetPhysicalSize)
            .with_tag(tag::GetVirtualSize)
            .with_tag(tag::GetDepth)
            .commit()
            .unwrap();

        let physical_size = message.get::<tag::GetPhysicalSize>().unwrap();
        let _virtual_size = message.get::<tag::GetVirtualSize>().unwrap();
        let depth = message.get::<tag::GetDepth>().unwrap();
        let pitch = message.get::<tag::GetPitch>().unwrap();

        hal::eprintln!(
            "Screen at {}, {}x{}, with {} bit per pixel. Pitch = {}.",
            ptr.ptr as u64,
            physical_size.width,
            physical_size.height,
            depth,
            pitch
        );

        Screen {
            ptr: ptr.ptr,
            size: physical_size,
        }
    }
}
