use std::{
      io::{Read, Result},
};
use subtle::CtOption;
use num_bigint::BigUint;
use pairing_bn256::bn256::Fq;
use pairing_bn256::arithmetic::FieldExt;
use pairing_bn256::group::ff::PrimeField;


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
    let mut by =  [0u64; 4];
    for i in 0..4 {
        let mut tmp: [u8;8] = [0u8;8];
        tmp.copy_from_slice(&bytes[i*8..i*8+8]);
        by[i] = u64::from_le_bytes(tmp);
    }
    Fq::from_raw(by)
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
    let mut by = [0u8;32];
    by.copy_from_slice(&fq_buf[..]);
    let fq2 = Fq::from_bytes(&by);
    let fqr =  Fq::from_raw([
        0xd35d438dc58f0d9d,
        0x0a78eb28f5c70b3d,
        0x666ea36f7879462c,
        0x0e0a77c19a07df2f,
    ]);
    println!("fq={:?}", fq);
    println!("fq1={:?}", fq1);
    println!("fq2={:?}", fq2.unwrap());
    println!("R={:?}", fqr);
    //let fq_one = Fq::one();
    let fq_one = Fq::from_raw([0x01,0x00,0x00,0x00]);
    println!("should equal to: {:?}", fq_one);
}
