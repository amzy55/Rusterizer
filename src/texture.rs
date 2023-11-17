use crate::utils::*;
use stb_image;
use std::path::Path;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    pub depth: usize,
}

impl Texture {
    pub fn load(path: &Path) -> Self {
        let decoded_image = stb_image::image::load(path);
        if let stb_image::image::LoadResult::ImageU8(image) = decoded_image {
            let data;
            if image.depth == 4 {
                data = (0..image.data.len() / 4)
                    .map(|id| {
                        to_argb8(
                            image.data[id * 4 + 3],
                            image.data[id * 4],
                            image.data[id * 4 + 1],
                            image.data[id * 4 + 2],
                        )
                    })
                    .collect();
            } else {
                data = (0..image.data.len() / 3)
                    .map(|id| {
                        from_u8_rgb(
                            image.data[id * 3],
                            image.data[id * 3 + 1],
                            image.data[id * 3 + 2],
                        )
                    })
                    .collect();
            }
            Self {
                width: image.width,
                height: image.height,
                data,
                depth: image.depth,
            }
        } else {
            panic!("Unsupported texture type");
        }
    }

    pub fn rgb_at_uv(&self, u: f32, v: f32) -> u32 {
        let (u, v) = (u * self.width as f32, v * self.height as f32);
        let (u, v) = (u as usize % self.width, v as usize % self.height);
        let id = u as usize + v as usize * self.width;
        if id < self.data.len() {
            self.data[id]
        } else {
            from_u8_rgb(0, 255, 255)
        }
    }
}
