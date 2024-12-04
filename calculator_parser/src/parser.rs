#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn onenine<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(49, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn digit<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _ordered_choice_match_range(48, 57);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn integer<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'-');
    let closure_2 = _optional(&closure_1);
    let closure_3 = move |parent: Key, source: &Source, position: u32| {
        onenine(parent, context, source, position)
    };
    let closure_4 =
        move |parent: Key, source: &Source, position: u32| digit(parent, context, source, position);
    let closure_5 = _one_or_more(&closure_4);
    let closure_6 = _sequence(&closure_3, &closure_5);
    let closure_7 = _subexpression(&closure_6);
    let closure_8 =
        move |parent: Key, source: &Source, position: u32| digit(parent, context, source, position);
    let closure_9 = _ordered_choice(&closure_7, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    let closure_11 = _sequence(&closure_2, &closure_10);
    closure_11(parent, source, position)
}
#[allow(dead_code)]
pub fn number<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _var_name(Rules::integer, context, integer);
    let closure_2 = _var_name(Rules::fraction, context, fraction);
    let closure_3 = _optional(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _var_name(Rules::exponent, context, exponent);
    let closure_6 = _optional(&closure_5);
    let closure_7 = _sequence(&closure_4, &closure_6);
    closure_7(parent, source, position)
}
#[allow(dead_code)]
pub fn fraction<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'.');
    let closure_2 =
        move |parent: Key, source: &Source, position: u32| digit(parent, context, source, position);
    let closure_3 = _one_or_more(&closure_2);
    let closure_4 = _sequence(&closure_1, &closure_3);
    let closure_5 = _subexpression(&closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn exponent<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'E');
    let closure_2 = _terminal(b'e');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _var_name(Rules::sign, context, sign);
    let closure_6 = _sequence(&closure_4, &closure_5);
    let closure_7 =
        move |parent: Key, source: &Source, position: u32| digit(parent, context, source, position);
    let closure_8 = _one_or_more(&closure_7);
    let closure_9 = _sequence(&closure_6, &closure_8);
    let closure_10 = _subexpression(&closure_9);
    closure_10(parent, source, position)
}
#[allow(dead_code)]
pub fn sign<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let closure_1 = _terminal(b'+');
    let closure_2 = _terminal(b'-');
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _subexpression(&closure_3);
    let closure_5 = _optional(&closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn div_expr<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set = vec![Rules::expr_divmul, Rules::div_expr, Rules::mult_expr];

    let closure_1 =
        _var_name_indirect_left_recursion(&involved_set, Rules::expr_divmul, context, expr_divmul);
    let closure_2 = _terminal(b'/');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::term, context, term);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn mult_expr<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set = vec![Rules::expr_divmul, Rules::div_expr, Rules::mult_expr];

    let closure_1 =
        _var_name_indirect_left_recursion(&involved_set, Rules::expr_divmul, context, expr_divmul);
    let closure_2 = _terminal(b'*');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::term, context, term);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn add_expr<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set = vec![
        Rules::expr_addsub,
        Rules::add_expr,
        Rules::sub_expr,
        Rules::expr_divmul,
        Rules::div_expr,
        Rules::mult_expr,
    ];

    let closure_1 =
        _var_name_indirect_left_recursion(&involved_set, Rules::expr_addsub, context, expr_addsub);
    let closure_2 = _terminal(b'+');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 =
        _var_name_indirect_left_recursion(&involved_set, Rules::expr_divmul, context, expr_divmul);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn sub_expr<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set = vec![
        Rules::expr_addsub,
        Rules::add_expr,
        Rules::sub_expr,
        Rules::expr_divmul,
        Rules::div_expr,
        Rules::mult_expr,
    ];

    let closure_1 =
        _var_name_indirect_left_recursion(&involved_set, Rules::expr_addsub, context, expr_addsub);
    let closure_2 = _terminal(b'-');
    let closure_3 = _sequence(&closure_1, &closure_2);
    let closure_4 =
        _var_name_indirect_left_recursion(&involved_set, Rules::expr_divmul, context, expr_divmul);
    let closure_5 = _sequence(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn term<T: Context>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    // For now we just use a json number and don't differentiate between integers and floats since it doesn't affect the left recursion test value
    let closure_1 = _var_name(Rules::number, context, number);
    closure_1(parent, source, position)
}
#[allow(dead_code)]
pub fn expr_divmul<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set = vec![Rules::expr_divmul, Rules::div_expr, Rules::mult_expr];

    let closure_1 =
        _var_name_indirect_left_recursion(&involved_set, Rules::div_expr, context, div_expr);
    let closure_2 =
        _var_name_indirect_left_recursion(&involved_set, Rules::mult_expr, context, mult_expr);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 = _var_name(Rules::term, context, term);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn expr_addsub<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set = vec![
        Rules::expr_addsub,
        Rules::add_expr,
        Rules::sub_expr,
        Rules::expr_divmul,
        Rules::div_expr,
        Rules::mult_expr,
    ];

    let closure_1 =
        _var_name_indirect_left_recursion(&involved_set, Rules::add_expr, context, add_expr);
    let closure_2 =
        _var_name_indirect_left_recursion(&involved_set, Rules::sub_expr, context, sub_expr);
    let closure_3 = _ordered_choice(&closure_1, &closure_2);
    let closure_4 =
        _var_name_indirect_left_recursion(&involved_set, Rules::expr_divmul, context, expr_divmul);
    let closure_5 = _ordered_choice(&closure_3, &closure_4);
    closure_5(parent, source, position)
}
#[allow(dead_code)]
pub fn grammar<T: Context + 'static>(
    parent: Key,
    context: &RefCell<T>,
    source: &Source,
    position: u32,
) -> (bool, u32) {
    let involved_set = vec![Rules::expr_addsub, Rules::add_expr, Rules::sub_expr];

    let closure_1 =
        _var_name_indirect_left_recursion(&involved_set, Rules::expr_addsub, context, expr_addsub);
    closure_1(parent, source, position)
}
