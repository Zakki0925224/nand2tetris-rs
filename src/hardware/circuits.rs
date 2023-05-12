use super::{calcs::inc16, consts::ZERO16, gates::*, BoolArray16};

#[derive(Clone, Copy)]
pub struct Dff(bool);

impl Dff
{
    pub fn new(value: bool) -> Self { return Self(value); }

    pub fn dff(&mut self, value: bool) -> bool
    {
        let result = self.0;
        self.0 = value;
        return result;
    }

    pub fn get(&self) -> bool { return self.0; }
}

#[derive(Clone, Copy)]
pub struct Bit(Dff);

impl Default for Bit
{
    fn default() -> Self { return Bit::new(false); }
}

impl Bit
{
    pub fn new(value: bool) -> Self { return Self(Dff::new(value)); }

    pub fn bit(&mut self, value: bool, load: bool) -> bool
    {
        let v = mux(self.0 .0, value, load);
        return self.0.dff(v);
    }

    pub fn get(&self) -> bool { return self.0.get(); }
}

#[derive(Clone, Copy)]
pub struct Register(pub [Bit; 16]);
impl Register
{
    pub fn new() -> Self { return Self([Bit::default(); 16]) }

    pub fn register(&mut self, value: BoolArray16, load: bool) -> BoolArray16
    {
        let result = [
            self.0[0].bit(value[0], load),
            self.0[1].bit(value[1], load),
            self.0[2].bit(value[2], load),
            self.0[3].bit(value[3], load),
            self.0[4].bit(value[4], load),
            self.0[5].bit(value[5], load),
            self.0[6].bit(value[6], load),
            self.0[7].bit(value[7], load),
            self.0[8].bit(value[8], load),
            self.0[9].bit(value[9], load),
            self.0[10].bit(value[10], load),
            self.0[11].bit(value[11], load),
            self.0[12].bit(value[12], load),
            self.0[13].bit(value[13], load),
            self.0[14].bit(value[14], load),
            self.0[15].bit(value[15], load),
        ];
        return result;
    }
}

#[derive(Clone, Copy)]
pub struct Ram8([Register; 8]);

impl Ram8
{
    pub fn new() -> Self { return Self([Register::new(); 8]); }

    pub fn ram(&mut self, value: [bool; 16], addr: [bool; 3], load: bool) -> [bool; 16]
    {
        let sel = dmux8way(load, addr);

        return mux8way16(
            self.0[0].register(value, sel[0]),
            self.0[1].register(value, sel[1]),
            self.0[2].register(value, sel[2]),
            self.0[3].register(value, sel[3]),
            self.0[4].register(value, sel[4]),
            self.0[5].register(value, sel[5]),
            self.0[6].register(value, sel[6]),
            self.0[7].register(value, sel[7]),
            addr,
        );
    }
}

#[derive(Clone, Copy)]
pub struct Ram64([Ram8; 8]);

impl Ram64
{
    pub fn new() -> Self { return Self([Ram8::new(); 8]); }

    pub fn ram(&mut self, value: [bool; 16], addr: [bool; 6], load: bool) -> [bool; 16]
    {
        let upper = [addr[0], addr[1], addr[2]];
        let lower = [addr[3], addr[4], addr[5]];

        let sel = dmux8way(load, upper);

        return mux8way16(
            self.0[0].ram(value, lower, sel[0]),
            self.0[1].ram(value, lower, sel[1]),
            self.0[2].ram(value, lower, sel[2]),
            self.0[3].ram(value, lower, sel[3]),
            self.0[4].ram(value, lower, sel[4]),
            self.0[5].ram(value, lower, sel[5]),
            self.0[6].ram(value, lower, sel[6]),
            self.0[7].ram(value, lower, sel[7]),
            upper,
        );
    }
}

#[derive(Clone, Copy)]
pub struct Ram512([Ram64; 8]);

impl Ram512
{
    pub fn new() -> Self { return Self([Ram64::new(); 8]); }

    pub fn ram(&mut self, value: [bool; 16], addr: [bool; 9], load: bool) -> [bool; 16]
    {
        let upper = [addr[0], addr[1], addr[2]];
        let lower = [addr[3], addr[4], addr[5], addr[6], addr[7], addr[8]];

        let sel = dmux8way(load, upper);

        return mux8way16(
            self.0[0].ram(value, lower, sel[0]),
            self.0[1].ram(value, lower, sel[1]),
            self.0[2].ram(value, lower, sel[2]),
            self.0[3].ram(value, lower, sel[3]),
            self.0[4].ram(value, lower, sel[4]),
            self.0[5].ram(value, lower, sel[5]),
            self.0[6].ram(value, lower, sel[6]),
            self.0[7].ram(value, lower, sel[7]),
            upper,
        );
    }
}

#[derive(Clone, Copy)]
pub struct Ram4k([Ram512; 8]);

impl Ram4k
{
    pub fn new() -> Self { return Self([Ram512::new(); 8]); }

    pub fn ram(&mut self, value: [bool; 16], addr: [bool; 12], load: bool) -> [bool; 16]
    {
        let upper = [addr[0], addr[1], addr[2]];
        let lower =
            [addr[3], addr[4], addr[5], addr[6], addr[7], addr[8], addr[9], addr[10], addr[11]];

        let sel = dmux8way(load, upper);

        return mux8way16(
            self.0[0].ram(value, lower, sel[0]),
            self.0[1].ram(value, lower, sel[1]),
            self.0[2].ram(value, lower, sel[2]),
            self.0[3].ram(value, lower, sel[3]),
            self.0[4].ram(value, lower, sel[4]),
            self.0[5].ram(value, lower, sel[5]),
            self.0[6].ram(value, lower, sel[6]),
            self.0[7].ram(value, lower, sel[7]),
            upper,
        );
    }
}

#[derive(Clone, Copy)]
pub struct Ram16k([Ram4k; 4]);

impl Ram16k
{
    pub fn new() -> Self { return Self([Ram4k::new(); 4]); }

    pub fn ram(&mut self, value: [bool; 16], addr: [bool; 14], load: bool) -> [bool; 16]
    {
        let upper = [addr[0], addr[1]];
        let lower = [
            addr[2], addr[3], addr[4], addr[5], addr[6], addr[7], addr[8], addr[9], addr[10],
            addr[11], addr[12], addr[13],
        ];

        let sel = dmux4way(load, upper);

        return mux4way16(
            self.0[0].ram(value, lower, sel[0]),
            self.0[1].ram(value, lower, sel[1]),
            self.0[2].ram(value, lower, sel[2]),
            self.0[3].ram(value, lower, sel[3]),
            upper,
        );
    }
}

pub struct Counter(Register);

impl Counter
{
    pub fn new() -> Self { return Self(Register::new()); }

    pub fn count(&mut self, value: [bool; 16], inc: bool, load: bool, reset: bool) -> [bool; 16]
    {
        let current = self.0.register(ZERO16, false);
        let next = inc16(current);
        let calced = mux8way16(
            current,
            next,
            value,
            value,
            ZERO16,
            ZERO16,
            ZERO16,
            ZERO16,
            [reset, load, inc],
        );

        return self.0.register(calced, true);
    }
}
