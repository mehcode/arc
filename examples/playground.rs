extern crate arc;

use arc::*;

fn main() {
    let c = Context::new();

    let mut window = Window::new(&c, 600., 600.);
    window.set_title("Playground");

    let mut cell = Text::new(&c);
    cell.set_flex_grow(1.);
    cell.set_font_family("Input Mono");
    cell.set_text("Firefox Nightly");
    cell.set_text_size(16.);
    cell.set_background_color(0xff_d2d2d2);
    // cell.set_font_weight(600);

    let mut inner = View::new(&c);
    inner.set_flex_grow(1.);
    inner.set_background_color(0xff_e2e2e2);
    inner.set_padding(Edge::All, 6.);
    inner.set_corner_radius(6.);
    inner.add(cell);

    let mut root = View::new(&c);
    root.set_background_color(0xff_f5f5f5);
    root.set_padding(Edge::All, 20.);
    root.set_flex_direction(FlexDirection::Row);
    root.add(inner);

    window.set_view(root);

    c.add_window(window);
    c.run();
}
