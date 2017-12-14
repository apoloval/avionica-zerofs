use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug, PartialEq)]
pub enum Domain { Fsuipc, Lvar }

impl Domain {
    fn decode<R: io::Read>(input: &mut R) -> io::Result<Domain> {
        let i = input.read_u8()?;
        match i {
            0 => Ok(Domain::Fsuipc),
            1 => Ok(Domain::Lvar),
            _ => Err(io::Error::from(io::ErrorKind::InvalidData)),
        }
    }

    pub fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<usize> {
        match self {
            &Domain::Fsuipc => output.write_u8(0)?,
            &Domain::Lvar => output.write_u8(1)?,
        }
        Ok(1)
    }
}

#[derive(Debug, PartialEq)]
pub struct Address(u16);

impl Address {
    fn decode<R: io::Read>(input: &mut R) -> io::Result<Address> {
        input.read_u16::<BigEndian>().map(|n| Address(n))
    }

    pub fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<usize> {
        output.write_u16::<BigEndian>(self.0).map(|_| 2)
    }
}

#[derive(Debug, PartialEq)]
pub struct Value(i32);

impl Value {
    fn decode<R: io::Read>(input: &mut R) -> io::Result<Value> {
        input.read_i32::<BigEndian>().map(|n| Value(n))
    }

    pub fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<usize> {
        output.write_i32::<BigEndian>(self.0).map(|_| 4)
    }
}

#[derive(Debug, PartialEq)]
pub struct Event {
    domain: Domain,
    address: Address,
    value: Value,
}

impl Event {
    pub fn decode<R: io::Read>(input: &mut R) -> io::Result<Event> {
        let domain = Domain::decode(input)?;
        let address = Address::decode(input)?;
        let value = Value::decode(input)?;
        let event = Event { domain, address, value };
        Ok(event)
    }

    pub fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<usize> {
        let mut nbytes = 0;
        nbytes += self.domain.encode(output)?;
        nbytes += self.address.encode(output)?;
        nbytes += self.value.encode(output)?;
        Ok(nbytes)
    }
}

#[cfg(test)]
mod test {
    use std::io;

    use super::*;

    #[test]
    fn test_decode_event() {
        let mut input = io::Cursor::new(vec![0x00, 0x12, 0x34, 0x01, 0x02, 0x03, 0x04]);
        let result = Event::decode(&mut input).unwrap();

        assert_eq!(Domain::Fsuipc, result.domain);
        assert_eq!(Address(0x1234), result.address);
        assert_eq!(Value(0x01020304), result.value);
    }

    #[test]
    fn test_encode_event() {
        let event = Event{
            domain: Domain::Fsuipc,
            address: Address(0x1234),
            value: Value(0x01020304),
        };
        let mut output = Vec::<u8>::new();
        let result = event.encode(&mut output).unwrap();

        assert_eq!(7, result);
        assert_eq!(vec![0x00, 0x12, 0x34, 0x01, 0x02, 0x03, 0x04], output);
    }
}
