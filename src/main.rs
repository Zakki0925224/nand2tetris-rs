use hardware::gates::*;

use crate::hardware::{calcs::*, consts::*};

mod hardware;

fn main()
{
    println!("{}", not(false));
}

#[test]
fn not_test()
{
    // a | out
    // 0 |  1
    // 1 |  0

    assert_eq!(not(false), true);
    assert_eq!(not(true), false);
}

#[test]
fn and_test()
{
    // a | b | out
    // 0 | 0 |  0
    // 0 | 1 |  0
    // 1 | 0 |  0
    // 1 | 1 |  1

    assert_eq!(and(false, false), false);
    assert_eq!(and(false, true), false);
    assert_eq!(and(true, false), false);
    assert_eq!(and(true, true), true);
}

#[test]
fn or_test()
{
    // a | b | out
    // 0 | 0 |  0
    // 0 | 1 |  1
    // 1 | 0 |  1
    // 1 | 1 |  1

    assert_eq!(or(false, false), false);
    assert_eq!(or(false, true), true);
    assert_eq!(or(true, false), true);
    assert_eq!(or(true, true), true);
}

#[test]
fn xor_test()
{
    // a | b | out
    // 0 | 0 |  0
    // 0 | 1 |  1
    // 1 | 0 |  1
    // 1 | 1 |  0

    assert_eq!(xor(false, false), false);
    assert_eq!(xor(false, true), true);
    assert_eq!(xor(true, false), true);
    assert_eq!(xor(true, true), false);
}

#[test]
fn mux_test()
{
    // a | b | sel | out
    // 0 | 0 |  0  |  0
    // 0 | 1 |  0  |  0
    // 1 | 0 |  0  |  1
    // 1 | 1 |  0  |  1
    // 0 | 0 |  1  |  0
    // 0 | 1 |  1  |  1
    // 1 | 0 |  1  |  0
    // 1 | 1 |  1  |  1

    // sel | out
    //  0  |  a
    //  1  |  b

    assert_eq!(mux(false, false, false), false);
    assert_eq!(mux(false, true, false), false);
    assert_eq!(mux(true, false, false), true);
    assert_eq!(mux(true, true, false), true);
    assert_eq!(mux(false, false, true), false);
    assert_eq!(mux(false, true, true), true);
    assert_eq!(mux(true, false, true), false);
    assert_eq!(mux(true, true, true), true);
}

#[test]
fn dmux_test()
{
    // a | sel | (out, out)
    // 0 |  0  | ( 0 ,  0 )
    // 0 |  1  | ( 0 ,  0 )
    // 1 |  0  | ( 1 ,  0 )
    // 1 |  1  | ( 0 ,  1 )

    assert_eq!(dmux(false, false), (false, false));
    assert_eq!(dmux(false, true), (false, false));
    assert_eq!(dmux(true, false), (true, false));
    assert_eq!(dmux(true, true), (false, true));
}

#[test]
fn add16_test()
{
    // |        a         |        b         |       out        |
    // | 0000000000000000 | 0000000000000000 | 0000000000000000 |
    // | 0000000000000000 | 1111111111111111 | 1111111111111111 |
    // | 1111111111111111 | 1111111111111111 | 1111111111111110 |
    // | 1010101010101010 | 0101010101010101 | 1111111111111111 |
    // | 0011110011000011 | 0000111111110000 | 0100110010110011 |
    // | 0001001000110100 | 1001100001110110 | 1010101010101010 |
    let mut a = ZERO16;
    let mut b = ZERO16;
    let mut out = ZERO16;
    assert_eq!(add16(a, b), out);

    b = MAX16;
    out = MAX16;
    assert_eq!(add16(a, b), out);

    a = MAX16;
    out[0] = false;
    assert_eq!(add16(a, b), out);

    a = [
        false, true, false, true, false, true, false, true, false, true, false, true, false, true,
        false, true,
    ];
    b = [
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false,
    ];
    out[0] = true;
    assert_eq!(add16(a, b), out);

    a = [
        true, true, false, false, false, false, true, true, false, false, true, true, true, true,
        false, false,
    ];
    b = [
        false, false, false, false, true, true, true, true, true, true, true, true, false, false,
        false, false,
    ];
    out = [
        true, true, false, false, true, true, false, true, false, false, true, true, false, false,
        true, false,
    ];
    assert_eq!(add16(a, b), out);
}

