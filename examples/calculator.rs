extern crate arc;

use arc::*;
use std::sync::{Arc, Mutex};

const PADDING: f32 = 5.;

struct State {
    // Node ID of the operand label
    operand_label: Text,

    // Operand being shown and edited
    operand: Option<f64>,

    // Last operand
    last_operand: Option<f64>,

    // Pending operation to be performed
    operator: Option<Operator>,
}

fn main() {
    let mut c = Context::new();

    let mut window = Window::new(85. * 4. + 10., 121. + 85. * 5. + 10.);
    window.set_title("Calculator");
    window.set_resizable(false);

    let mut label_val = Text::new();
    label_val.set_text(&(0.).to_string());
    label_val.set_text_color(0xff_252525);
    label_val.set_font(&Font::builder().weight(500).build());
    label_val.set_text_size(46.);
    label_val.set_flex_grow(1.);
    label_val.set_gravity(Gravity::END | Gravity::BOTTOM);

    let state = Arc::new(Mutex::new(State {
        operand_label: label_val,
        operand: None,
        last_operand: None,
        operator: None,
    }));

    let mut row0 = View::new();
    row0.set_flex_basis(120.);
    row0.set_background_color(0xffffffff);
    row0.set_padding(Edge::Horizontal, 12.);
    row0.add(label_val);

    let mut row1 = View::new();
    row1.set_margin(Edge::Vertical, PADDING);
    row1.set_flex_direction(FlexDirection::Row);
    row1.set_flex_basis(75.);
    row1.add(op_button(&state, Operator::Clear));
    row1.add(op_button(&state, Operator::InvertSign));
    row1.add(op_button(&state, Operator::Percent));
    row1.add(op_button(&state, Operator::Divide));

    let mut row2 = View::new();
    row2.set_margin(Edge::Vertical, PADDING);
    row2.set_flex_direction(FlexDirection::Row);
    row2.set_flex_basis(75.);
    row2.add(digit_button(&state, 7));
    row2.add(digit_button(&state, 8));
    row2.add(digit_button(&state, 9));
    row2.add(op_button(&state, Operator::Multiply));

    let mut row3 = View::new();
    row3.set_margin(Edge::Vertical, PADDING);
    row3.set_flex_direction(FlexDirection::Row);
    row3.set_flex_basis(75.);
    row3.add(digit_button(&state, 4));
    row3.add(digit_button(&state, 5));
    row3.add(digit_button(&state, 6));
    row3.add(op_button(&state, Operator::Subtract));

    let mut row4 = View::new();
    row4.set_margin(Edge::Vertical, PADDING);
    row4.set_flex_direction(FlexDirection::Row);
    row4.set_flex_basis(75.);
    row4.add(digit_button(&state, 1));
    row4.add(digit_button(&state, 2));
    row4.add(digit_button(&state, 3));
    row4.add(op_button(&state, Operator::Add));

    let mut row5 = View::new();
    row5.set_margin(Edge::Vertical, PADDING);
    row5.set_flex_direction(FlexDirection::Row);
    row5.set_flex_basis(75.);
    row5.add(digit_button(&state, 0));
    row5.add(op_button(&state, Operator::Decimal));
    row5.add(op_button_2x(&state, Operator::Solve));

    let mut panel = View::new();
    panel.set_padding(Edge::All, PADDING);
    panel.add(row1);
    panel.add(row2);
    panel.add(row3);
    panel.add(row4);
    panel.add(row5);

    let mut main = View::new();
    main.set_background_color(0xffffffff);
    main.add(row0);
    main.add(panel);

    window.set_view(main);
    window.show();

    c.run();
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

impl Operator {
    fn apply(&self, a: f64, b: f64) -> f64 {
        match *self {
            Operator::Percent => a % b,
            Operator::Divide => a / b,
            Operator::Multiply => a * b,
            Operator::Subtract => a - b,
            Operator::Add => a + b,

            _ => unreachable!(),
        }
    }
}

fn button() -> Text {
    let mut button = Text::new();
    button.set_flex_basis(75.);
    button.set_margin(Edge::Horizontal, 5.);
    button.set_corner_radius(6.);
    button.set_gravity(Gravity::CENTER);
    button.set_text_color(0xff_353535);
    button.set_text_size(20.);

    // TODO: Reduce font creation
    // TODO: Does it make sense to build fonts as a user or use `.set_font_xxx` type functions
    button.set_font(&Font::builder().weight(500).build());

    button
}

fn op_button(state: &Arc<Mutex<State>>, op: Operator) -> Text {
    let state = state.clone();
    let mut button = button();

    let (background_color, background_color_pressed, color) = match op {
        Operator::Add | Operator::Subtract | Operator::Multiply | Operator::Divide => {
            (0xff_bddff9, 0xff_6cb8f1, 0xff_0070e5)
        }
        Operator::Solve => (0xff_0070e5, 0xff_0059b7, 0xff_ffffff),
        Operator::Clear => (0xff_ffd5d4, 0xff_ff7976, 0xff_dd5353),
        _ => (0xff_f2f2f2, 0xff_c2c2c2, 0xff_858585),
    };

    let ch = match op {
        Operator::Add => "+",
        Operator::Subtract => "\u{2212}",
        Operator::Multiply => "\u{00D7}",
        Operator::Divide => "\u{00F7}",
        Operator::Solve => "=",
        Operator::Percent => "%",
        Operator::Clear => "AC",
        Operator::InvertSign => "\u{00B1}",
        Operator::Decimal => ".",
    };

    button.set_text(ch);
    button.set_text_color(color);
    button.set_background_color(background_color);

    button.mouse_down().add(move |_| {
        button.set_background_color(background_color_pressed);
    });

    button.mouse_up().add(move |_| {
        let mut state = state.lock().unwrap();

        match op {
            Operator::Clear => {
                state.operand = None;
                state.last_operand = None;
                state.operator = None;
            }

            Operator::Add
            | Operator::Subtract
            | Operator::Multiply
            | Operator::Divide
            | Operator::Percent => {
                if let Some(operator) = &state.operator {
                    if let Some(operand) = state.operand {
                        if let Some(last_operand) = state.last_operand {
                            state.operand = Some(operator.apply(last_operand, operand));
                        }
                    }
                }

                state.operator = Some(op);
            }

            _ => {}
        }

        if let Some(operand) = state.operand.take() {
            state.last_operand = Some(operand);
        }

        // Set the background to the normal state
        button.set_background_color(background_color);

        // Update the operand label
        let operand_str = state.last_operand.unwrap_or_default().to_string();
        state.operand_label.set_text(operand_str);
    });

    button
}

fn digit_button(state: &Arc<Mutex<State>>, digit: u8) -> Text {
    const BACKGROUND_COLOR_NORMAL: u32 = 0xff_f2f2f2;
    const BACKGROUND_COLOR_PRESSED: u32 = 0xff_c2c2c2;

    let state = state.clone();

    let mut button = button();
    button.set_text(&digit.to_string());
    button.set_background_color(BACKGROUND_COLOR_NORMAL);

    button.mouse_down().add(move |_| {
        button.set_background_color(BACKGROUND_COLOR_PRESSED);
    });

    button.mouse_up().add(move |_| {
        let mut state = state.lock().unwrap();

        // Add a new digit to the end
        state.operand = Some((state.operand.unwrap_or_default() * 10.) + f64::from(digit));

        // Set the background to the normal state
        button.set_background_color(BACKGROUND_COLOR_NORMAL);

        // Update the operand label
        let operand_str = state.operand.unwrap_or_default().to_string();
        state.operand_label.set_text(operand_str);
    });

    button
}

fn op_button_2x(state: &Arc<Mutex<State>>, op: Operator) -> Text {
    let mut button = op_button(state, op);
    // TODO: button.set_flex_basis(button.flex_basis() * 2.);
    button.set_flex_basis(160.);
    button
}
