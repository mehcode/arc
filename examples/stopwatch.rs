extern crate arc;

use arc::{Context, FlexDirection, Font, Gravity, Node, NodeId, Text, View, WeakContext, Window};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

struct State {
    context: WeakContext,
    instant: Instant,
    is_running: bool,
    play_pause_button_label_id: Option<NodeId>,
    timer_label_id: Option<NodeId>,
}

fn update_play_pause_button_label(state: &State) {
    if let Some(cx) = state.context.upgrade() {
        if let Some(play_pause_button_label_id) = state.play_pause_button_label_id {
            let is_running = state.is_running;

            cx.update_by_id(
                play_pause_button_label_id,
                |play_pause_button_label: &mut Text| {
                    play_pause_button_label.set_text(if is_running { "Pause" } else { "\u{25b6}" });
                },
            );
        }
    }
}

fn update_timer_label(state: &State) {
    if let Some(cx) = state.context.upgrade() {
        if let Some(timer_label_id) = state.timer_label_id {
            let elapsed = state.instant.elapsed();
            let elapsed_ms = ((elapsed.as_secs() as u128) * 1_000_000_000
                + (elapsed.subsec_nanos() as u128))
                / 1_000_000;

            let hours = elapsed_ms / 3_600_000;
            let minutes = (elapsed_ms % 3_600_000) / 60_000;
            let seconds = (elapsed_ms % 60_000) / 1_000;
            let ms = elapsed_ms % 1_000;

            cx.update_by_id(timer_label_id, move |timer_label: &mut Text| {
                timer_label.set_text(format!(
                    "{:02}:{:02}:{:02}.{:03}",
                    hours, minutes, seconds, ms,
                ));
            });
        }
    }
}

fn main() {
    let cx = Context::new();

    // Create a shared `State` with a (downgraded) context remembered
    let state = Arc::new(Mutex::new(State {
        context: cx.downgrade(),
        instant: Instant::now(),
        is_running: false,
        timer_label_id: None,
        play_pause_button_label_id: None,
    }));

    let mut window = Window::new(&cx, 400., 200.);
    window.set_title("Stopwatch");
    window.set_resizable(false);
    window.set_background_color(0xff_e3e3e3);

    let mut play_pause_button_label = Text::new(&cx);
    play_pause_button_label.set_text("\u{25b6}");
    play_pause_button_label.set_text_size(32.);
    play_pause_button_label.set_text_color(0xff_e3e3e3);

    // FIXME: We need intrinsic sizing
    play_pause_button_label.set_flex_grow(1.);
    play_pause_button_label.set_gravity(Gravity::CENTER);

    // Remember the ID for the `play_pause_button_label`
    state.lock().unwrap().play_pause_button_label_id = Some(play_pause_button_label.id());

    let mut play_pause_button = View::new(&cx);
    play_pause_button.set_flex_grow(1.);
    play_pause_button.set_background_color(0xff_4a5853);
    play_pause_button.add(play_pause_button_label);

    play_pause_button.mouse_down().add({
        let state = state.clone();
        move |_| {
            let mut state = state.lock().unwrap();

            // Reset time if not running
            if !state.is_running {
                state.instant = Instant::now();
            }

            // Toggle is-running state
            state.is_running = !state.is_running;

            // Update the play/pause button label
            update_play_pause_button_label(&*state);

            // Update the label showing the current elapsed time
            update_timer_label(&*state);
        }
    });

    let mut reset_button_label = Text::new(&cx);
    reset_button_label.set_text("\u{21bb}");
    reset_button_label.set_text_size(32.);
    reset_button_label.set_text_color(0xff_e3e3e3);

    // FIXME: We need intrinsic sizing
    reset_button_label.set_flex_grow(1.);
    reset_button_label.set_gravity(Gravity::CENTER);

    let mut reset_button = View::new(&cx);
    reset_button.set_flex_grow(1.);
    reset_button.set_background_color(0xff_94cfe0);
    reset_button.add(reset_button_label);

    reset_button.mouse_down().add({
        let state = state.clone();
        move |_| {
            let mut state = state.lock().unwrap();

            // Reset the start time
            state.instant = Instant::now();

            // Update the label showing the current elapsed time
            update_timer_label(&mut *state);
        }
    });

    let mut buttons = View::new(&cx);
    buttons.set_height(75.);
    buttons.set_flex_direction(FlexDirection::Row);
    buttons.add(play_pause_button);
    buttons.add(reset_button);

    let mut timer_label = Text::new(&cx);
    timer_label.set_text("00:00:00.000");
    timer_label.set_text_size(52.);
    timer_label.set_font(&Font::builder(&cx).name("Iosevka").build());

    // FIXME: The following commands shouldn't be needed with intrinsic sizes
    timer_label.set_gravity(Gravity::CENTER);
    timer_label.set_flex_grow(1.);

    // Remember the ID for the `timer_label`
    state.lock().unwrap().timer_label_id = Some(timer_label.id());

    let mut watch = View::new(&cx);
    watch.set_flex_grow(1.);
    watch.add(timer_label);

    let mut root = View::new(&cx);
    root.add(watch);
    root.add(buttons);

    window.set_view(root);
    window.show();

    // Spawn a thread to periodically update the timer label
    thread::spawn(move || loop {
        {
            let state = state.lock().unwrap();
            if state.is_running {
                update_timer_label(&*state);
            }
        }

        thread::sleep(Duration::from_millis(100));
    });

    cx.run();
}
