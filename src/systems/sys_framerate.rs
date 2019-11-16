use crate::game::Game;

pub fn sys_framerate(game: &mut Game, delta: f32) {
    let window = game.canvas.window_mut();
    // let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    // let diff = now - game.last_time;
    // let fps = 1000 / diff;

    // game.last_time = now;

    window.set_title(format!("fps: {}", (1.0 / delta) as u128).as_str()).unwrap();
}
