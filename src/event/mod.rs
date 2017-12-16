use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

mod input;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Id(u16);

impl Id {
    fn decode<R: io::Read>(input: &mut R) -> io::Result<Id> {
        input.read_u16::<BigEndian>().map(|n| Id(n))
    }

    pub fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<usize> {
        output.write_u16::<BigEndian>(self.0).map(|_| 2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Value(i32);

impl Value {
    fn decode<R: io::Read>(input: &mut R) -> io::Result<Value> {
        input.read_i32::<BigEndian>().map(|n| Value(n))
    }

    pub fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<usize> {
        output.write_i32::<BigEndian>(self.0).map(|_| 4)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Event {
    pub id: Id,
    pub value: Value,
}

impl Event {
    pub fn decode<R: io::Read>(input: &mut R) -> io::Result<Event> {
        let id = Id::decode(input)?;
        let value = Value::decode(input)?;
        let event = Event { id, value };
        Ok(event)
    }

    pub fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<usize> {
        let mut nbytes = 0;
        nbytes += self.id.encode(output)?;
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
        let mut input = io::Cursor::new(vec![0x12, 0x34, 0x01, 0x02, 0x03, 0x04]);
        let result = Event::decode(&mut input).unwrap();

        assert_eq!(Id(0x1234), result.id);
        assert_eq!(Value(0x01020304), result.value);
    }

    #[test]
    fn test_encode_event() {
        let event = Event{
            id: Id(0x1234),
            value: Value(0x01020304),
        };
        let mut output = Vec::<u8>::new();
        let result = event.encode(&mut output).unwrap();

        assert_eq!(6, result);
        assert_eq!(vec![0x12, 0x34, 0x01, 0x02, 0x03, 0x04], output);
    }
}
