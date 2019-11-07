use pure;
use pure::scan::*;
use pure::scan::Radix::*;
use pure::scan::Lexeme::*;
use std::ops::Add;

fn to_lexemes<'a>(tokens: &'a Vec<Token>) -> Vec<&'a Lexeme<'a>> {
    tokens.iter().map(|token| token.lexeme()).collect()
}

#[test]
fn tokenize__number_binary() {
    let tokens = tokenize(
        "0b 0b0 0b1 0b10 0b01 0b__11 0b1_0 0b01_ 0b_\n"
    );
    let lexemes = to_lexemes(&tokens);

    assert_eq!(lexemes[0], &Number("0b", Binary));
    assert_eq!(lexemes[1], &Number("0b0", Binary));
    assert_eq!(lexemes[2], &Number("0b1", Binary));
    assert_eq!(lexemes[3], &Number("0b10", Binary));
    assert_eq!(lexemes[4], &Number("0b01", Binary));
    assert_eq!(lexemes[5], &Number("0b__11", Binary));
    assert_eq!(lexemes[6], &Number("0b1_0", Binary));
    assert_eq!(lexemes[7], &Number("0b01_", Binary));
    assert_eq!(lexemes[8], &Number("0b_", Binary));
}

#[test]
fn tokenize__number_not_binary() {
    let tokens = tokenize(
        "b 00b _0b 0_b b_ 0b1__b0 0b0_0xf\n"
    );
    let lexemes = to_lexemes(&tokens);

    assert_eq!(lexemes[0], &Identifier("b"));
    assert_eq!(lexemes[1], &Number("00", Decimal));
    assert_eq!(lexemes[2], &Identifier("b"));
    assert_eq!(lexemes[3], &Identifier("_0b"));
    assert_eq!(lexemes[4], &Number("0_", Decimal));
    assert_eq!(lexemes[5], &Identifier("b"));
    assert_eq!(lexemes[6], &Identifier("b_"));
    assert_eq!(lexemes[7], &Number("0b1__", Binary));
    assert_eq!(lexemes[8], &Identifier("b0"));
    assert_eq!(lexemes[9], &Number("0b0_0", Binary));
    assert_eq!(lexemes[10], &Identifier("xf"));
}

#[test]
fn tokenize__number_decimal_integer() {
    let tokens = tokenize(
        "0 00 000 001 01 1 29 3___8 47_ 5_\n"
    );
    let lexemes = to_lexemes(&tokens);

    assert_eq!(lexemes[0], &Number("0", Decimal));
    assert_eq!(lexemes[1], &Number("00", Decimal));
    assert_eq!(lexemes[2], &Number("000", Decimal));
    assert_eq!(lexemes[3], &Number("001", Decimal));
    assert_eq!(lexemes[4], &Number("01", Decimal));
    assert_eq!(lexemes[5], &Number("1", Decimal));
    assert_eq!(lexemes[6], &Number("29", Decimal));
    assert_eq!(lexemes[7], &Number("3___8", Decimal));
    assert_eq!(lexemes[8], &Number("47_", Decimal));
    assert_eq!(lexemes[9], &Number("5_", Decimal));
}

#[test]
fn tokenize__number_decimal_float() {
    let tokens = tokenize(
        "0. 0.0 0.1 1.0 2.9 3_.4_8 56.789_ 0._1\n"
    );
    let lexemes = to_lexemes(&tokens);

    assert_eq!(lexemes[0], &Number("0.", Decimal));
    assert_eq!(lexemes[1], &Number("0.0", Decimal));
    assert_eq!(lexemes[2], &Number("0.1", Decimal));
    assert_eq!(lexemes[3], &Number("1.0", Decimal));
    assert_eq!(lexemes[4], &Number("2.9", Decimal));
    assert_eq!(lexemes[5], &Number("3_.4_8", Decimal));
    assert_eq!(lexemes[6], &Number("56.789_", Decimal));
    assert_eq!(lexemes[7], &Number("0._1", Decimal));
}

#[test]
fn tokenize__number_not_decimal() {
    let tokens = tokenize(
        "_9 a\n"
    );
    let lexemes = to_lexemes(&tokens);

    assert_eq!(lexemes[0], &Identifier("_9"));
    assert_eq!(lexemes[1], &Identifier("a"));
}

#[test]
fn scan__tokenize__number_hexadecimal_good() {
    let tokens = tokenize(
        "0x 0x0 0x1 0x10 0x01 0x_2F 0xe_3 0x4D_ 0x__\n"
    );
    let lexemes = to_lexemes(&tokens);

    assert_eq!(lexemes[0], &Number("0x", Hexadecimal));
    assert_eq!(lexemes[1], &Number("0x0", Hexadecimal));
    assert_eq!(lexemes[2], &Number("0x1", Hexadecimal));
    assert_eq!(lexemes[3], &Number("0x10", Hexadecimal));
    assert_eq!(lexemes[4], &Number("0x01", Hexadecimal));
    assert_eq!(lexemes[5], &Number("0x_2F", Hexadecimal));
    assert_eq!(lexemes[6], &Number("0xe_3", Hexadecimal));
    assert_eq!(lexemes[7], &Number("0x4D_", Hexadecimal));
    assert_eq!(lexemes[8], &Number("0x__", Hexadecimal));
}

#[test]
fn tokenize__number_not_hexadecimal() {
    let tokens = tokenize(
        "x 00x _0x 0_x x_ 0x1__x0 0x0_0b0\n"
    );
    let lexemes = to_lexemes(&tokens);

    assert_eq!(lexemes[0], &Identifier("x"));
    assert_eq!(lexemes[1], &Number("00", Decimal));
    assert_eq!(lexemes[2], &Identifier("x"));
    assert_eq!(lexemes[3], &Identifier("_0x"));
    assert_eq!(lexemes[4], &Number("0_", Decimal));
    assert_eq!(lexemes[5], &Identifier("x"));
    assert_eq!(lexemes[6], &Identifier("x_"));
    assert_eq!(lexemes[7], &Number("0x1__", Hexadecimal));
    assert_eq!(lexemes[8], &Identifier("x0"));
    assert_eq!(lexemes[9], &Number("0x0_0b0", Hexadecimal));
}