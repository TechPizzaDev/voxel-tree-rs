#[cfg(not(target_os = "android"))]
fn main() {
    use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::CLOSE))
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // wgpu uses `log` for all of our logging, so we initialize a logger with the `env_logger` crate.
    //
    // To change the log level, set the `RUST_LOG` environment variable. See the `env_logger`
    // documentation for more information.

    //env_logger::init();

    let event_loop = winit::event_loop::EventLoop::with_user_event()
        .build()
        .unwrap();

    // When the current loop iteration finishes, immediately begin a new
    // iteration regardless of whether or not new events are available to
    // process. Preferred for applications that want to render as fast as
    // possible, like games.
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    // When the current loop iteration finishes, suspend the thread until
    // another event arrives. Helps keeping CPU utilization low if nothing
    // is happening, which is preferred if the application might be idling in
    // the background.
    // event_loop.set_control_flow(ControlFlow::Wait);

    let asset_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .or_else(|_| std::env::current_dir())
        .unwrap()
        .join("assets");

    let mut app = client::App::new(asset_dir);
    event_loop.run_app(&mut app).unwrap();
}

#[cfg(target_os = "android")]
fn main() {}
