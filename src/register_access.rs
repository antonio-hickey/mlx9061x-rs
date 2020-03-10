use crate::{crc8, Error, Mlx9061x};
use embedded_hal::blocking::i2c;

pub const DEV_ADDR: u8 = 0x5A;

pub mod mlx90614 {
    pub struct Register {}
    impl Register {
        pub const RAW_IR1: u8 = 0x04;
        pub const RAW_IR2: u8 = 0x05;
        pub const TA: u8 = 0x06;
        pub const TOBJ1: u8 = 0x07;
        pub const TOBJ2: u8 = 0x08;
    }
}

impl<E, I2C, IC> Mlx9061x<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E>,
{
    pub(crate) fn read_u16(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 3];
        self.i2c
            .write_read(DEV_ADDR, &[register], &mut data)
            .map_err(Error::I2C)?;
        let pec = data[2];
        Self::check_pec(
            &[
                DEV_ADDR << 1,
                register,
                (DEV_ADDR << 1) + 1,
                data[0],
                data[1],
            ],
            pec,
        )?;
        Ok(u16::from(data[0]) | (u16::from(data[1]) << 8))
    }

    fn check_pec(data: &[u8], expected: u8) -> Result<(), Error<E>> {
        if crc8(data) != expected {
            Err(Error::ChecksumMismatch)
        } else {
            Ok(())
        }
    }
}
