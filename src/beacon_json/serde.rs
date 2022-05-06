use serde::Serialize;
use serde_json::{ser::Formatter, Error, Serializer};
use std::io::{self, Write};
// use serde::de;

// pub(crate) fn from_slice<'a, T>(v: &'a [u8]) -> Result<T, Error>
// where
//     T: de::Deserialize<'a>,
// {
//     // let mut de = serde::Deserializer::from_slice(v);
//     // let value = T::deserialize(Wrapper(&mut de))?;
//     // de.end().unwrap();
//     // Ok(value)
// }

struct BeaconFormatter;

impl Formatter for BeaconFormatter {
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        write!(writer, "\"{}\"", value)
    }

    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: ?Sized + Write,
    {
        write!(writer, "\"{}\"", value)
    }
}

pub(crate) fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    let formatter = BeaconFormatter;
    let mut ser = Serializer::with_formatter(&mut writer, formatter);
    value.serialize(&mut ser)?;
    Ok(writer)
}
