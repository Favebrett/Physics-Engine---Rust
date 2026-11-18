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
    ball.dy -= 9.81 * 0.032;
}

fn drag(ball: &mut Ball) {
    let rho = 1.225;
    let cd = 0.23;
    let area = 0.01;

    let xdrag = -0.5 * rho * cd * area * ball.dx * ball.dx.abs();
    let ydrag = -0.5 * rho * cd * area * ball.dy * ball.dy.abs();

    ball.dx += xdrag;
    ball.dy += ydrag;
}

fn moveball(ball: &mut Ball) {
    ball.x += ball.dx * 0.032;
    ball.y += ball.dy * 0.032;
}

fn collision(ball: &mut Ball) {
    if ball.y - (ball.radius / 2.0) <= -30.0 {
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
    collision(ball);
    moveball(ball);
    gravity(ball);
    drag(ball);
    draw(&ball);
    telemetry(&ball);
}
