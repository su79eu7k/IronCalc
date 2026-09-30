#![allow(clippy::panic)]

use std::collections::HashMap;

use crate::expressions::parser::move_formula::{move_formula, MoveContext};
use crate::expressions::parser::tests::utils::{new_parser, to_english_localized_string};
use crate::expressions::parser::Node;
use crate::expressions::token::OpUnary;
use crate::expressions::types::{Area, CellReferenceRC};
use crate::language::get_default_language;
use crate::locale::get_default_locale;

fn context() -> CellReferenceRC {
    CellReferenceRC {
        sheet: "Sheet1".to_string(),
        row: 1,
        column: 1,
    }
}

fn count_minus_signs(node: &Node) -> usize {
    match node {
        Node::UnaryKind {
            kind: OpUnary::Minus,
            right,
        } => 1 + count_minus_signs(right),
        _ => 0,
    }
}

#[test]
fn every_minus_sign_is_kept() {
    let worksheets = vec!["Sheet1".to_string()];
    let mut parser = new_parser(worksheets, vec![], HashMap::new());
    let context = context();

    for (formula, minus_signs) in [("-A1", 1), ("--A1", 2), ("---A1", 3), ("+-+-A1", 2)] {
        let t = parser.parse(formula, &context);
        assert_eq!(count_minus_signs(&t), minus_signs, "{formula}");
    }
    let t = parser.parse("--TRUE", &context);
    assert_eq!(to_english_localized_string(&t, &context), "--TRUE");
    let t = parser.parse("SUMPRODUCT(--(A1:A3>1))", &context);
    assert_eq!(
        to_english_localized_string(&t, &context),
        "SUMPRODUCT(--(A1:A3>1))"
    );
}

#[test]
fn unary_minus_keeps_parentheses_of_its_operand() {
    let worksheets = vec!["Sheet1".to_string()];
    let mut parser = new_parser(worksheets, vec![], HashMap::new());
    let context = context();

    for formula in [
        "-(A1>1)", "-(1&2)", "-(2*3)^2", "-(A1/B1)", "-(A1+B1)", "-(A1^2)", "-(5%)",
    ] {
        let t = parser.parse(formula, &context);
        let text = to_english_localized_string(&t, &context);
        assert_eq!(text, formula);
        // Parsing the text again gives the same formula.
        let again = parser.parse(&text, &context);
        assert_eq!(to_english_localized_string(&again, &context), formula);
    }

    // Parentheses that are not needed are still dropped.
    let t = parser.parse("-(A1)", &context);
    assert_eq!(to_english_localized_string(&t, &context), "-A1");
    let t = parser.parse("-(SUM(A1:A3))", &context);
    assert_eq!(to_english_localized_string(&t, &context), "-SUM(A1:A3)");
}

#[test]
fn moved_unary_minus_keeps_parentheses_of_its_operand() {
    let worksheets = vec!["Sheet1".to_string()];
    let mut parser = new_parser(worksheets, vec![], HashMap::new());
    let context = context();
    // Moving A1:B1 one row down.
    let area = &Area {
        sheet: 0,
        row: 1,
        column: 1,
        width: 2,
        height: 1,
    };
    let move_context = MoveContext {
        source_sheet_name: "Sheet1",
        row: 1,
        column: 1,
        area,
        target_sheet_name: "Sheet1",
        row_delta: 1,
        column_delta: 0,
    };
    let locale = get_default_locale();
    let language = get_default_language();
    for (formula, moved) in [
        ("-(A1+B1)", "-(A2+B2)"),
        ("-(A1>1)", "-(A2>1)"),
        ("-(1&A1)", "-(1&A2)"),
        ("-(A1*B1)^2", "-(A2*B2)^2"),
        ("--A1", "--A2"),
    ] {
        let t = parser.parse(formula, &context);
        assert_eq!(
            move_formula(&t, &move_context, locale, language),
            moved,
            "{formula}"
        );
    }
}
