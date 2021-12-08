use nannou::prelude::*;

pub trait SetMag {
    fn get_vec2(&mut self) -> &mut Vec2;
    fn set_mag(&mut self, mag: f32) {
        let v = self.get_vec2();
        *v = v.normalize();
        v.x *= mag;
        v.y *= mag;
    }
}

#[derive(Clone)]
pub struct Mass(f32);

impl From<f32> for Mass {
    fn from(value: f32) -> Self {
        Mass(value)
    }
}

impl Mass {
    pub fn as_f32(&self) -> f32 {
        let Mass(value) = self;
        *value
    }
}

#[readonly::make]
#[derive(Clone)]
pub struct Velocity {
    pub value: Vec2,
    pub limit: f32,
}

impl From<Vec2> for Velocity {
    fn from(value: Vec2) -> Self {
        Velocity::new(value)
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::new(vec2(0.0, 0.0))
    }
}

impl Velocity {
    pub fn new(value: Vec2) -> Self {
        let limit = f32::MAX;
        Self { value, limit }
    }

    pub fn set_limit(&mut self, limit: f32) {
        self.limit = limit;
        self.clamp(limit);
    }

    fn clamp(&mut self, limit: f32) {
        if self.value.length() > limit {
            self.set_mag(limit)
        }
    }

    pub fn accelerate(&mut self, acc: &Acceleration) {
        let Acceleration(value) = acc;
        self.value += *value;
        self.clamp(self.limit);
    }
}

impl SetMag for Velocity {
    fn get_vec2(&mut self) -> &mut Vec2 {
        &mut self.value
    }
}

#[derive(Clone)]
pub struct Acceleration(Vec2);

impl Acceleration {
    pub fn add_force(&mut self, force: Force) -> &mut Self {
        let Acceleration(value) = self;
        *value += Vec2::from(force);
        self
    }

    pub fn reset(&mut self) {
        let Acceleration(value) = self;
        *value = vec2(0.0, 0.0);
    }
}

impl From<Force> for Acceleration {
    fn from(force: Force) -> Self {
        let Force(value) = force;
        Acceleration(value)
    }
}

impl From<Vec2> for Acceleration {
    fn from(value: Vec2) -> Self {
        Acceleration(value)
    }
}

impl Default for Acceleration {
    fn default() -> Self {
        Self(vec2(0.0, 0.0))
    }
}

#[derive(Clone)]
pub struct Force(Vec2);

impl From<f32> for Force {
    fn from(angle: f32) -> Self {
        let x = angle.cos();
        let y = angle.sin();
        Self(Vec2::new(x, y))
    }
}

impl From<Vec2> for Force {
    fn from(v: Vec2) -> Force {
        Force(v)
    }
}

impl From<Force> for Vec2 {
    fn from(force: Force) -> Self {
        let Force(value) = force;
        value
    }
}

impl SetMag for Force {
    fn get_vec2(&mut self) -> &mut Vec2 {
        let Force(force) = self;
        force
    }
}

impl Force {
    pub fn new(mag: f32, angle: f32) -> Self {
        let x = angle.cos() * mag;
        let y = angle.sin() * mag;

        Self(Vec2::new(x, y))
    }
}

#[readonly::make]
#[derive(Clone)]
pub struct MoverData {
    pub mass: Mass,
    pub velocity: Velocity,
    pub location: Vec2,
    pub acceleration: Acceleration,
}

impl MoverData {
    pub fn new(mass: Mass, velocity: Velocity, location: Vec2, acceleration: Acceleration) -> Self {
        Self {
            mass,
            velocity,
            location,
            acceleration,
        }
    }

    pub fn set_location(&mut self, location: Vec2) {
        self.location = location;
    }

    pub fn distance(&self, other: &Self) -> f32 {
        self.location.distance(other.location)
    }

    pub fn distance_squared(&self, other: &Self) -> f32 {
        self.location.distance_squared(other.location)
    }
}

pub trait Mover {
    fn get_mover_data(&mut self) -> &mut MoverData;

    fn update(&mut self) {
        let mover_data = self.get_mover_data();
        mover_data.velocity.accelerate(&mover_data.acceleration);
        mover_data.location += mover_data.velocity.value;
        mover_data.acceleration.reset();
    }

    fn add_force(&mut self, force: Force) {
        let mover_data = self.get_mover_data();
        mover_data.acceleration.add_force(force);
    }

    fn set_max_speed(&mut self, speed: f32) {
        let mover_data = self.get_mover_data();
        mover_data.velocity.set_limit(speed);
    }
}
