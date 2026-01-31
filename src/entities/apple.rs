use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Apple {
    pub x: f64,
    pub y: f64,
}

impl Apple {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn spawn_random<F>(box_size: f64, overlaps_checker: F) -> Self
    where
        F: Fn(f64, f64) -> bool,
    {
        let mut rng = rand::rng();

        loop {
            let apple = Apple {
                x: (rng.random_range(0..25) as f64) * box_size,
                y: (rng.random_range(0..25) as f64) * box_size,
            };

            if !overlaps_checker(apple.x, apple.y) {
                return apple;
            }
        }
    }
}
