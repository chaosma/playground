use std::{
      io::{Read, Result},
};
use subtle::CtOption;
use num_bigint::BigUint;
use pairing_bn256::bn256::Fq;
use pairing_bn256::arithmetic::FieldExt;

pub fn read_bigint<R: Read>(mut reader: R) -> Result<BigUint> {
      let mut bytes = vec![0u8; 32];
      reader.read_exact(&mut bytes[..])?;
      Ok(BigUint::from_bytes_le(&bytes[..]))
  }

pub fn bn_to_field<F: FieldExt>(bn: &BigUint) -> F {
      let mut bytes = bn.to_bytes_le();
      bytes.resize(32, 0);
      let mut bytes = &bytes[..];
      F::read(&mut bytes).unwrap()
}

pub fn bn_to_field_1(bn: &BigUint) -> Fq {
    let mut bytes = bn.to_bytes_le();
    bytes.resize(32, 0);
    let bytes = &bytes[..];
    let mut by =  [0u8; 32];
    by.copy_from_slice(bytes);
    CtOption::from(Fq::from_bytes(&by)).unwrap()
}

fn deserialize_field<R: Read>(reader: &mut R) -> Result<Fq> {
      let bigint = read_bigint(reader)?;
      Ok(bn_to_field::<Fq>(&bigint))
}

fn deserialize_field_1<R: Read>(reader: &mut R) -> Result<Fq> {
      let bigint = read_bigint(reader)?;
      Ok(bn_to_field_1(&bigint))
}

fn main() {
    let fq_buf = vec![
              157, 13, 143, 197, 141, 67, 93, 211, 61, 11, 199, 245, 40, 235, 120, 10, 44, 70, 121,
              120, 111, 163, 110, 102, 47, 223, 7, 154, 193, 119, 10, 14,
          ];
    let fq = deserialize_field(&mut &fq_buf[..]).unwrap();
    let fq1 = deserialize_field_1(&mut &fq_buf[..]).unwrap();
    println!("fq={:?}", fq);
    println!("fq1={:?}", fq1);
    let fq_one = Fq::one();
    println!("should equal to: {:?}", fq_one);
}
