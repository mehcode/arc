extern crate arc;

use arc::*;

fn main() {
    let c = Context::new();

    let mut window = Window::new(&c, 375., 640.);
    window.set_title("Playground");

    let mut cell = Text::new(&c);
    cell.set_flex_grow(1.);
    cell.set_font_family("Futura");
    cell.set_text("Bacon ipsum dolor amet ground round prosciutto picanha turkey, shank pancetta tail burgdoggen kielbasa chuck kevin boudin. Doner pork loin strip steak fatback burgdoggen, cupim turducken beef ribs chicken bresaola andouille shoulder alcatra hamburger filet mignon.\n\nChicken meatloaf beef salami meatball, doner brisket chuck prosciutto landjaeger filet mignon tri-tip capicola. Burgdoggen ham picanha, andouille strip steak meatball spare ribs pork doner pork belly pork loin. ");
    cell.set_text_size(20.);
    cell.set_text_color(0xff_424242);
    cell.set_padding(Edge::All, 20.);

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
