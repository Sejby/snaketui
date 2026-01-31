use crate::game::{GameState, constants::*};
use ratatui::Frame;
use ratatui::style::Color;
use ratatui::widgets::Block;
use ratatui::widgets::canvas::{Canvas, Rectangle};

pub fn draw_game(frame: &mut Frame, game_state: &GameState) {
    let canvas = Canvas::default()
        .block(Block::bordered().title("SnakeTUI"))
        .background_color(Color::Black)
        .x_bounds([WORLD_MIN, WORLD_MAX])
        .y_bounds([WORLD_MIN, WORLD_MAX])
        .paint(|ctx| {
            ctx.print(0.0, 4.8, format!("Score: {}", game_state.score()));
            ctx.print(0.0, 4.6, "Press Q to quit the game");
            ctx.print(0.0, 4.4, "Pro tip: Don't let the snake eat its own tail");

            if game_state.is_game_over() {
                ctx.print(2.5, 4.0, "Game Over, Press R to restart");
            }

            let apple = game_state.apple();
            ctx.draw(&Rectangle {
                x: apple.x,
                y: apple.y,
                width: SNAKE_BOX_SIZE,
                height: SNAKE_BOX_SIZE,
                color: Color::Red,
            });

            for seg in game_state.snake().segments() {
                ctx.draw(&Rectangle {
                    x: seg.x,
                    y: seg.y,
                    width: SNAKE_BOX_SIZE,
                    height: SNAKE_BOX_SIZE,
                    color: Color::Green,
                });
            }
        });

    frame.render_widget(canvas, frame.area());
}
