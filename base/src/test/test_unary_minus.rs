#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

#[test]
fn double_minus_coerces_to_numbers() {
    let mut model = new_empty_model();
    model._set("A1", "1");
    model._set("A2", "2");
    model._set("A3", "3");
    model._set("B1", "=--TRUE");
    model._set("B2", "=--FALSE");
    model._set("B3", "=--\"3\"");
    model._set("B4", "=---1");
    model._set("B5", "=-(-TRUE)");
    model._set("B6", "=--\"abc\"");
    model._set("B7", "=--A2");
    model.evaluate();

    assert_eq!(model._get_text("B1"), *"1");
    assert_eq!(model._get_text("B2"), *"0");
    assert_eq!(model._get_text("B3"), *"3");
    assert_eq!(model._get_text("B4"), *"-1");
    assert_eq!(model._get_text("B5"), *"1");
    assert_eq!(model._get_text("B6"), *"#VALUE!");
    assert_eq!(model._get_text("B7"), *"2");
    assert_eq!(model._get_formula("B1"), *"=--TRUE");
}

#[test]
fn unary_operators_work_on_arrays() {
    let mut model = new_empty_model();
    model._set("A1", "1");
    model._set("A2", "2");
    model._set("A3", "3");
    model._set("B1", "=SUMPRODUCT(--(A1:A3>1))");
    model._set("B2", "=SUMPRODUCT(--(A1:A3>=2),A1:A3)");
    model._set("B3", "=SUMPRODUCT(--(A1:A3>1),--(A1:A3<3))");
    model._set("B4", "=SUM(-A1:A3)");
    model._set("B5", "=SUM(-{1,2,3})");
    model._set("B6", "=SUM((A1:A3)%)");
    model._set("B7", "=SUM(-(A1:A3/0))");
    model.evaluate();

    assert_eq!(model._get_text("B1"), *"2");
    assert_eq!(model._get_text("B2"), *"5");
    assert_eq!(model._get_text("B3"), *"1");
    assert_eq!(model._get_text("B4"), *"-6");
    assert_eq!(model._get_text("B5"), *"-6");
    assert_eq!(model._get_text("B6"), *"0.06");
    assert_eq!(model._get_text("B7"), *"#DIV/0!");
}

#[test]
fn unary_minus_keeps_its_meaning_when_written_back() {
    let mut model = new_empty_model();
    model._set("A1", "=-(2*3)^2");
    model._set("A2", "=-2^2");
    model._set("A3", "=-(1&2)");
    model._set("A4", "=-(B1>1)");
    model._set("B1", "2");
    model.evaluate();

    assert_eq!(model._get_text("A1"), *"36");
    assert_eq!(model._get_text("A2"), *"4");
    assert_eq!(model._get_text("A3"), *"-12");
    assert_eq!(model._get_text("A4"), *"-1");

    // The formulas are written back with the parentheses they need, so setting
    // the written text again gives the same values.
    for cell in ["A1", "A2", "A3", "A4"] {
        let formula = model._get_formula(cell);
        model._set(cell, &formula);
    }
    model.evaluate();
    assert_eq!(model._get_formula("A1"), *"=-(2*3)^2");
    assert_eq!(model._get_formula("A3"), *"=-(1&2)");
    assert_eq!(model._get_formula("A4"), *"=-(B1>1)");
    assert_eq!(model._get_text("A1"), *"36");
    assert_eq!(model._get_text("A2"), *"4");
    assert_eq!(model._get_text("A3"), *"-12");
    assert_eq!(model._get_text("A4"), *"-1");
}
