use gilrs::Gilrs;

mod sprite;
use sprite::Sprite;

extern crate rodio;

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

mod timer;
use timer::Timer;

mod color;
use crate::color::Color;

mod cast_ray;
use cast_ray::cast_rays;

mod framebuffer;
use framebuffer::Framebuffer;

mod input;
use input::{has_won, process_events, update_direction};

use minifb::{Key, MouseMode, Window, WindowOptions};

mod player;
use player::Player;

mod maze;
use maze::load_maze;

mod texture;
use texture::Texture;

use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::thread;

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/WALL.jpg")));
static START_SCREEN: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/start.jpg")));
static LOST_SCREEN: Lazy<Arc<Texture>> =
    Lazy::new(|| Arc::new(Texture::new("assets/lost_backrooms.jpg")));

enum GameState {
    StartScreen,
    Playing,
    Won,
    Lost, // Estado agregado
}

fn draw_cell(framebuffer: &mut Framebuffer, x: usize, y: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' => Color::from_hex(0xD6C34E),
        '-' => Color::from_hex(0xBFAA25),
        '|' => Color::from_hex(0xB9AB53),
        'g' => Color::from_hex(0xc92828),
        ' ' => Color::from_hex(0x7F5A1B),
        _ => Color::from_hex(0x000000),
    };

    framebuffer.draw_rectangle(x, y, block_size, block_size, color);
}

fn render_start_screen(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width;
    let height = framebuffer.height;

    // Draw the start screen image
    for y in 0..height {
        for x in 0..width {
            let tx = x * START_SCREEN.width as usize / width;
            let ty = y * START_SCREEN.height as usize / height;
            let color = START_SCREEN.get_pixel(tx, ty);
            framebuffer.point_with_color(x, y, color);
        }
    }

    // Draw the "Press any key to start" text
    framebuffer.draw_text(
        "Press any key to start",
        width / 2 - 150,
        height - 100,
        Color::from_hex(0xFFFFFF),
    );
}

fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    sprite: &Sprite,
) {
    let block_size = 100;
    let num_rays = framebuffer.width;

    let hh = framebuffer.height as f32 / 2.0;
    let distance_to_projection_plane = 100.0;

    let ceiling_color = Color::from_hex(0x88814a);
    let floor_color = Color::from_hex(0x58450e);

    for y in 0..(framebuffer.height / 2) {
        for x in 0..framebuffer.width {
            framebuffer.point_with_color(x, y, ceiling_color.clone());
        }
    }

    for y in (framebuffer.height / 2)..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point_with_color(x, y, floor_color.clone());
        }
    }

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.angle - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_rays(framebuffer, maze, player, a, block_size, false);

        let distance_to_wall = intersect.distance;

        if distance_to_wall > 0.0 {
            let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
            let stake_top = (hh - (stake_height / 2.0)) as usize;
            let stake_bottom = (hh + (stake_height / 2.0)) as usize;

            for y in stake_top..stake_bottom {
                let tx = intersect.tx;
                let ty = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32))
                    * 512.0;
                let wall_color = WALL1.get_pixel(tx, ty as usize);
                framebuffer.point_with_color(i, y, wall_color);
            }
        }
    }

    // Renderizar el sprite
    sprite.render(framebuffer, player);

    // Renderizar el mini-mapa
    let mini_map_scale = 8;
    let mini_map_size = block_size / mini_map_scale;
    let mini_map_x_offset = framebuffer.width - maze[0].len() * mini_map_size - 10;
    let mini_map_y_offset = framebuffer.height - maze.len() * mini_map_size - 10;

    render2d_mini_map(
        framebuffer,
        player,
        maze,
        mini_map_size,
        mini_map_x_offset,
        mini_map_y_offset,
    );
}

fn render2d_mini_map(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    x_offset: usize,
    y_offset: usize,
) {
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(
                framebuffer,
                x_offset + col * block_size,
                y_offset + row * block_size,
                block_size,
                maze[row][col],
            );
        }
    }

    // Draw the player on the mini-map
    framebuffer.draw_rectangle(
        x_offset + (player.pos.x * block_size as f32) as usize,
        y_offset + (player.pos.y * block_size as f32) as usize,
        block_size / 2,
        block_size / 2,
        Color::from_hex(0x5F88CC),
    );
}

fn render_lost_screen(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width;
    let height = framebuffer.height;

    for y in 0..height {
        for x in 0..width {
            let tx = x * LOST_SCREEN.width as usize / width;
            let ty = y * LOST_SCREEN.height as usize / height;
            let color = LOST_SCREEN.get_pixel(tx, ty);
            framebuffer.point_with_color(x, y, color);
        }
    }

    framebuffer.draw_text(
        "You lost! Press any key to return to start.",
        width / 2 - 150,
        height - 50,
        Color::from_hex(0xFFFFFF),
    );
}

fn play_background_music(sink: Arc<Mutex<Sink>>, stream_handle: rodio::OutputStreamHandle) {
    let file = File::open("./assets/horror.mp3").expect("Failed to open music file");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

    let amplified_source = source.amplify(0.5);

    let sink = sink.lock().unwrap();
    sink.append(amplified_source.repeat_infinite());
    sink.play();
}

fn play_sound_effect(
    file_path: &str,
    bg_music_sink: Arc<Mutex<Sink>>,
    stream_handle: rodio::OutputStreamHandle,
) {
    let file = File::open(file_path).expect("Failed to open sound effect file");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

    // Lower the background music volume
    {
        let bg_music_sink = bg_music_sink.lock().unwrap();
        bg_music_sink.set_volume(0.2);
    }

    let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");
    sink.append(source);
    sink.sleep_until_end(); // Esperar hasta que el sonido termine de reproducirse

    // Restore the background music volume
    let bg_music_sink = bg_music_sink.lock().unwrap();
    bg_music_sink.set_volume(0.5);
}

