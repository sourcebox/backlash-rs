//! Unit tests

use super::*;

/// Ramp i32 value up, then down, then up again starting at 0.
#[test]
fn up_down_up_i32_from_0() {
    const DEADBAND_WIDTH: i32 = 4;

    let mut backlash = Backlash::<i32>::new(DEADBAND_WIDTH);

    let input_values = [0, 1, 2, 3, 4, 3, 2, 1, 0, -1, -2, -3, -2, -1, 0, 1, 2];
    let expected_values = [0, 0, 0, 1, 2, 2, 2, 2, 2, 1, 0, -1, -1, -1, -1, -1, 0];

    for (input_value, expected_value) in input_values.into_iter().zip(expected_values.into_iter()) {
        let value = backlash.update(input_value);
        assert_eq!(value, expected_value);
    }
}

/// Ramp f32 value up, then down, then up again starting at 0.
#[test]
fn up_down_up_f32_from_0() {
    const DEADBAND_WIDTH: f32 = 4.0;

    let mut backlash = Backlash::<f32>::new(DEADBAND_WIDTH);

    let input_values = [
        0.0, 1.0, 2.0, 3.0, 4.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0,
    ];
    let expected_values = [
        0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 1.0, 0.0, -1.0, -1.0, -1.0, -1.0, -1.0, 0.0,
    ];

    for (input_value, expected_value) in input_values.into_iter().zip(expected_values.into_iter()) {
        let value = backlash.update(input_value);
        assert_eq!(value, expected_value);
    }
}

/// Ramp i32 value up, then down starting with an initial value.
#[test]
fn up_down_i32_from_initial() {
    const DEADBAND_WIDTH: i32 = 4;

    let mut backlash = Backlash::<i32>::new(DEADBAND_WIDTH);

    let input_values = [20, 21, 22, 23, 22, 19, 16];
    let expected_values = [18, 19, 20, 21, 21, 21, 18];

    for (input_value, expected_value) in input_values.into_iter().zip(expected_values.into_iter()) {
        let value = backlash.update(input_value);
        assert_eq!(value, expected_value);
    }
}

/// Sets different deadband widths.
#[test]
fn set_deadband_width() {
    const DEADBAND_WIDTH: i32 = 4;
    const DEADBAND_WIDTH_2: i32 = 30;

    let mut backlash = Backlash::<i32>::new(DEADBAND_WIDTH);
    assert_eq!(backlash.deadband_width(), DEADBAND_WIDTH);

    backlash.set_deadband_width(DEADBAND_WIDTH_2);
    assert_eq!(backlash.deadband_width(), DEADBAND_WIDTH_2);
}

/// Borders check.
#[test]
fn borders() {
    const DEADBAND_WIDTH: i32 = 10;

    let mut backlash = Backlash::<i32>::new(DEADBAND_WIDTH);
    assert_eq!(backlash.borders(), (-5, 5));

    let input_values = [40, -6, -4];
    let expected_values = [(30, 40), (-6, 4), (-6, 4)];

    for (input_value, expected_value) in input_values.into_iter().zip(expected_values.into_iter()) {
        backlash.update(input_value);
        assert_eq!(backlash.borders(), expected_value);
    }
}

/// Center borders around a value.
#[test]
fn center_borders() {
    const DEADBAND_WIDTH: i32 = 10;

    let mut backlash = Backlash::<i32>::new(DEADBAND_WIDTH);
    assert_eq!(backlash.borders(), (-5, 5));

    let input_values = [40, -6, 4];
    let expected_values = [(35, 45), (-11, -1), (-1, 9)];

    for (input_value, expected_value) in input_values.into_iter().zip(expected_values.into_iter()) {
        backlash.center_borders(input_value);
        assert_eq!(backlash.borders(), expected_value);
    }
}
