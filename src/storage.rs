use hal::blocking::i2c::{Write, WriteRead};
use {addr_size, page_size, Eeprom24x, Error, eeprom24x::MultiSizeAddr};
use hal::storage::{SingleWrite,SingleRead,MultiRead,MultiWrite,Address};

impl<I2C, E, PS, AS> SingleWrite<u8,u32> for Eeprom24x<I2C, PS, AS> 
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
    AS: MultiSizeAddr,
{
    type Error = Error<E>;
    fn try_write(&mut self, address: Address<u32>, word: u8) -> nb::Result<(), Self::Error> {
        self.write_byte(address.0, word).map(|_| ()).map_err(|e| nb::Error::Other(e))
    }
}

impl<I2C, E, PS, AS> MultiWrite<u8,u32> for Eeprom24x<I2C, PS, AS> 
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
    AS: MultiSizeAddr,
{
    type Error = Error<E>;
    fn try_write_slice(&mut self, address: Address<u32>, buf: &[u8]) -> nb::Result<(), Self::Error> {
        for item in buf {
            self.write_byte(address.0, *item).map_err(|e| nb::Error::Other(e))?;
        }

        Ok(())        
    }
}

impl<I2C, E, PS, AS> SingleRead<u8,u32> for Eeprom24x<I2C, PS, AS> 
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
    AS: MultiSizeAddr,
{
    type Error = Error<E>;
    fn try_read(&mut self, address: Address<u32>) -> nb::Result<u8, Self::Error> {
        self.read_byte(address.0).map(|d| d).map_err(|e| nb::Error::Other(e))
    }
}

impl<I2C, E, PS, AS> MultiRead<u8,u32> for Eeprom24x<I2C, PS, AS> 
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
    AS: MultiSizeAddr,
{
    type Error = Error<E>;
    fn try_read_slice(&mut self, address: Address<u32>,  buf: &mut [u8]) -> nb::Result<(), Self::Error> {
        self.read_data(address.0,buf).map(|_| ()).map_err(|e| nb::Error::Other(e))
    }
}