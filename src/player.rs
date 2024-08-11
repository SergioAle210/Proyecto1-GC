pub struct Player {
    pub pos: Vec2,
    pub dir: Vec2,
    pub angle: f32,
    pub speed: f32,
    pub rotation_speed: f32,
    pub fov: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, fov: f32, speed: f32, rotation_speed: f32) -> Self {
        let angle: f32 = 0.0;
        Self {
            pos: Vec2 { x, y },
            dir: Vec2 {
                x: angle.cos(),
                y: angle.sin(),
            },
            angle,
            speed,
            rotation_speed: rotation_speed * 0.3,
            fov,
        }
    }
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
