//! Framebuffer info

#[repr(C)]
pub struct ColorInfoPaletteEntry {
    red_value: u8,
    green_value: u8,
    blue_value: u8
}

#[repr(C)]
struct ColorInfoPalette {
    framebuffer_palette_num_colors: u32,
    framebuffer_palette: [ColorInfoPaletteEntry; 1]
}


#[repr(C)]
#[derive(Debug)]
struct ColorInfoRgb {
    framebuffer_red_field_position: u8,
    framebuffer_red_mask_size: u8,
    framebuffer_green_field_position: u8,
    framebuffer_green_mask_size: u8,
    framebuffer_blue_field_position: u8,
    framebuffer_blue_mask_size: u8,
}

pub enum FramebufferType {
    IndexedColor(&'static [ColorInfoPaletteEntry]),
    DirectRgb {
        framebuffer_red_field_position: u8,
        framebuffer_red_mask_size: u8,
        framebuffer_green_field_position: u8,
        framebuffer_green_mask_size: u8,
        framebuffer_blue_field_position: u8,
        framebuffer_blue_mask_size: u8,
    },
    EGAText
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct FramebufferInfoTag {
    typ: u32,
    size: u32,
    framebuffer_addr: u64,
    framebuffer_pitch: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u8,
    framebuffer_type: u8,
    reserved: u8,
    color_info: [u8]
}

impl FramebufferInfo {
    /// Framebuffer physical address. This field is 64-bit wide but bootloader should set
    /// it under 4GiB if possible for compatibility with payloads which aren't aware of PAE or
    /// amd64.
    pub fn framebuffer_addr(&self) -> usize {
        self.framebuffer_addr as usize
    }

    /// Contains pitch in bytes.
    pub fn framebuffer_pitch(&self) -> u32 {
        self.framebuffer_pitch
    }

    /// Framebuffer width and height. Those are in pixels, unless framebuffer_type is set to 2, in
    /// which case they are in characters.
    pub fn framebuffer_dimensions(&self) -> (u32, u32) {
        (self.framebuffer_width, self.framebuffer_height)
    }

    /// Number of bits per pixel
    pub fn framebuffer_bpp(&self) -> u8 {
        self.framebuffer_bpp
    }

    /// The type of framebuffer provided by the bootloader.
    pub fn framebuffer_type(&self) -> FramebufferType {
        if self.framebuffer_type == 0 {
            let color_info_num = &self.color_info as *const [u8] as *const u8 as *const ColorInfoPalette;
            let slice: &[ColorInfoPaletteEntry] = unsafe {
                ::core::slice::from_raw_parts(&(*color_info_num).framebuffer_palette[0], (*color_info_num).framebuffer_palette_num_colors as usize)
            };
            FramebufferType::IndexedColor(slice)
        } else if self.framebuffer_type == 1 {
            let color_info = &self.color_info as *const [u8] as *const u8 as *const ColorInfoRgb;
            FramebufferType::DirectRgb {
                framebuffer_red_field_position: (*color_info).framebuffer_red_field_position,
                framebuffer_red_mask_size: (*color_info).framebuffer_red_mask_size,
                framebuffer_green_field_position: (*color_info).framebuffer_green_field_position,
                framebuffer_green_mask_size: (*color_info).framebuffer_green_mask_size,
                framebuffer_blue_field_position: (*color_info).framebuffer_blue_field_position,
                framebuffer_blue_mask_size: (*color_info).framebuffer_blue_mask_size,
            }
        } else if self.framebuffer_type == 2 {
            FramebufferType::EGAText
        } else {
            panic!("Unknown framebuffer type {}", self.framebuffer_type);
        }
    }
}
