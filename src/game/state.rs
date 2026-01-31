use crate::entities::{Apple, Segment, Snake, SnakeDirection};
use crate::game::constants::*;
use std::time::Instant;

#[derive(Debug)]
pub struct GameState {
    snake: Snake,
    apple: Apple,
    game_over: bool,
    grow: bool,
    last_tick: Instant,
}

impl GameState {
    pub fn new() -> Self {
        let initial_segments = vec![
            Segment::new(2.0, 2.0),
            Segment::new(1.8, 2.0),
            Segment::new(1.6, 2.0),
        ];

        Self {
            snake: Snake::new(initial_segments, SnakeDirection::Right),
            apple: Apple::new(3.0, 3.0),
            game_over: false,
            grow: false,
            last_tick: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        let initial_segments = vec![
            Segment::new(2.0, 2.0),
            Segment::new(1.8, 2.0),
            Segment::new(1.6, 2.0),
        ];
        self.snake = Snake::new(initial_segments, SnakeDirection::Right);
        self.apple = Apple::new(3.0, 3.0);
        self.game_over = false;
        self.grow = false;
        self.last_tick = Instant::now();
    }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    pub fn apple(&self) -> &Apple {
        &self.apple
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn score(&self) -> usize {
        self.snake.len().saturating_sub(INITIAL_SNAKE_LENGTH)
    }

    pub fn set_direction(&mut self, direction: SnakeDirection) {
        self.snake.set_direction(direction);
    }

    pub fn should_tick(&self) -> bool {
        self.last_tick.elapsed() >= TICK_RATE
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        let head = self.snake.head();

        let apple_hit = (head.x - self.apple.x).abs() < SNAKE_BOX_SIZE
            && (head.y - self.apple.y).abs() < SNAKE_BOX_SIZE;

        if apple_hit {
            self.grow = true;
            self.spawn_new_apple();
        }

        let new_head = self.snake.calculate_next_head(SNAKE_STEP_SIZE);

        if self.wall_hit(&new_head) || self.snake.collides_with_self(&new_head) {
            self.game_over = true;
        } else {
            self.snake.move_forward(new_head, self.grow);
            self.grow = false;
        }

        self.last_tick = Instant::now();
    }

    fn wall_hit(&self, segment: &Segment) -> bool {
        segment.x < WORLD_MIN
            || segment.x > WORLD_MAX
            || segment.y < WORLD_MIN
            || segment.y > WORLD_MAX
    }

    fn spawn_new_apple(&mut self) {
        self.apple = Apple::spawn_random(SNAKE_BOX_SIZE, |x, y| {
            self.snake.overlaps(x, y, SNAKE_BOX_SIZE)
        });
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
