/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// CRC checksum mismatch (PEC)
    ChecksumMismatch,
    /// Invalid input data
    InvalidInputData,
}

/// Possible tempature output types
pub enum TempType {
    Celsius,
    Fahrenheit,
} 

/// IC marker
pub mod ic {
    /// MLX90614 IC marker
    pub struct Mlx90614;
    /// MLX90615 IC marker
    pub struct Mlx90615;
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address
    Alternative(u8),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}
