use eframe::NativeOptions;
use egui::ViewportBuilder;
use tracing_subscriber;

use wealth_tracker::app::WealthTracker;

fn main() -> eframe::Result {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .init();

    let viewport = ViewportBuilder::default()
        // .with_decorations(false)
        .with_inner_size((800.0, 800.0))
        // .with_transparent(true)
        .with_drag_and_drop(true);

    let native_options = NativeOptions {
        viewport: viewport,
        ..Default::default()
    };

    eframe::run_native(
        WealthTracker::name(),
        native_options,
        Box::new(|cc| Ok(Box::new(WealthTracker::new(cc)))),
    )
}
