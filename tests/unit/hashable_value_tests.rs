use crate::{ArrayType, Value};
use std::collections::HashSet;

/// Each case: values with repeats, and the distinct values a set must keep of them, none
/// of which may collapse into another.
#[test]
fn a_set_keeps_one_of_each_equal_value() {
    let int_array = || Value::from(vec![0_i32, 1, 2]);
    let float_array = || Value::from(vec![0_f32, 1.0, 2.0]);
    let other_float_array = || Value::from(vec![3_f32, 2.0, 1.0]);

    let cases = [
        (
            "a NULL of each variant is its own value; NaN equals NaN but not NULL",
            vec![
                Value::Int(None),
                Value::Int(None),
                Value::BigInt(None),
                Value::BigInt(None),
                Value::Float(None),
                Value::Float(None),
                Value::Float(Some(f32::NAN)),
                Value::Float(Some(f32::NAN)),
                Value::Double(None),
                Value::Double(None),
                Value::Double(Some(f64::NAN)),
                Value::Double(Some(f64::NAN)),
            ],
            vec![
                Value::Int(None),
                Value::BigInt(None),
                Value::Float(None),
                Value::Double(None),
                Value::Float(Some(f32::NAN)),
                Value::Double(Some(f64::NAN)),
            ],
        ),
        (
            "equal numbers of one variant collapse; equal numbers of two do not",
            vec![
                Value::Int(None),
                Value::Int(Some(1)),
                Value::Int(Some(1)),
                Value::BigInt(Some(2)),
                Value::BigInt(Some(2)),
                Value::Float(Some(3.0)),
                Value::Float(Some(3.0)),
                Value::Double(Some(3.0)),
                Value::Double(Some(3.0)),
                Value::BigInt(Some(5)),
            ],
            vec![
                Value::Int(None),
                Value::Int(Some(1)),
                Value::BigInt(Some(2)),
                Value::Float(Some(3.0)),
                Value::Double(Some(3.0)),
                Value::BigInt(Some(5)),
            ],
        ),
        (
            "arrays are equal when their element type and elements are",
            vec![
                int_array(),
                int_array(),
                float_array(),
                float_array(),
                other_float_array(),
            ],
            vec![int_array(), float_array(), other_float_array()],
        ),
    ];

    for (case, values, distinct) in cases {
        let expected_len = distinct.len();
        let distinct: HashSet<Value> = distinct.into_iter().collect();
        assert_eq!(
            distinct.len(),
            expected_len,
            "{case}: distinct values collapsed"
        );

        let values: HashSet<Value> = values.into_iter().collect();
        assert_eq!(values, distinct, "{case}");
    }
}

#[test]
fn a_vec_becomes_a_typed_array() {
    let cases = [
        (
            Value::from(vec![0_i32, 1, 2]),
            ArrayType::Int,
            vec![
                Value::Int(Some(0)),
                Value::Int(Some(1)),
                Value::Int(Some(2)),
            ],
        ),
        (
            Value::from(vec![0_f32, 1.0, 2.0]),
            ArrayType::Float,
            vec![
                Value::Float(Some(0.0)),
                Value::Float(Some(1.0)),
                Value::Float(Some(2.0)),
            ],
        ),
    ];

    for (array, element_type, elements) in cases {
        assert_eq!(array, Value::Array(element_type, Some(Box::new(elements))));
    }
}
