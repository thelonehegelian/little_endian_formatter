use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Error;

#[derive(Default, PartialEq, Debug)]
struct Payload {
    kind: u8,
    value: u16,
}
fn main() {
    let payload = Payload::default();
    let encoded = encode(&payload).unwrap();
    println!("{:?}", encoded);
    let decoded = decode(&encoded).unwrap();
    println!("{:?}", decoded);
}

// encode in little endian format
fn encode(payload: &Payload) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![2, 31, 41];
    // write bytes into the payload struct
    bytes.write_u8(payload.kind).unwrap();
    bytes.write_u16::<LittleEndian>(payload.value).unwrap();
    Ok(bytes)
}

fn decode(mut bytes: &[u8]) -> Result<Payload, Error> {
    let payload = Payload {
        kind: bytes.read_u8()?,
        value: bytes.read_u16::<LittleEndian>()?,
    };
    Ok(payload)
}
