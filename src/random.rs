use rand::random;

use nannou::prelude::{random_range, vec2, Vec2};

pub fn random_vec2(max_bounds: &Vec2) -> Vec2 {
    let bounds = vec2(max_bounds.x * 0.5, max_bounds.y * 0.5);

    let x = random_range(-bounds.x, bounds.x);
    let y = random_range(-bounds.y, bounds.y);

    vec2(x, y)
}

pub fn get_random<T>(choices: &[T]) -> T
where
    T: Clone,
{
    let r = random::<usize>();
    let winner = r % choices.len();

    choices[winner].clone()
}

pub fn chance(probability: f32) -> bool {
    if probability < 0.0 || probability > 100.0 {
        eprintln!("This is impossible ({}% !!!", probability);
        false
    } else {
        let probability = probability / 100.0;
        if random::<f32>() < probability {
            true
        } else {
            false
        }
    }
}
