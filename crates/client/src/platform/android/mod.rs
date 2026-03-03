use std::sync::Once;

use winit::platform::android::{
    EventLoopBuilderExtAndroid,
    activity::{AndroidApp, WindowManagerFlags},
};

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    init_logging();

    // Trick egui to persist storage in our dedicated Android storage location.
    if let Some(path) = app.internal_data_path() {
        log::info!("internal data path: {:?}", path);
        unsafe {
            std::env::set_var("HOME", path.to_str().unwrap());
            std::env::set_var("XDG_DATA_HOME", path.to_str().unwrap());
        }
    }

    // Ask Android to keep the screen on while this app is visible. This is very
    // helpful for ensuring that our timer thread won't be hindered and that the
    // timer state is clearly visible while in use.
    //
    // Also confirm that our app layout accounts for painting under the status and
    // navigation bars.
    app.set_window_flags(
        WindowManagerFlags::KEEP_SCREEN_ON
            | WindowManagerFlags::LAYOUT_IN_SCREEN
            | WindowManagerFlags::LAYOUT_INSET_DECOR,
        WindowManagerFlags::empty(),
    );

    let event_loop = winit::event_loop::EventLoop::with_user_event()
        .with_android_app(app)
        .build()
        .unwrap();

    let mut app = crate::App::default();
    event_loop.run_app(&mut app).unwrap();
}

fn init_logging() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        android_logger::init_once(
            android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
        );

        log_panics::init();
    });
}
