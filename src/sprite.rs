use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::input::is_collision;
use crate::player::Player;
use crate::texture::Texture;

pub struct Sprite {
    pub texture: Texture,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub speed: f32,
}

impl Sprite {
    pub fn new(texture_path: &str, maze_x: f32, maze_y: f32, size: f32, speed: f32) -> Self {
        let texture = Texture::new(texture_path);
        Sprite {
            texture,
            x: maze_x,
            y: maze_y,
            size,
            speed,
        }
    }

    pub fn move_towards_player(
        &mut self,
        player: &Player,
        maze: &Vec<Vec<char>>,
        block_size: usize,
    ) {
        let dx = player.pos.x - self.x;
        let dy = player.pos.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.0 {
            // Normalizar la dirección y aplicar la velocidad
            let direction_x = dx / distance;
            let direction_y = dy / distance;

            let mut new_x = self.x + direction_x * self.speed;
            let mut new_y = self.y + direction_y * self.speed;

            // Verificar colisiones con el laberinto antes de mover el sprite
            if is_collision(new_x, self.y, maze, block_size) {
                // Intentar moverse perpendicularmente si hay colisión en la dirección X
                new_x = self.x;
                new_y += self.speed * if direction_y < 0.0 { -1.0 } else { 1.0 };
            }
            if is_collision(self.x, new_y, maze, block_size) {
                // Intentar moverse perpendicularmente si hay colisión en la dirección Y
                new_y = self.y;
                new_x += self.speed * if direction_x < 0.0 { -1.0 } else { 1.0 };
            }

            // Si ambas direcciones están bloqueadas, retroceder
            if is_collision(new_x, self.y, maze, block_size)
                && is_collision(self.x, new_y, maze, block_size)
            {
                new_x = self.x - direction_x * self.speed;
                new_y = self.y - direction_y * self.speed;
            }

            // Finalmente, aplicar el movimiento si no hay colisión
            if !is_collision(new_x, self.y, maze, block_size) {
                self.x = new_x;
            }

            if !is_collision(self.x, new_y, maze, block_size) {
                self.y = new_y;
            }
        }
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, player: &Player) {
        let dx = self.x - player.pos.x;
        let dy = self.y - player.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 0.5 || distance > 50.0 {
            // No renderizar si el sprite está demasiado cerca o demasiado lejos
            return;
        }

        // Calcular el ángulo entre el jugador y el sprite
        let sprite_angle = (dy).atan2(dx);
        let mut angle_diff = (sprite_angle - player.angle).rem_euclid(2.0 * std::f32::consts::PI);

        // Asegurarse de que el ángulo de diferencia esté dentro del campo de visión del jugador
        if angle_diff > std::f32::consts::PI {
            angle_diff -= 2.0 * std::f32::consts::PI;
        }

        if angle_diff.abs() > player.fov / 2.0 {
            // El sprite está fuera del campo de visión, no renderizar
            return;
        }

        // Proyección del sprite en la pantalla
        let screen_x = framebuffer.width as f32 / 2.0 * (1.0 + angle_diff / player.fov);
        let sprite_height = ((framebuffer.height as f32 / distance) * self.size).max(1.0) as usize;
        let sprite_width = sprite_height; // Mantener proporciones cuadradas

        let start_x = (screen_x as isize - sprite_width as isize / 2).max(0) as usize;
        let end_x = (start_x + sprite_width).min(framebuffer.width);

        let start_y = ((framebuffer.height as isize / 2 - sprite_height as isize / 2).max(0)
            as usize)
            .min(framebuffer.height);
        let end_y = (start_y + sprite_height).min(framebuffer.height);

        // Dibujar el sprite en la pantalla
        for y in start_y..end_y {
            for x in start_x..end_x {
                let tx = (x - start_x) * self.texture.width as usize / sprite_width;
                let ty = (y - start_y) * self.texture.height as usize / sprite_height;
                let color = self.texture.get_pixel(tx, ty);
                if color != Color::new(0, 0, 0) {
                    // No dibujar píxeles transparentes (negros)
                    framebuffer.point_with_color(x, y, color);
                }
            }
        }
    }
}