#[test]
fn alu_test()
{
    // |        x         |        y         |zx |nx |zy |ny | f |no |       out        |zr |ng |
    // | 0000000000000000 | 1111111111111111 | 1 | 0 | 1 | 0 | 1 | 0 | 0000000000000000 | 1 | 0 |
    // | 0000000000000000 | 1111111111111111 | 1 | 1 | 1 | 1 | 1 | 1 | 0000000000000001 | 0 | 0 |
    // | 0000000000000000 | 1111111111111111 | 1 | 1 | 1 | 0 | 1 | 0 | 1111111111111111 | 0 | 1 |
    // | 0000000000000000 | 1111111111111111 | 0 | 0 | 1 | 1 | 0 | 0 | 0000000000000000 | 1 | 0 |
    // | 0000000000000000 | 1111111111111111 | 1 | 1 | 0 | 0 | 0 | 0 | 1111111111111111 | 0 | 1 |
    // | 0000000000000000 | 1111111111111111 | 0 | 0 | 1 | 1 | 0 | 1 | 1111111111111111 | 0 | 1 |
    // | 0000000000000000 | 1111111111111111 | 1 | 1 | 0 | 0 | 0 | 1 | 0000000000000000 | 1 | 0 |
    // | 0000000000000000 | 1111111111111111 | 0 | 0 | 1 | 1 | 1 | 1 | 0000000000000000 | 1 | 0 |
    // | 0000000000000000 | 1111111111111111 | 1 | 1 | 0 | 0 | 1 | 1 | 0000000000000001 | 0 | 0 |
    // | 0000000000000000 | 1111111111111111 | 0 | 1 | 1 | 1 | 1 | 1 | 0000000000000001 | 0 | 0 |
    // | 0000000000000000 | 1111111111111111 | 1 | 1 | 0 | 1 | 1 | 1 | 0000000000000000 | 1 | 0 |
    // | 0000000000000000 | 1111111111111111 | 0 | 0 | 1 | 1 | 1 | 0 | 1111111111111111 | 0 | 1 |
    // | 0000000000000000 | 1111111111111111 | 1 | 1 | 0 | 0 | 1 | 0 | 1111111111111110 | 0 | 1 |
    // | 0000000000000000 | 1111111111111111 | 0 | 0 | 0 | 0 | 1 | 0 | 1111111111111111 | 0 | 1 |
    // | 0000000000000000 | 1111111111111111 | 0 | 1 | 0 | 0 | 1 | 1 | 0000000000000001 | 0 | 0 |
    // | 0000000000000000 | 1111111111111111 | 0 | 0 | 0 | 1 | 1 | 1 | 1111111111111111 | 0 | 1 |
    // | 0000000000000000 | 1111111111111111 | 0 | 0 | 0 | 0 | 0 | 0 | 0000000000000000 | 1 | 0 |
    // | 0000000000000000 | 1111111111111111 | 0 | 1 | 0 | 1 | 0 | 1 | 1111111111111111 | 0 | 1 |
    // | 0000000000010001 | 0000000000000011 | 1 | 0 | 1 | 0 | 1 | 0 | 0000000000000000 | 1 | 0 |
    // | 0000000000010001 | 0000000000000011 | 1 | 1 | 1 | 1 | 1 | 1 | 0000000000000001 | 0 | 0 |
    // | 0000000000010001 | 0000000000000011 | 1 | 1 | 1 | 0 | 1 | 0 | 1111111111111111 | 0 | 1 |
    // | 0000000000010001 | 0000000000000011 | 0 | 0 | 1 | 1 | 0 | 0 | 0000000000010001 | 0 | 0 |
    // | 0000000000010001 | 0000000000000011 | 1 | 1 | 0 | 0 | 0 | 0 | 0000000000000011 | 0 | 0 |
    // | 0000000000010001 | 0000000000000011 | 0 | 0 | 1 | 1 | 0 | 1 | 1111111111101110 | 0 | 1 |
    // | 0000000000010001 | 0000000000000011 | 1 | 1 | 0 | 0 | 0 | 1 | 1111111111111100 | 0 | 1 |
    // | 0000000000010001 | 0000000000000011 | 0 | 0 | 1 | 1 | 1 | 1 | 1111111111101111 | 0 | 1 |
    // | 0000000000010001 | 0000000000000011 | 1 | 1 | 0 | 0 | 1 | 1 | 1111111111111101 | 0 | 1 |
    // | 0000000000010001 | 0000000000000011 | 0 | 1 | 1 | 1 | 1 | 1 | 0000000000010010 | 0 | 0 |
    // | 0000000000010001 | 0000000000000011 | 1 | 1 | 0 | 1 | 1 | 1 | 0000000000000100 | 0 | 0 |
    // | 0000000000010001 | 0000000000000011 | 0 | 0 | 1 | 1 | 1 | 0 | 0000000000010000 | 0 | 0 |
    // | 0000000000010001 | 0000000000000011 | 1 | 1 | 0 | 0 | 1 | 0 | 0000000000000010 | 0 | 0 |
    // | 0000000000010001 | 0000000000000011 | 0 | 0 | 0 | 0 | 1 | 0 | 0000000000010100 | 0 | 0 |
    // | 0000000000010001 | 0000000000000011 | 0 | 1 | 0 | 0 | 1 | 1 | 0000000000001110 | 0 | 0 |
    // | 0000000000010001 | 0000000000000011 | 0 | 0 | 0 | 1 | 1 | 1 | 1111111111110010 | 0 | 1 |
    // | 0000000000010001 | 0000000000000011 | 0 | 0 | 0 | 0 | 0 | 0 | 0000000000000001 | 0 | 0 |
    // | 0000000000010001 | 0000000000000011 | 0 | 1 | 0 | 1 | 0 | 1 | 0000000000010011 | 0 | 0 |

    let mut x = ZERO16;
    let mut y = MAX16;
    let mut out = ZERO16;

    let mut zx = true;
    let mut nx = false;
    let mut zy = true;
    let mut ny = false;
    let mut f = true;
    let mut no = false;

    assert_eq!(alu(x, y, zx, zy, nx, ny, f, no), (out, true, false));

    zx = true;
    nx = true;
    zy = true;
    ny = true;
    f = true;
    no = true;

    out = ONE16;
    assert_eq!(alu(x, y, zx, zy, nx, ny, f, no), (out, false, false));
}
