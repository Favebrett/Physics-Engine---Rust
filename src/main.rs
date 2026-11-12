struct Ball {
    dx: f64,
    dy: f64,
    x: f64,
    y: f64,
}

fn main() {
    let mut ball = pos(-5.0, 0.0, 0.0, 0.0);

    for _ in 1..10 {
        game_loop(&mut ball);
    }
}

fn moveball(ball: &mut Ball) {
    ball.x += ball.dx;
    ball.y += ball.dy;
}

fn pos(_dx: f64, _dy: f64, _x: f64, _y: f64) -> Ball {
    Ball {
        dx: _dx,
        dy: _dy,
        x: _x,
        y: _y,
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
    telemetry(&ball);
}
