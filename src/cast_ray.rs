use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::texture::Texture;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: usize,
}

pub fn cast_rays(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    angle: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;

    loop {
        let cos = d * angle.cos();
        let sin = d * angle.sin();
        let x = (player.pos.x * block_size as f32 + cos) as usize;
        let y = (player.pos.y * block_size as f32 + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if draw_line {
            framebuffer.point_with_color(x, y, Color::from_hex(0xFF33DD)); // Dibuja el punto del rayo
        }

        if maze[j][i] != ' ' {
            let hitx = x - i * block_size;
            let hity = y - j * block_size;
            let mut maxhit = hity;

            if 1 < hitx && hitx < block_size - 1 {
                maxhit = hitx;
            }

            return Intersect {
                distance: d,
                impact: maze[j][i],
                tx: maxhit * 512 / block_size,
            };
        }

        d += 1.0; // Incrementa d en pequeños pasos para mayor precisión
    }
}
