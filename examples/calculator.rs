extern crate square;

use square::*;

const PADDING: f32 = 5.;

fn main() {
    let c = Context::new();

    let mut window = Window::new(&c, 85. * 4. + 10., 121. + 85. * 5. + 10.);
    window.set_title("Calculator");
    window.set_resizable(false);

    let mut row0 = View::new(&c);
    row0.set_flex_basis(120.);
    row0.set_background_color(0xffffffff);
    row0.set_margin(Edge::Bottom, 1.);

    let mut row1 = View::new(&c);
    row1.set_margin(Edge::Vertical, PADDING);
    row1.set_flex_direction(FlexDirection::Row);
    row1.set_flex_basis(75.);
    row1.add(op_button(&c, Operator::Clear));
    row1.add(op_button(&c, Operator::InvertSign));
    row1.add(op_button(&c, Operator::Percent));
    row1.add(op_button(&c, Operator::Divide));

    let mut row2 = View::new(&c);
    row2.set_margin(Edge::Vertical, PADDING);
    row2.set_flex_direction(FlexDirection::Row);
    row2.set_flex_basis(75.);
    row2.add(digit_button(&c, 7));
    row2.add(digit_button(&c, 8));
    row2.add(digit_button(&c, 9));
    row2.add(op_button(&c, Operator::Multiply));

    let mut row3 = View::new(&c);
    row3.set_margin(Edge::Vertical, PADDING);
    row3.set_flex_direction(FlexDirection::Row);
    row3.set_flex_basis(75.);
    row3.add(digit_button(&c, 4));
    row3.add(digit_button(&c, 5));
    row3.add(digit_button(&c, 6));
    row3.add(op_button(&c, Operator::Subtract));

    let mut row4 = View::new(&c);
    row4.set_margin(Edge::Vertical, PADDING);
    row4.set_flex_direction(FlexDirection::Row);
    row4.set_flex_basis(75.);
    row4.add(digit_button(&c, 1));
    row4.add(digit_button(&c, 2));
    row4.add(digit_button(&c, 3));
    row4.add(op_button(&c, Operator::Add));

    let mut row5 = View::new(&c);
    row5.set_margin(Edge::Vertical, PADDING);
    row5.set_flex_direction(FlexDirection::Row);
    row5.set_flex_basis(75.);
    row5.add(digit_button(&c, 0));
    row5.add(op_button(&c, Operator::Decimal));
    row5.add(op_button_2x(&c, Operator::Solve));

    let mut panel = View::new(&c);
    panel.set_padding(Edge::All, PADDING);
    panel.add(row1);
    panel.add(row2);
    panel.add(row3);
    panel.add(row4);
    panel.add(row5);

    let mut main = View::new(&c);
    main.set_background_color(0xffffffff);
    main.add(row0);
    main.add(panel);

    window.set_view(main);

    c.add_window(window);
    c.run();
}

enum Operator {
    Clear,
    InvertSign,
    Percent,
    Divide,
    Multiply,
    Subtract,
    Add,
    Decimal,
    Solve,
}

fn button(c: &Context) -> View {
    let mut button = View::new(c);
    button.set_flex_basis(75.);
    button.set_margin(Edge::Horizontal, 5.);
    button.set_corner_radius(6.);

    button
}

fn op_button(c: &Context, op: Operator) -> View {
    let mut button = button(c);

    let background_color = match op {
        Operator::Add | Operator::Subtract | Operator::Multiply | Operator::Divide => 0xffbddff9,
        Operator::Solve => 0xff0070e5,
        Operator::Clear => 0xffffd5d4,
        _ => 0xfff2f2f6,
    };

    button.set_background_color(background_color);
    button
}

fn digit_button(c: &Context, digit: u8) -> View {
    let mut button = button(c);
    button.set_background_color(0xfff2f2f6);
    button
}

fn op_button_2x(c: &Context, op: Operator) -> View {
    let mut button = op_button(c, op);
    // TODO: button.set_flex_basis(button.flex_basis() * 2.);
    button.set_flex_basis(160.);

    button
}
