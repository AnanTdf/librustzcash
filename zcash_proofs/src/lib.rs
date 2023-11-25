extern crate bellman;
extern crate blake2_rfc;
extern crate byteorder;
extern crate ehttp;
extern crate ff;
extern crate pairing;
extern crate rand;
extern crate sapling_crypto;
extern crate zcash_primitives;

mod hashreader;
pub mod sapling;

#[cfg(target_arch = "wasm32")]
pub fn download_parameters(
    base_url: &str,
) -> (
    Parameters<Bls12>,
    PreparedVerifyingKey<Bls12>,
    Parameters<Bls12>,
    PreparedVerifyingKey<Bls12>,
) {
    let (output_params, output_vk) = download_params_by_name(base_url, "sapling-output.params");
    let (spend_params, spend_vk) = download_params_by_name(base_url, "sapling-spend.params");
    (spend_params, spend_vk, output_params, output_vk)
}

#[cfg(target_arch = "wasm32")]
fn download_params_by_name(
    baseurl: &str,
    name: &str,
) -> (Parameters<Bls12>, PreparedVerifyingKey<Bls12>) {
    // https://download.z.cash/downloads/sapling-output.params
    let request = Request {
        ..Request::get(format!("{}/{}", baseurl, name))
    };

    let res = ehttp::fetch_async(&request).unwrap();
    let bytes = res.bytes;
    let mut reader = hashreader::HashReader::new(BufReader::with_capacity(1024 * 1024, &bytes[..]));
    let params = Parameters::<Bls12>::read(&mut reader, false)
        .expect("couldn't deserialize Sapling spend parameters file");
    let vk = prepare_verifying_key(&params.vk);
    println!("file hash: {}", reader.into_hash());
    (params, vk)
}

#[test]
#[cfg(target_arch = "wasm32")]
fn test_download() {
    download_parameters("https://download.z.cash/downloads");
}
