use scap::{
    capturer::{Point, Area, Size, Capturer, Options},
    frame::Frame,
};

use crate::frb_generated::{StreamSink, FLUTTER_RUST_BRIDGE_HANDLER};
use flutter_rust_bridge::frb;

#[flutter_rust_bridge::frb]
pub fn start_screen_capture(sink: StreamSink<String>) {
    if !scap::is_supported() {
        sink.add("❌ Platform not supported".to_string());
        return;
    }

    if !scap::has_permission() {
        sink.add("❌ Permission not granted. Requesting permission...".to_string());
        if !scap::request_permission() {
            sink.add("❌ Permission denied".to_string());
            return;
        }
    }

    let options = Options {
        fps: 60,
        target: None,
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_720p,
        // source_rect: Some(Area {
        //     origin: Point { x: 0.0, y: 0.0 },
        //     size: Size { width: 2000.0, height: 1000.0 },
        // }),
        ..Default::default()
    };

    let mut capturer = Capturer::new(options);
    capturer.start_capture();

    sink.add("✅ Screen capture started".to_string());

    // Example of a blocking call to keep the capture running
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    capturer.stop_capture();
    sink.add("🛑 Screen capture stopped".to_string());
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[frb]
pub fn initialize_screen_capture() -> bool {
    // Your screen capture implementation
    true
}
