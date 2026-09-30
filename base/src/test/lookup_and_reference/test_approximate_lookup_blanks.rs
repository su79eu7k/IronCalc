#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

// Approximate (binary search) lookups skip empty cells, as in Excel (see the
// Excel-saved tests/fixtures/lookup.xlsx of pycel, sheet LookupBlanks).
//
//   F: 1, 2, 3, <blank>, 5, 6, 7, 8
//   G: F * 10
fn model_with_blank_row() -> crate::Model<'static> {
    let mut model = new_empty_model();
    for (row, value) in [(1, 1), (2, 2), (3, 3), (5, 5), (6, 6), (7, 7), (8, 8)] {
        model._set(&format!("F{row}"), &value.to_string());
        model._set(&format!("G{row}"), &(value * 10).to_string());
    }
    model
}

#[test]
fn approximate_match_skips_blank_cells() {
    let mut model = model_with_blank_row();
    model._set("A1", "=MATCH(6,F1:F8)");
    model._set("A2", "=MATCH(9,F1:F8)");
    model._set("A3", "=MATCH(9,F1:F20)");
    model._set("A4", "=MATCH(5,F1:F20,0)");
    model.evaluate();

    assert_eq!(model._get_text("A1"), *"6");
    assert_eq!(model._get_text("A2"), *"8");
    assert_eq!(model._get_text("A3"), *"8");
    assert_eq!(model._get_text("A4"), *"5");
}

#[test]
fn approximate_lookups_skip_trailing_blank_cells() {
    let mut model = model_with_blank_row();
    model._set("A1", "=MATCH(9,F:F)");
    model._set("A2", "=VLOOKUP(6,F:G,2)");
    model._set("A3", "=VLOOKUP(9,F1:G20,2)");
    model._set("A4", "=VLOOKUP(7,F1:G20,2,FALSE)");
    model._set("A5", "=LOOKUP(9,F1:F20,G1:G20)");
    model.evaluate();

    assert_eq!(model._get_text("A1"), *"8");
    assert_eq!(model._get_text("A2"), *"60");
    assert_eq!(model._get_text("A3"), *"80");
    assert_eq!(model._get_text("A4"), *"70");
    assert_eq!(model._get_text("A5"), *"80");
}
