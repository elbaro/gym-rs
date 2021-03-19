use atari_env::{AtariEnv, ObservationType, RenderMode};

fn main() {
    let env = AtariEnv::new(
        "pong".to_string(),
        RenderMode::Human,
        ObservationType::Image,
        (0, 0),
    );

    for i in 0..50 {
        env.step(1);
    }
    env.render();
}
