use super::{consts::ZERO16, BoolArray16};

pub fn nand(a: bool, b: bool) -> bool { return !(a && b); }

pub fn not(a: bool) -> bool { return nand(true, a); }

pub fn and(a: bool, b: bool) -> bool { return not(nand(a, b)); }

pub fn or(a: bool, b: bool) -> bool { return nand(not(a), not(b)); }

pub fn xor(a: bool, b: bool) -> bool
{
    let c = nand(a, b);
    return nand(nand(a, c), nand(b, c));
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool { return or(and(b, sel), and(a, not(sel))); }

pub fn dmux(a: bool, sel: bool) -> (bool, bool)
{
    let b = nand(a, sel);
    let c = nand(a, b);

    return (nand(c, c), nand(b, b));
}

pub fn not16(a: BoolArray16) -> BoolArray16
{
    let mut b = ZERO16;

    for i in 0..b.len()
    {
        b[i] = not(a[i]);
    }

    return b;
}

pub fn and16(a: BoolArray16, b: BoolArray16) -> BoolArray16
{
    let mut c: BoolArray16 = ZERO16;

    for i in 0..c.len()
    {
        c[i] = and(a[i], b[i]);
    }

    return c;
}

pub fn or16(a: BoolArray16, b: BoolArray16) -> BoolArray16
{
    let mut c: BoolArray16 = ZERO16;

    for i in 0..c.len()
    {
        c[i] = or(a[i], b[i]);
    }

    return c;
}

pub fn mux16(a: BoolArray16, b: BoolArray16, sel: bool) -> BoolArray16
{
    let mut c: BoolArray16 = ZERO16;

    for i in 0..c.len()
    {
        if sel
        {
            c[i] = a[i];
        }
        else
        {
            c[i] = b[i];
        }
    }

    return c;
}

pub fn or8way(a: BoolArray16) -> bool
{
    for i in 0..a.len()
    {
        if a[i]
        {
            return true;
        }
    }

    return false;
}

pub fn mux4way16(
    a: BoolArray16,
    b: BoolArray16,
    c: BoolArray16,
    d: BoolArray16,
    sel: [bool; 2],
) -> BoolArray16
{
    let e: BoolArray16;

    if !sel[1] && !sel[0]
    {
        e = a;
    }
    else if !sel[1] && sel[0]
    {
        e = b;
    }
    else if sel[0] && !sel[1]
    {
        e = c;
    }
    else
    {
        e = d;
    }

    return e;
}

pub fn mux8way16(
    a: BoolArray16,
    b: BoolArray16,
    c: BoolArray16,
    d: BoolArray16,
    e: BoolArray16,
    f: BoolArray16,
    g: BoolArray16,
    h: BoolArray16,
    sel: [bool; 3],
) -> BoolArray16
{
    let i: BoolArray16;

    if !sel[2] && !sel[1] && !sel[0]
    {
        i = a;
    }
    else if !sel[2] && !sel[1] && sel[0]
    {
        i = b;
    }
    else if !sel[2] && sel[1] && !sel[0]
    {
        i = c;
    }
    else if !sel[2] && sel[1] && sel[0]
    {
        i = d;
    }
    else if sel[2] && !sel[1] && !sel[0]
    {
        i = e;
    }
    else if sel[2] && !sel[1] && sel[0]
    {
        i = f;
    }
    else if sel[2] && sel[1] && !sel[0]
    {
        i = g;
    }
    else
    {
        i = h;
    }

    return i;
}

pub fn dmux4way(a: bool, sel: [bool; 2]) -> [bool; 4]
{
    let mut b = false;
    let mut c = false;
    let mut d = false;
    let mut e = false;

    if !sel[1] && sel[0]
    {
        b = a;
    }
    else if !sel[1] && sel[0]
    {
        c = a;
    }
    else if sel[1] && !sel[0]
    {
        d = a;
    }
    else
    {
        e = a;
    }

    return [b, c, d, e];
}

pub fn dmux8way(a: bool, sel: [bool; 3]) -> [bool; 8]
{
    let mut b = false;
    let mut c = false;
    let mut d = false;
    let mut e = false;
    let mut f = false;
    let mut g = false;
    let mut h = false;
    let mut i = false;

    if !sel[2] && !sel[1] && !sel[0]
    {
        b = a;
    }
    else if !sel[2] && !sel[1] && sel[0]
    {
        c = a;
    }
    else if !sel[2] && sel[1] && !sel[0]
    {
        d = a;
    }
    else if !sel[2] && sel[1] && sel[0]
    {
        e = a;
    }
    else if sel[2] && !sel[1] && !sel[0]
    {
        f = a;
    }
    else if sel[2] && !sel[1] && sel[0]
    {
        g = a;
    }
    else if sel[2] && sel[1] && !sel[0]
    {
        h = a;
    }
    else
    {
        i = a;
    }

    return [b, c, d, e, f, g, h, i];
}
