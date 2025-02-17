use crate::{
    ic,
    register_access::mlx90615::{self, Register, DEV_ADDR},
    Error, Mlx9061x, SlaveAddr, types::TempType, 
};
use core::marker::PhantomData;
use embedded_hal::{
    blocking::{delay::DelayMs, i2c},
    digital::v2::OutputPin,
};

impl<E, I2C> Mlx9061x<I2C, ic::Mlx90615>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Create new instance of the MLX90615 device.
    ///
    /// The slave address must match the address stored in the device EEPROM.
    /// To change it you need to connect first and then change it with `set_address()`.
    /// An invalid alternative slave address will return `Error::InvalidInputData`.
    ///
    /// When writing to the EEPROM waiting a certain amount of time is necessary.
    /// This delay is configured through the `eeprom_write_delay_ms` parameter
    /// in milliseconds.
    pub fn new_mlx90615(
        i2c: I2C,
        address: SlaveAddr,
        eeprom_write_delay_ms: u8,
    ) -> Result<Self, Error<E>> {
        let address = Self::get_address(address, DEV_ADDR)?;
        Ok(Mlx9061x {
            i2c,
            eeprom_write_delay_ms,
            address,
            _ic: PhantomData,
        })
    }
}

impl<E, I2C> Mlx9061x<I2C, ic::Mlx90615>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Read the ambient temperature in celsius degrees
    pub fn ambient_temperature(&mut self, temp_type: Option<TempType>) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TA)?;
        if let Some(tt) = temp_type {
            match tt {
                TempType::Celsius => Ok(
                    f32::from(t) * 0.02 - 273.15
                ),
                TempType::Fahrenheit => Ok(
                    (f32::from(t) * 0.02 - 273.15) * 1.80 + 32.00
                ),
            }
        } else {
            Ok(f32::from(t) * 0.02 - 273.15)
        }
    }

    /// Read the object temperature in celsius degrees
    pub fn object_temperature(&mut self, temp_type: Option<TempType>) -> Result<f32, Error<E>> {
        let t = self.read_u16(Register::TA)?;
        if let Some(tt) = temp_type {
            match tt {
                TempType::Celsius => Ok(
                    f32::from(t) * 0.02 - 273.15
                ),
                TempType::Fahrenheit => Ok(
                    (f32::from(t) * 0.02 - 273.15) * 1.80 + 32.00
                ),
            }
        } else {
            Ok(f32::from(t) * 0.02 - 273.15)
        }
    }

    /// Read the raw IR data
    pub fn raw_ir(&mut self) -> Result<u16, Error<E>> {
        self.read_u16(Register::RAW_IR)
    }

    /// Get emissivity epsilon
    pub fn emissivity(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_u16(Register::EMISSIVITY)?;
        Ok(f32::from(raw) / 16384.0)
    }

    /// Set emissivity epsilon [0.0-1.0]
    ///
    /// Wrong values will return `Error::InvalidInputData`.
    pub fn set_emissivity<D: DelayMs<u8>>(
        &mut self,
        epsilon: f32,
        delay: &mut D,
    ) -> Result<(), Error<E>> {
        if epsilon < 0.0 || epsilon > 1.0 {
            return Err(Error::InvalidInputData);
        }
        let eps = (epsilon * 16384.0 + 0.5) as u16;
        self.write_u16_eeprom(Register::EMISSIVITY, eps as u16, delay)
    }

    /// Get the device ID
    pub fn device_id(&mut self) -> Result<u32, Error<E>> {
        let id0 = self.read_u16(Register::ID0)?;
        let id1 = self.read_u16(Register::ID0 + 1)?;
        Ok((u32::from(id0) << 16) | u32::from(id1))
    }
}

/// Wake device from sleep mode.
///
/// Note that this includes a 39ms delay.
pub fn wake_mlx90615<E, P: OutputPin<Error = E>, D: DelayMs<u8>>(
    scl: &mut P,
    delay: &mut D,
) -> Result<(), E> {
    scl.set_low()?;
    delay.delay_ms(mlx90615::WAKE_DELAY_MS);
    scl.set_high()
}
