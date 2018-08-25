use arc::{self, Font, Gravity, Text, Window};
use chrono::Local;
use std::{
    thread::{sleep, spawn},
    time::Duration,
};

fn main() {
    // Create a new window of the given width and height
    // NOTE: The plan is allow intrinsic sizing for windows (as big as their elements).
    let mut window = Window::new(350., 200.);

    // Create a new text element
    let mut label = Text::new();

    // Use a monospace font for our label
    // NOTE: We are considering replacing this method with direct `set_font_family` type methods
    label.set_font(&Font::builder().name("monospace").build());

    // Center the text inside the text element
    // [Default] label.set_gravity(Gravity::START | Gravity::TOP);
    label.set_gravity(Gravity::CENTER);

    // Add the label to the window (a container of exactly one element)
    // Elements are shown when given an owner (in this case the window)
    window.set_root(&label);

    // Spawn a thread to update the label with the current time every 20ms
    spawn(move || {
        loop {
            // Accessing an element off the main thread will
            // transparently dispatch to the main thread and complete the request
            label.set_text(format!("Hello, World!\n\n{}", Local::now()));

            // Wait 20ms to update the time
            sleep(Duration::from_millis(20));
        }
    });

    // Show the window
    window.show();

    // Run the event loop; returns when the last window closes
    arc::run();
}
