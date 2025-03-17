use crate::COLLISION_DISTANCE;

#[derive(Debug, Copy, Clone)]
pub struct Particle  {
    pub x: f32,
    pub y: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Particle{
        Particle{
            x,
            y
        }
    }

    pub fn update(&mut self, d_x: f32, d_y: f32){
        self.move_x(d_x);
        self.move_y(d_y);
    }

    pub fn move_x(&mut self, d_x: f32){
        self.x += d_x;

        if self.x > 10.0{
            self.x = 0.0;
        }
        else if self.x < 0.0{
            self.x = 10.0;
        }
    }

    pub fn move_y(&mut self, d_y: f32){
        self.y += d_y;

        if self.y > 10.0{
            self.y = 0.0;
        }
        else if self.y < 0.0{
            self.y = 10.0;
        }
    }

    pub fn collide(&self, o_x: f32, o_y:f32) -> bool{
        if (self.x - o_x).powi(2) + (self.y - o_y).powi(2) < COLLISION_DISTANCE.powi(2){
            return true;
        }
        return false;
    }
}
