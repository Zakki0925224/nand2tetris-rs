use hardware::gate::*;

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