fn check_collision(player: &Player, sprite: &Sprite) -> bool {
    let distance_x = player.pos.x - sprite.x;
    let distance_y = player.pos.y - sprite.y;
    let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();
    distance < 0.5 // Puedes ajustar este valor según el tamaño del sprite y el jugador
}

fn reset_game(player: &mut Player, sprite: &mut Sprite, maze: &mut Vec<Vec<char>>) {
    *player = Player::new(1.5, 1.5, std::f32::consts::PI / 3.0, 0.02, 0.1);
    *sprite = Sprite::new("./assets/sprite.png", 1.5, 3.5, 1.0, 0.007);
    *maze = load_maze("./maze.txt");
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let bg_music_sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));

    // Start playing background music in a separate thread.
    let bg_music_sink_clone = Arc::clone(&bg_music_sink);
    let stream_handle_clone = stream_handle.clone();
    thread::spawn(move || {
        play_background_music(bg_music_sink_clone, stream_handle_clone);
    });

    let width = 1300; // Framebuffer width
    let height = 900; // Framebuffer height
    let mut framebuffer = Framebuffer::new(width, height);

    let mut window = Window::new(
        "The B∀CKROOMS",
        (width as f32 / 1.3) as usize,
        (height as f32 / 1.3) as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let block_size = 100; // Block size in pixels

    // Set up player
    let mut player = Player::new(1.5, 1.5, std::f32::consts::PI / 3.0, 0.02, 0.1);

    let mut sprite = Sprite::new("./assets/sprite.png", 1.5, 3.5, 1.0, 0.007);

    let mut maze = load_maze("./maze.txt");

    let mut state = GameState::StartScreen; // Initial state

    let mut last_mouse_pos = window
        .get_mouse_pos(MouseMode::Pass)
        .unwrap_or((0.0, 0.0))
        .0;

    let sensitivity = 0.003; // Mouse sensitivity

    let mut timer = Timer::new(); // Timer instance
    let mut gilrs = Gilrs::new().unwrap(); // Gilrs instance

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();

        match state {
            GameState::StartScreen => {
                render_start_screen(&mut framebuffer);
                // Start the game on any key press
                if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Space, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Left, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Right, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Up, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Down, minifb::KeyRepeat::No)
                {
                    reset_game(&mut player, &mut sprite, &mut maze);
                    state = GameState::Playing;
                }
            }
            GameState::Playing => {
                process_events(&window, &mut player, &maze, block_size, &mut gilrs);

                // Mueve el sprite hacia el jugador
                sprite.move_towards_player(&player, &maze, block_size);

                // Manejar la rotación del ratón
                if let Some((mouse_x, _)) = window.get_mouse_pos(MouseMode::Pass) {
                    let delta = mouse_x - last_mouse_pos;
                    player.angle += delta * sensitivity;
                    if player.angle < 0.0 {
                        player.angle += 2.0 * std::f32::consts::PI;
                    }
                    if player.angle >= 2.0 * std::f32::consts::PI {
                        player.angle -= 2.0 * std::f32::consts::PI;
                    }
                    update_direction(&mut player);
                    last_mouse_pos = mouse_x;
                }

                // Renderizar la vista 3D o 2D
                render3d(&mut framebuffer, &mut player, &maze, &sprite);

                // Verificar si el jugador ha ganado
                if has_won(player.pos.x, player.pos.y, &maze, block_size) {
                    state = GameState::Won;

                    let bg_music_sink_clone = Arc::clone(&bg_music_sink);
                    let stream_handle_clone = stream_handle.clone();
                    thread::spawn(move || {
                        play_sound_effect(
                            "./assets/change.mp3",
                            bg_music_sink_clone,
                            stream_handle_clone,
                        );
                    });
                }

                // Verificar si el sprite colisiona con el jugador (lógica de pérdida)
                if check_collision(&player, &sprite) {
                    state = GameState::Lost;

                    let bg_music_sink_clone = Arc::clone(&bg_music_sink);
                    let stream_handle_clone = stream_handle.clone();
                    thread::spawn(move || {
                        play_sound_effect(
                            "./assets/game-over.mp3",
                            bg_music_sink_clone,
                            stream_handle_clone,
                        );
                    });
                }
            }
            GameState::Lost => {
                render_lost_screen(&mut framebuffer);
                if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Space, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Left, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Right, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Up, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Down, minifb::KeyRepeat::No)
                {
                    // Aquí puedes bloquear la opción de reiniciar el juego hasta que termine la canción
                    // o simplemente no permitir reiniciar mientras `sink` sigue reproduciendo sonido.
                    reset_game(&mut player, &mut sprite, &mut maze);
                    state = GameState::StartScreen;
                }
            }
            GameState::Won => {
                framebuffer.draw_text(
                    "You won! Press any key to return to start.",
                    100,
                    100,
                    Color::from_hex(0x7F5A1B),
                );
                if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Space, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Left, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Right, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Up, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Down, minifb::KeyRepeat::No)
                {
                    // Aquí puedes bloquear la opción de reiniciar el juego hasta que termine la canción
                    // o simplemente no permitir reiniciar mientras `sink` sigue reproduciendo sonido.
                    reset_game(&mut player, &mut sprite, &mut maze);
                    state = GameState::StartScreen;
                }
            }
        }

        // Update timer and display FPS
        timer.update();
        let fps_text = format!("FPS: {:.2}", timer.get_fps());
        framebuffer.draw_text(&fps_text, 10, 10, Color::from_hex(0xFFFFFF));

        // Render framebuffer to window
        window
            .update_with_buffer(&framebuffer.to_u32_buffer(), width, height)
            .unwrap();
    }
}
