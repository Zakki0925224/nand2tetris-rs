use super::{consts::{MAX16, ZERO16}, gates::{and16, mux4way16, not16, or}, BoolArray16};

pub fn half_adder(a: bool, b: bool) -> (bool, bool)
{
    let mut sum = false;
    let mut carry = false;

    if a && b
    {
        carry = true;
    }
    else
    {
        sum = or(a, b);
    }

    return (sum, carry);
}

pub fn full_addr(a: bool, b: bool, c: bool) -> (bool, bool)
{
    if a == false
    {
        return half_adder(b, c);
    }

    let mut sum = false;
    let mut carry = false;

    if !b && !c
    {
        sum = true;
    }
    else if (!b && c) || (b && !c)
    {
        carry = true;
    }
    else
    {
        sum = true;
        carry = true;
    }

    return (sum, carry);
}

pub fn add16(a: BoolArray16, b: BoolArray16) -> BoolArray16
{
    let mut c = ZERO16;
    let mut carry = false;

    for i in 0..a.len()
    {
        let (sum, crr) = full_addr(a[i], b[i], carry);
        c[i] = sum;
        carry = crr;
    }

    return c;
}

pub fn inc16(a: BoolArray16) -> BoolArray16
{
    let mut b = ZERO16;
    b[b.len() - 1] = true;

    return add16(a, b);
}

pub fn alu(
    x: BoolArray16,
    y: BoolArray16,
    zx: bool, // x to zero
    zy: bool, // y to zero
    nx: bool, // negate x
    ny: bool, // negate y
    f: bool,  // false -> and, true -> add
    no: bool, // negate out
) -> (BoolArray16, bool, bool)
{
    let mut zr = false; // if out == 0
    let mut ng = false; // if out < 0

    let x = mux4way16(x, not16(x), ZERO16, MAX16, [zx, nx]);
    let y = mux4way16(y, not16(y), ZERO16, MAX16, [zy, ny]);

    let out = mux4way16(and16(x, y), add16(x, y), not16(and16(x, y)), not16(add16(x, y)), [no, f]);

    if !out.contains(&true)
    {
        zr = true;
    }

    if out[out.len() - 1]
    {
        ng = true;
    }

    return (out, zr, ng);
}
