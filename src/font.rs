use glam::{Mat4, UVec3, Vec2, Vec3, Vec4};
use rusterizer::Vertex;

use crate::Mesh;
use crate::Texture;
use std::collections::HashMap;
use std::path::Path;

pub struct Font {
    pub(crate) texture: Texture,
    pub(crate) symbol_size: u32,
    pub(crate) symbol_pos_map: HashMap<char, u32>,
    pub(crate) to_render: Vec<Mesh>,
}

impl Default for Font {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(char::from(' '), 0);
        map.insert(char::from('!'), 1);
        map.insert(char::from('"'), 2);
        map.insert(char::from('#'), 3);
        map.insert(char::from('$'), 4);
        map.insert(char::from('%'), 5);
        map.insert(char::from('&'), 6);
        map.insert(char::from('\''), 7);
        map.insert(char::from('('), 8);
        map.insert(char::from(')'), 9);
        map.insert(char::from('*'), 10);
        map.insert(char::from('+'), 11);
        map.insert(char::from(','), 12);
        map.insert(char::from('-'), 13);
        map.insert(char::from('.'), 14);
        map.insert(char::from('/'), 15);
        map.insert(char::from('0'), 16);
        map.insert(char::from('1'), 17);
        map.insert(char::from('2'), 18);
        map.insert(char::from('3'), 19);
        map.insert(char::from('4'), 20);
        map.insert(char::from('5'), 21);
        map.insert(char::from('6'), 22);
        map.insert(char::from('7'), 23);
        map.insert(char::from('8'), 24);
        map.insert(char::from('9'), 25);
        map.insert(char::from(':'), 26);
        map.insert(char::from(';'), 27);
        map.insert(char::from('<'), 28);
        map.insert(char::from('='), 29);
        map.insert(char::from('>'), 30);
        map.insert(char::from('?'), 31);
        map.insert(char::from('@'), 32);
        map.insert(char::from('a'), 33);
        map.insert(char::from('A'), 33);
        map.insert(char::from('b'), 34);
        map.insert(char::from('B'), 34);
        map.insert(char::from('c'), 35);
        map.insert(char::from('C'), 35);
        map.insert(char::from('d'), 36);
        map.insert(char::from('D'), 36);
        map.insert(char::from('e'), 37);
        map.insert(char::from('E'), 37);
        map.insert(char::from('f'), 38);
        map.insert(char::from('F'), 38);
        map.insert(char::from('g'), 39);
        map.insert(char::from('G'), 39);
        map.insert(char::from('h'), 40);
        map.insert(char::from('H'), 40);
        map.insert(char::from('i'), 41);
        map.insert(char::from('I'), 41);
        map.insert(char::from('j'), 42);
        map.insert(char::from('J'), 42);
        map.insert(char::from('k'), 43);
        map.insert(char::from('K'), 43);
        map.insert(char::from('l'), 44);
        map.insert(char::from('L'), 44);
        map.insert(char::from('m'), 45);
        map.insert(char::from('M'), 45);
        map.insert(char::from('n'), 46);
        map.insert(char::from('N'), 46);
        map.insert(char::from('o'), 47);
        map.insert(char::from('O'), 47);
        map.insert(char::from('p'), 48);
        map.insert(char::from('P'), 48);
        map.insert(char::from('q'), 49);
        map.insert(char::from('Q'), 49);
        map.insert(char::from('r'), 50);
        map.insert(char::from('R'), 50);
        map.insert(char::from('s'), 51);
        map.insert(char::from('S'), 51);
        map.insert(char::from('t'), 52);
        map.insert(char::from('T'), 52);
        map.insert(char::from('u'), 53);
        map.insert(char::from('U'), 53);
        map.insert(char::from('v'), 54);
        map.insert(char::from('V'), 54);
        map.insert(char::from('w'), 55);
        map.insert(char::from('W'), 55);
        map.insert(char::from('x'), 56);
        map.insert(char::from('X'), 56);
        map.insert(char::from('y'), 57);
        map.insert(char::from('Y'), 57);
        map.insert(char::from('z'), 58);
        map.insert(char::from('Z'), 58);
        map.insert(char::from('['), 59);
        map.insert(char::from('\\'), 60);
        map.insert(char::from(']'), 61);
        map.insert(char::from('^'), 62);
        map.insert(char::from('_'), 63);
        map.insert(char::from('|'), 92);
        map.insert(char::from('~'), 94);
        map.insert(char::from('á'), 102);

