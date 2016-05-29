use i2c::I2C;

#[derive(Debug)]
pub struct ReadResult {
    a0: bool,
    a1: bool,
    a2: bool,
    a3: bool,
    a4: bool,
    a5: bool,
    a6: bool,
    a7: bool,
    b0: bool,
    b1: bool,
    b2: bool,
    b3: bool,
    b4: bool,
    b5: bool,
    b6: bool,
    b7: bool,
}

pub enum Side {
    GpioA,
    GpioB,
}

//pub enum Pin {
//    // A0-7, B0-7
//    GpioB0,
//}

pub struct MCP23017 {
    i2c: I2C,
}

pub fn from_i2c(mut i2c: I2C) -> MCP23017 {
    set_chip_to_bank1(&mut i2c);
    init_side(&mut i2c, Side::GpioA);
    init_side(&mut i2c, Side::GpioB);

    return MCP23017 {
        i2c: i2c,
    };
}

fn set_chip_to_bank1(i2c: &mut I2C) {
    // Write 0x84 to register 0x0A. If the chip is in BANK0 mode, this writes to IOCON to set
    // BANK=1 and ODR=1. If already in BANK1 mode, this writes to OLATA. This also keeps the
    // default value of sequential operation (SEQOP)
    i2c.write(b"\x0a\x84");

    // Write 0x00 to 0x0A. The above write guarantees being in BANK1, so this always writes to
    // OLATA, resetting it in case the previous write set something.
    i2c.write(b"\x0a\x00");
}

fn init_side(i2c: &mut I2C, side: Side) {
    let register = match side {
        Side::GpioA => 0x00,
        Side::GpioB => 0x10,
    };
    // IOCON has SEQOP on from above, so this writes the 7 hardcoded bytes starting at the register
    // determined above (e.g. write first byte to register 0x10, second to 0x11, etc). See Table
    // 1-2 in the MCP23017 spec sheet for addresses (IOCON.BANK=1)
    let seq = [
        register,
        0xFF, // IODIRn   = xFF for all inputs
        0xFF, // IPOLn    = xFF to flip inputs (makes connection to ground = 1)
        0xFF, // GPINTENn = xFF use all pins
        0x00, // DEFVALn  = x00 default value something...?
        0x00, // INTCONn  = x00 compare to previous or default
        0x84, // IOCONn   = x84 like above
        0xFF, // GPPUn    = xFF sets resistors for inputs
    ];
    i2c.write(&seq);
}

impl MCP23017 {
    /*
    pub fn getPinValue(&mut self, pin: Pin) -> bool {
        let i = match sideFromPin(pin) {
            Side::GpioA => 0x09,
            Side::GpioB => 0x19,
        };
        self.i2c.write(&[i]);
        // do read things
        return true;
    }
    */

    pub fn read(&mut self) -> ReadResult {
        self.i2c.write(&[BANK1_GPIOA]);
        let side_a = self.i2c.get_byte();
        self.i2c.write(&[BANK1_GPIOB]);
        let side_b = self.i2c.get_byte();
        return ReadResult {
            a0: side_a & (1 << 0) > 0,
            a1: side_a & (1 << 1) > 0,
            a2: side_a & (1 << 2) > 0,
            a3: side_a & (1 << 3) > 0,
            a4: side_a & (1 << 4) > 0,
            a5: side_a & (1 << 5) > 0,
            a6: side_a & (1 << 6) > 0,
            a7: side_a & (1 << 7) > 0,

            b0: side_b & (1 << 0) > 0,
            b1: side_b & (1 << 1) > 0,
            b2: side_b & (1 << 2) > 0,
            b3: side_b & (1 << 3) > 0,
            b4: side_b & (1 << 4) > 0,
            b5: side_b & (1 << 5) > 0,
            b6: side_b & (1 << 6) > 0,
            b7: side_b & (1 << 7) > 0,
        };
    }
}

/*
fn sideFromPin(pin: Pin) -> Side {
    match pin {
        Pin::GpioB0 => Side::GpioB,
    }
}
*/
const BANK0_GPIOA: u8 = 0x12;
const BANK0_GPIOB: u8 = 0x13;

const BANK1_GPIOA: u8 = 0x09;
const BANK1_GPIOB: u8 = 0x19;

