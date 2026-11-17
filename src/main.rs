use macroquad::prelude::*;
use std::{thread, time::Duration};

struct Ball {
    dx: f32,
    dy: f32,
    x: f32,
    y: f32,
    radius: f32, // i would be more optimized to use i8 but for further calculation is use f32
}

fn main() {
    let mut ball = create(0.0, 0.0, 0.0, 0.0, 10.0);

    loop {
        game_loop(&mut ball);

        thread::sleep(Duration::from_millis(32));
    }
}

async fn draw(ball: &Ball) {
    clear_background(BLACK);

    draw_circle(ball.x, ball.y, ball.radius, YELLOW);

    next_frame().await
}

fn gravity(ball: &mut Ball) {
    ball.dy -= 9.81 * 0.016;
}

fn moveball(ball: &mut Ball) {
    ball.x += ball.dx * 0.016;
    ball.y += ball.dy * 0.016;
}

fn collision(ball: &mut Ball) {
    if ball.y - (ball.radius / 2.0) <= -300.0 {
        ball.dy -= ball.dy * 2.0 // inverse the y speed
    }
}

fn create(_dx: f32, _dy: f32, _x: f32, _y: f32, _radius: f32) -> Ball {
    Ball {
        dx: _dx,
        dy: _dy,
        x: _x,
        y: _y,
        radius: _radius,
    }
}

fn telemetry(ball: &Ball) {
    println!(
        "dx: {}, dy: {}, x: {}, y: {}",
        ball.dx, ball.dy, ball.x, ball.y
    );
}

fn game_loop(ball: &mut Ball) {
    moveball(ball);
    gravity(ball);
    collision(ball);
    draw(&ball);
    telemetry(&ball);
}
