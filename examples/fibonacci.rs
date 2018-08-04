extern crate square;

use square::*;

fn recursive_view(value: i16) -> View {

    let (direction, color) = if value % 2 == 0 {
        if value % 4 == 0 {
            (FlexDirection::Row, 0xa9d2f0)
        } else {
            (FlexDirection::RowReverse, 0xa8ecc2)
        }
    } else {
        if (value - 1) % 4 == 0 {
            (FlexDirection::ColumnReverse, 0xf3c99f)
        }else {
            (FlexDirection::Column, 0xf5b3ac)
        }
    };

    let mut root = View::new();
    root.set_flex_direction(direction);

    let mut view = View::new();

    view.set_flex_grow(8.);
    view.set_background_color(color);

    view.set_margin(Edge::All, 1.5);

    root.add_child(view);

    let mut bview = if value != 0 {
        recursive_view(value - 1)
    } else {
        let mut v = View::new();
        v.set_background_color(0xf8e896);
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
    window.set_background_color(0xa9b2bb);

    app.add_window(window);

    app.run();
}