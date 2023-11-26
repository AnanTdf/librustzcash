extern crate bellman;
extern crate blake2_rfc;
extern crate byteorder;
extern crate ff;
extern crate pairing;
extern crate rand;
extern crate sapling_crypto;
extern crate zcash_primitives;

use std::io::BufReader;

use bellman::groth16::{prepare_verifying_key, Parameters, PreparedVerifyingKey, VerifyingKey};
use pairing::bls12_381::Bls12;

mod downloadreader;
mod hashreader;
pub mod sapling;

// #[cfg(target_arch = "wasm32")]
pub fn download_parameters(
    base_url: &str,
) -> (
    Parameters<Bls12>,
    PreparedVerifyingKey<Bls12>,
    Parameters<Bls12>,
    PreparedVerifyingKey<Bls12>,
) {
    let (output_params, output_vk) =
        download_params_by_name(base_url, "sapling-output.params").unwrap();
    let (spend_params, spend_vk) = download_params_by_name(base_url, "sapling-spend.params").unwrap();
    (spend_params, spend_vk, output_params, output_vk)
}

fn download_params_by_name(
    baseurl: &str,
    name: &str,
) -> Result<(Parameters<Bls12>, PreparedVerifyingKey<Bls12>), minreq::Error> {
    use downloadreader::ResponseLazyReader;

    // https://download.z.cash/downloads/sapling-output.params
    let url = format!("{}/{}", baseurl, name);
    let req = minreq::get(url);
    let res = ResponseLazyReader::from(req);

    let downloaded = BufReader::with_capacity(1024 * 1024, res);
    let mut hash_reader = hashreader::HashReader::new(downloaded);
    let params = Parameters::<Bls12>::read(&mut hash_reader, false)
        .expect("couldn't deserialize Sapling spend parameters file");
    let vk = prepare_verifying_key(&params.vk);
    println!("file hash: {}", hash_reader.into_hash());
    Ok((params, vk))
}

#[test]
fn test_download() {
    download_parameters("https://download.z.cash/downloads");
}
