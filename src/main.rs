#[macro_use]
mod macros;
mod about_dialog;
mod app;
mod audio_vumeter;
mod header_bar;
mod pipeline;
mod settings;
mod utils;

use gio::prelude::*;

use std::env::args;
use std::error;

use crate::app::App;

// Unique application name to identify it
//
// This is used for ensuring that there's only ever a single instance of our application
pub const APPLICATION_NAME: &str = "com.igalia.gstwpe.broadcast.demo";

fn main() -> Result<(), Box<dyn error::Error>> {
    // Initialize GStreamer. This checks, among other things, what plugins are available
    gst::init()?;

    // Create an application with our name and the default flags. By default, applications can only
    // have a single instance and any second instance will only activate the first one again
    let application =
        gtk::Application::new(Some(APPLICATION_NAME), gio::ApplicationFlags::empty())?;

    // On application startup (of the first instance) we create our application. A second instance
    // would not run this
    application.connect_startup(|application| {
        App::on_startup(application);
    });

    // And now run the application until the end
    application.run(&args().collect::<Vec<_>>());

    Ok(())
}
