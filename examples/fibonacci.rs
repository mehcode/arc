extern crate square;

use square::*;

fn recursive_view(value: i16) -> View {

    println!("Current iteration -> {}", value);

    let direction = if value % 2 == 0 {
        if value % 4 == 0 {
            FlexDirection::Row
        } else {
            FlexDirection::RowReverse
        }
    } else {
        if (value - 1) % 4 == 0 {
            FlexDirection::ColumnReverse
        }else {
            FlexDirection::Column
        }
    };

    let mut root = View::new();
    root.set_flex_direction(direction);

    let mut view = View::new();

    view.set_flex_grow(8.);
    view.set_background_color(Color::from((value as u32 * 800 )/ 2));

    root.add_child(view);


    let mut bview = if value != 0 {
        recursive_view(value - 1)
    } else {
        let mut v = View::new();
        v.set_background_color(0x0d2a4e);
        return v;
    };

    bview.set_flex_grow(5.);

    root.add_child(bview);

    root
}


fn main() {
    let app = Application::new();

    let mut window = Window::new(1280., 800.);
    window.set_title("Fibonacci-ish");
    window.set_view(recursive_view(12));

    app.add_window(window);

    app.run();
}