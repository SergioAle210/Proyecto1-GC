use crate::player::Player;
use gilrs::{Button, Event, EventType, Gilrs};
use minifb::{Key, Window};
use rodio::OutputStreamHandle; // Import the stream handle
use rodio::{Decoder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub fn play_footstep_sound(
    stream_handle: &OutputStreamHandle,
    footstep_sink: &Arc<Mutex<Option<Sink>>>,
) {
    let mut sink_guard = footstep_sink.lock().unwrap();

    // Check if the footstep sound is already playing; if so, do nothing
    if sink_guard.is_none() {
        // Load the footstep sound
        let file =
            File::open("./assets/footsteps.mp3").expect("Failed to open footstep sound file");
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode footstep audio");

        // Reduce the volume by applying an amplification factor
        let source = source.amplify(0.8); // Adjust this value as needed

        // Create a new sink to play the sound
        let sink = Sink::try_new(stream_handle).expect("Failed to create footstep sink");
        sink.append(source.repeat_infinite()); // Play the sound in a loop

        // Start playing the sound
        sink.play();

        // Store the sink in the Arc<Mutex<Option<Sink>>>
        *sink_guard = Some(sink);
    }
}

pub fn stop_footstep_sound(footstep_sink: &Arc<Mutex<Option<Sink>>>) {
    let mut sink_guard = footstep_sink.lock().unwrap();

    // Stop and drop the sink if it exists
    if let Some(sink) = sink_guard.take() {
        sink.stop(); // Stop the sound
    }
}

pub fn process_events(
    window: &Window,
    player: &mut Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    gilrs: &mut Gilrs,
    stream_handle: &OutputStreamHandle, // Add the stream handle as a parameter
    footstep_sink: &Arc<Mutex<Option<Sink>>>, // Add footstep_sink as a parameter
) {
    let mut player_moved = false;

    // Handle keyboard input
    if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
        let new_x = player.pos.x + player.dir.x * player.speed;
        let new_y = player.pos.y + player.dir.y * player.speed;

        if !is_collision(new_x, new_y, maze, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
            player_moved = true;
        }
    }

    if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
        let new_x = player.pos.x - player.dir.x * player.speed;
        let new_y = player.pos.y - player.dir.y * player.speed;

        if !is_collision(new_x, new_y, maze, block_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
            player_moved = true;
        }
    }

    if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
        player.angle -= player.rotation_speed;
        if player.angle < 0.0 {
            player.angle += 2.0 * std::f32::consts::PI;
        }
        update_direction(player);
        player_moved = true;
    }

    if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
        player.angle += player.rotation_speed;
        if player.angle >= 2.0 * std::f32::consts::PI {
            player.angle -= 2.0 * std::f32::consts::PI;
        }
        update_direction(player);
        player_moved = true;
    }

    // If the player moved, play the footstep sound
    if player_moved {
        play_footstep_sound(&stream_handle, &footstep_sink);
    } else {
        // Si el jugador no se movió, detén el sonido de los pasos
        stop_footstep_sound(&footstep_sink);
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
                    player_moved = true;
                }
            }
            EventType::ButtonPressed(Button::DPadDown, ..) => {
                let new_x = player.pos.x - player.dir.x * player.speed;
                let new_y = player.pos.y - player.dir.y * player.speed;
                if !is_collision(new_x, new_y, maze, block_size) {
                    player.pos.x = new_x;
                    player.pos.y = new_y;
                    player_moved = true;
                }
            }
            EventType::ButtonPressed(Button::DPadLeft, ..) => {
                player.angle -= player.rotation_speed;
                if player.angle < 0.0 {
                    player.angle += 2.0 * std::f32::consts::PI;
                }
                update_direction(player);
                player_moved = true;
            }
            EventType::ButtonPressed(Button::DPadRight, ..) => {
                player.angle += player.rotation_speed;
                if player.angle >= 2.0 * std::f32::consts::PI {
                    player.angle -= 2.0 * std::f32::consts::PI;
                }
                update_direction(player);
                player_moved = true;
            }
            _ => {}
        }
    }

    // If the player moved using the gamepad, play the footstep sound
    if player_moved {
        play_footstep_sound(&stream_handle, &footstep_sink);
    } else {
        // Si el jugador no se movió, detén el sonido de los pasos
        stop_footstep_sound(&footstep_sink);
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
