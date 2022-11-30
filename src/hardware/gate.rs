pub fn nand(a: bool, b: bool) -> bool { return !(a && b); }

pub fn not(a: bool) -> bool { return nand(true, a); }

pub fn and(a: bool, b: bool) -> bool { return not(nand(a, b)); }

pub fn or(a: bool, b: bool) -> bool { return nand(not(a), not(b)); }

pub fn xor(a: bool, b: bool) -> bool
{
    let nandab = nand(a, b);
    return nand(nand(a, nandab), nand(b, nandab));
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool { return or(and(b, sel), and(a, not(sel))); }

pub fn dmux(a: bool, sel: bool) -> (bool, bool)
{
    let nandasel = nand(a, sel);
    let nandanandasel = nand(a, nandasel);

    return (nand(nandanandasel, nandanandasel), nand(nandasel, nandasel));
}