        let font_texture = Texture::load(Path::new("assets/fonts/outline_cute.png"));

        Self {
            texture: font_texture,
            symbol_size: 20,
            symbol_pos_map: map,
            to_render: Vec::new(),
        }
    }
}

// only supporting lowercase for now
impl Font {
    pub fn text(&mut self, text: String, pos: Vec2) {
        let symbol_percentage_x = self.symbol_size as f32 / self.texture.width as f32;
        let symbol_percentage_y = self.symbol_size as f32 / self.texture.height as f32;

        let mut v0 = Vertex {
            pos: Vec4::new(pos.x, pos.y, 1.0, 1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            color: Vec3::new(1.0, 1.0, 1.0),
            uv: glam::vec2(0.0, 0.0),
        };
        let mut v1 = Vertex {
            pos: Vec4::new(pos.x, pos.y + symbol_percentage_y, 1.0, 1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            color: Vec3::new(1.0, 1.0, 1.0),
            uv: glam::vec2(0.0, 1.0),
        };
        let mut v2 = Vertex {
            pos: Vec4::new(pos.x + symbol_percentage_x, pos.y, 1.0, 1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            color: Vec3::new(1.0, 1.0, 1.0),
            uv: glam::vec2(1.0, 0.0),
        };
        let mut v3 = Vertex {
            pos: Vec4::new(
                pos.x + symbol_percentage_x,
                pos.y + symbol_percentage_y,
                1.0,
                1.0,
            ),
            normal: Vec3::new(0.0, 0.0, 1.0),
            color: Vec3::new(1.0, 1.0, 1.0),
            uv: glam::vec2(1.0, 1.0),
        };

        let image_number_of_symbols_width = self.texture.width as u32 / self.symbol_size;
        let image_number_of_symbols_height = self.texture.height as u32 / self.symbol_size;

        for (i, char) in text.chars().enumerate() {
            let screen_offset_x = i as f32 * symbol_percentage_x;

            v0.pos.x += screen_offset_x;
            v1.pos.x += screen_offset_x;
            v2.pos.x += screen_offset_x;
            v3.pos.x += screen_offset_x;

            if let Some(symbol_index) = self.symbol_pos_map.get(&char) {
                let texture_pos_x = symbol_index % image_number_of_symbols_width;
                let texture_pos_y = symbol_index / image_number_of_symbols_width;

                v0.uv.x = texture_pos_x as f32 / image_number_of_symbols_width as f32;
                v0.uv.y = texture_pos_y as f32 / image_number_of_symbols_height as f32;
                v1.uv.x = v0.uv.x;
                v1.uv.y = v0.uv.y + symbol_percentage_y;
                v2.uv.x = v0.uv.x + symbol_percentage_x;
                v2.uv.y = v0.uv.y;
                v3.uv.x = v2.uv.x;
                v3.uv.y = v1.uv.y;

                self.to_render.push(Mesh {
                    triangle_indices: vec![UVec3::new(0, 1, 2), UVec3::new(2, 1, 3)],
                    vertices: vec![v0, v1, v2, v3],
                });
            } else {
                println!("Symbol \"{}\" is not supported yet!", char);
            }

            // reset positions
            v0.pos.x -= screen_offset_x;
            v1.pos.x -= screen_offset_x;
            v2.pos.x -= screen_offset_x;
            v3.pos.x -= screen_offset_x;
        }
    }

    pub fn render(
        &mut self,
        buffer: &mut Vec<u32>,
        z_buffer: &mut Vec<f32>,
        mvp: &Mat4,
        viewport_size: Vec2,
    ) {
        let identity = Mat4::IDENTITY;
        for quad in &self.to_render {
            rusterizer::raster_mesh_for_text(
                quad,
                &identity,
                mvp,
                Some(&self.texture),
                buffer,
                z_buffer,
                viewport_size,
            );
        }
        self.to_render.clear();
    }
}
