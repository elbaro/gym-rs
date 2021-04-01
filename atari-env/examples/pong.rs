use atari_env::{AtariAction, AtariEnv, EmulatorConfig};
use pixels::SurfaceTexture;
use rand::seq::SliceRandom;
use rand::Rng;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event_loop::EventLoop;

fn create_window(
    title: &str,
    width: f64,
    height: f64,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(&event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let size = monitor.size().to_logical(hidpi_factor);
            (size.width, size.height)
        } else {
            (width, height)
        }
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round().max(1.0);
    // let scale = 1.0;

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}

fn main() {
    let mut env = AtariEnv::new(
        dirs::home_dir()
            .unwrap()
            .join(".local/lib/python3.9/site-packages/atari_py/atari_roms/space_invaders.bin"),
        EmulatorConfig {
            // display_screen: true,
            // sound: true,
            frame_skip: 1,
            color_averaging: false,
            ..EmulatorConfig::default()
        },
    );

    let event_loop = EventLoop::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("asdf", env.width() as f64, env.height() as f64, &event_loop);
    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);
    let mut pixels =
        pixels::Pixels::new(env.width() as u32, env.height() as u32, surface_texture).unwrap();

    let actions = env.minimal_actions();
    println!("action set: {:?}", actions);
    println!(
        "difficulty settings: {:?}",
        env.available_difficulty_settings()
    );

    loop {
        while !env.is_game_over() {
            let action = if rand::thread_rng().gen::<bool>() {
                *actions.choose(&mut rand::thread_rng()).unwrap()
            } else {
                AtariAction::Noop
            };
            env.step(action);
            env.render_rgb32(pixels.get_frame());
            pixels.render().unwrap();
            window.request_redraw();
        }
        env.reset();
    }
}
