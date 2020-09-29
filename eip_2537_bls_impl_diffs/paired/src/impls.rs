#![allow(dead_code)]
use paired::bls12_381::*;
use groupy::{CurveAffine,CurveProjective};
use fff::{PrimeField,Field};
use num_bigint::BigUint;
use groupy::EncodedPoint;
use paired::PairingCurveAffine;

use paired::Engine;

pub type TestResult<T> = Result<T, Box<dyn std::error::Error + Sync + Send>>;

fn u256_deserialize<R:std::io::Read>(r : &mut R) -> TestResult<Fr> {
    let mut bytes = [0u8;32];
    r.read_exact(&mut bytes[..])?;
    Ok(Fr::from_str(&BigUint::from_bytes_be(&bytes[..]).to_str_radix(10)).unwrap())
}

fn fq_deserialize<R:std::io::Read>(r : &mut R) -> TestResult<Fq> {
    let mut bytes = [0u8;48];
    r.read_exact(&mut bytes[..16])?;   // padding
    r.read_exact(&mut bytes[..])?;   // the value
    Ok(Fq::from_str(&BigUint::from_bytes_be(&bytes[..]).to_str_radix(10)).unwrap())
}

fn fq2_deserialize<R:std::io::Read>(r : &mut R) -> TestResult<Fq2> {
    let c0 = fq_deserialize(r)?;
    let c1 = fq_deserialize(r)?;
    Ok(Fq2 {c0,c1})
}

fn g1_deserialize<R:std::io::Read>(r : &mut R, check_subgroup: bool) -> TestResult<G1> {
    let mut g1 = G1Affine::zero().into_uncompressed();
    let mut padding = [0u8;16];
    r.read_exact(&mut padding[..])?; // zeros
    r.read_exact(&mut g1.as_mut()[..48])?;   // the value
    r.read_exact(&mut padding[..])?; // zeros
    r.read_exact(&mut g1.as_mut()[48..96])?;   // the value
    if check_subgroup {
        Ok(G1::from(g1.into_affine()?))
    } else {
        Ok(G1::from(g1.into_affine_unchecked()?))
    }
}

fn g2_deserialize<R:std::io::Read>(r : &mut R, check_subgroup: bool) -> TestResult<G2> {
    let mut g2 = G2Affine::zero().into_uncompressed();
    let mut padding = [0u8;16];
    let g2_buff = g2.as_mut();
    
    r.read_exact(&mut padding[..])?;
    r.read_exact(&mut g2_buff[1*48..2*48])?;

    r.read_exact(&mut padding[..])?;
    r.read_exact(&mut g2_buff[0*48..1*48])?;

    r.read_exact(&mut padding[..])?;
    r.read_exact(&mut g2_buff[3*48..4*48])?;

    r.read_exact(&mut padding[..])?;
    r.read_exact(&mut g2_buff[2*48..3*48])?;

    if check_subgroup {
        Ok(G2::from(g2.into_affine()?))
    } else {
        Ok(G2::from(g2.into_affine_unchecked()?))
    }
}

pub fn test_bls381_fp_to_g1(input: &[u8], output: &[u8]) -> TestResult<bool>{

    let mut reader = input;
    let fq = fq_deserialize(&mut reader)?;

    let mut reader = output;
    let expected = g1_deserialize(&mut reader,false)?;

    let mut result = G1::osswu_map(&fq);
    result.isogeny_map();
    result.clear_h();

    Ok(result == expected)
}

pub fn test_bls381_fp2_to_g2(input: &[u8], output: &[u8]) -> TestResult<bool>{

    let mut reader = input;
    let fq2 = fq2_deserialize(&mut reader)?;

    let mut reader = output;
    let expected = g2_deserialize(&mut reader,false)?;

    let mut result = G2::osswu_map(&fq2);
    result.isogeny_map();
    result.clear_h();

    Ok(result == expected)
}

pub fn test_bls381_g1_add(input: &[u8], output: &[u8]) -> TestResult<bool>{

    let mut reader = input;
    let mut lhs = g1_deserialize(&mut reader,false)?;
    let rhs = g1_deserialize(&mut reader,false)?;

    let mut reader = output;
    let expected = g1_deserialize(&mut reader,false)?;

    lhs.add_assign(&rhs);
    Ok(lhs == expected)
}

pub fn test_bls381_g1_mul(input: &[u8], output: &[u8]) -> TestResult<bool> {

    let mut reader = input;
    let mut lhs = g1_deserialize(&mut reader,false)?;
    let rhs = u256_deserialize(&mut reader)?;

    let mut reader = output;
    let expected = g1_deserialize(&mut reader, false)?;

    lhs.mul_assign(rhs);
    Ok(lhs == expected)
}    

pub fn test_bls381_g2_add(input: &[u8], output: &[u8]) -> TestResult<bool> {
    let mut reader = input;
    let mut lhs = g2_deserialize(&mut reader,false)?;
    let rhs = g2_deserialize(&mut reader,false)?;

    let mut reader = output;
    let expected = g2_deserialize(&mut reader,false)?;

    lhs.add_assign(&rhs);
    Ok(lhs == expected)
}

pub fn test_bls381_g2_mul(input: &[u8], output: &[u8]) -> TestResult<bool>{
    let mut reader = input;
    let mut lhs = g2_deserialize(&mut reader,false)?;
    let rhs = u256_deserialize(&mut reader)?;

    let mut reader = output;
    let expected = g2_deserialize(&mut reader,false)?;

    lhs.mul_assign(rhs);
    Ok(lhs == expected)
}

pub fn test_bls381_pairing(input: &[u8], output: &[u8]) -> TestResult<bool>{
    let mut reader = input;

    let mut p = Vec::new();

    while reader.len() > 0 {
        let g1 = g1_deserialize(&mut reader,true)?.into_affine();
        let g2 = g2_deserialize(&mut reader,true)?.into_affine();

        p.push((g1.prepare(),g2.prepare()));
    }

    let refs : Vec<(&G1Prepared,&G2Prepared)>= p.iter().map(|(g1,g2)| (g1,g2)).collect::<Vec<_>>();
    let pairing = Bls12::final_exponentiation(&Bls12::miller_loop(&refs)).unwrap();
    
    Ok(output[31] == if pairing == Fq12::one() { 1 } else { 0 })
}
