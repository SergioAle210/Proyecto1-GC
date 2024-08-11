use crate::player::Player;
use gilrs::{Button, Event, EventType, Gilrs};
use minifb::{Key, Window};

pub fn process_events(
    window: &Window,
    player: &mut Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    gilrs: &mut Gilrs,
) {
    // Existing keyboard input handling
    if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
        let new_x = player.pos.x + player.dir.x * player.speed;
        let new_y = player.pos.y + player.dir.y * player.speed;

        if !is_collision(new_x, new_y, maze, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }

    if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
        let new_x = player.pos.x - player.dir.x * player.speed;
        let new_y = player.pos.y - player.dir.y * player.speed;

        if !is_collision(new_x, new_y, maze, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }

    if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
        player.angle -= player.rotation_speed;
        if player.angle < 0.0 {
            player.angle += 2.0 * std::f32::consts::PI;
        }
        update_direction(player);
    }

    if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
        player.angle += player.rotation_speed;
        if player.angle >= 2.0 * std::f32::consts::PI {
            player.angle -= 2.0 * std::f32::consts::PI;
        }
        update_direction(player);
    }

    // Handle gamepad input
    while let Some(Event { event, .. }) = gilrs.next_event() {
        match event {
            EventType::ButtonPressed(Button::DPadUp, ..) => {
                let new_x = player.pos.x + player.dir.x * player.speed;
                let new_y = player.pos.y + player.dir.y * player.speed;
                if !is_collision(new_x, new_y, maze, block_size) {
                    player.pos.x = new_x;
                    player.pos.y = new_y;
                }
            }
            EventType::ButtonPressed(Button::DPadDown, ..) => {
                let new_x = player.pos.x - player.dir.x * player.speed;
                let new_y = player.pos.y - player.dir.y * player.speed;
                if !is_collision(new_x, new_y, maze, block_size) {
                    player.pos.x = new_x;
                    player.pos.y = new_y;
                }
            }
            EventType::ButtonPressed(Button::DPadLeft, ..) => {
                player.angle -= player.rotation_speed;
                if player.angle < 0.0 {
                    player.angle += 2.0 * std::f32::consts::PI;
                }
                update_direction(player);
            }
            EventType::ButtonPressed(Button::DPadRight, ..) => {
                player.angle += player.rotation_speed;
                if player.angle >= 2.0 * std::f32::consts::PI {
                    player.angle -= 2.0 * std::f32::consts::PI;
                }
                update_direction(player);
            }
            _ => {}
        }
    }
}

pub fn is_collision(x: f32, y: f32, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
    let maze_x = (x * block_size as f32) as usize / block_size;
    let maze_y = (y * block_size as f32) as usize / block_size;

    // Verificar si los índices están dentro de los límites del laberinto
    if maze_x >= maze[0].len() || maze_y >= maze.len() {
        return true; // Considerar fuera de los límites como una colisión
    }

    // No considerar 'g' como una pared
    maze[maze_y][maze_x] != ' ' && maze[maze_y][maze_x] != 'g'
}

pub fn has_won(x: f32, y: f32, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
    let maze_x = (x * block_size as f32) as usize / block_size;
    let maze_y = (y * block_size as f32) as usize / block_size;

    maze[maze_y][maze_x] == 'g'
}

pub fn update_direction(player: &mut Player) {
    player.dir.x = player.angle.cos();
    player.dir.y = player.angle.sin();
}
