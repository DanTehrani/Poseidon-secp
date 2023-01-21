use crate::{k256_consts, Poseidon, PoseidonConstants};
use ff::PrimeField;
use k256::{elliptic_curve::ScalarCore, Scalar};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn poseidon(input_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut input = Vec::new();
    for i in 0..(input_bytes.len() / 32) {
        let val = ScalarCore::from_be_slice(&input_bytes[(i * 32)..(i + 1) * 32]).unwrap();
        input.push(Scalar::from(val));
    }

    let round_constants: Vec<Scalar> = k256_consts::ROUND_CONSTANTS
        .iter()
        .map(|x| Scalar::from_str_vartime(x).unwrap())
        .collect();

    let mds_matrix: Vec<Vec<Scalar>> = k256_consts::MDS_MATRIX
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| Scalar::from_str_vartime(y).unwrap())
                .collect::<Vec<Scalar>>()
        })
        .collect();

    let constants = PoseidonConstants::<Scalar>::new(
        round_constants,
        mds_matrix,
        k256_consts::NUM_FULL_ROUNDS,
        k256_consts::NUM_PARTIAL_ROUNDS,
    );
    let mut poseidon = Poseidon::new(constants);

    let result = poseidon.hash(input);

    Ok(result.to_bytes().to_vec())
}
