use atari_env::{AtariEnv, EmulatorConfig};
use pixels::SurfaceTexture;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event_loop::EventLoop;
use winit_input_helper::WinitInputHelper;

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
    let env = AtariEnv::new(
        "/home/emppu/.local/lib/python3.9/site-packages/atari_py/atari_roms/zaxxon.bin".to_string(),
        EmulatorConfig {
            display_screen: true,
            sound: true,
            ..EmulatorConfig::default()
        },
    );

    let event_loop = EventLoop::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("asdf", env.width() as f64, env.height() as f64, &event_loop);
    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);
    let mut pixels = pixels::Pixels::new(env.width(), env.height(), surface_texture).unwrap();

    loop {
        env.step(0);
        env.step(0);
        env.step(0);
        env.step(0);
        env.render_rgb32(pixels.get_frame());
        pixels.render().unwrap();
        window.request_redraw();
    }
}
