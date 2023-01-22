use crate::{k256_consts, Poseidon, PoseidonConstants};
use ff::PrimeField;
use secq256k1::field::field_secq::FieldElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(dead_code)]
pub fn poseidon(input_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut input = Vec::new();
    for i in 0..(input_bytes.len() / 32) {
        let f: [u8; 32] = input_bytes[(i * 32)..(i + 1) * 32].try_into().unwrap();
        let val = FieldElement::from_bytes(&f).unwrap();
        input.push(FieldElement::from(val));
    }

    let round_constants: Vec<FieldElement> = k256_consts::ROUND_CONSTANTS
        .iter()
        .map(|x| FieldElement::from_str_vartime(x).unwrap())
        .collect();

    let mds_matrix: Vec<Vec<FieldElement>> = k256_consts::MDS_MATRIX
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| FieldElement::from_str_vartime(y).unwrap())
                .collect::<Vec<FieldElement>>()
        })
        .collect();

    let constants = PoseidonConstants::<FieldElement>::new(
        round_constants,
        mds_matrix,
        k256_consts::NUM_FULL_ROUNDS,
        k256_consts::NUM_PARTIAL_ROUNDS,
    );
    let mut poseidon = Poseidon::new(constants);

    let result = poseidon.hash(input);

    Ok(result.to_bytes().to_vec())
}
