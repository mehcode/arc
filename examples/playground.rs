use arc::*;

fn main() {
    let c = Context::new();

    // TODO: Look for a way to make this `const` or `static` (?)
    let iosevka_300 = Font::builder(&c).name("Iosevka").weight(300).build();

    let mut window = Window::new(&c, 800., 800.);
    window.set_title("Playground");

    let mut cell = Text::new(&c);
    cell.set_background_color(0xff_fafafa);
    cell.set_flex_grow(1.);
    cell.set_font(&iosevka_300);
    cell.set_text("Bacon ipsum dolor amet ground round prosciutto picanha turkey, shank pancetta tail burgdoggen kielbasa chuck kevin boudin. Doner pork loin strip steak fatback burgdoggen, cupim turducken beef ribs chicken bresaola andouille shoulder alcatra hamburger filet mignon.");
    cell.set_text_size(20.);
    cell.set_text_color(0xff_424242);
    cell.set_padding(Edge::All, 20.);
    cell.set_corner_radius(6.);
    cell.set_gravity(Gravity::CENTER);

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
    window.show();

    c.run();
}
