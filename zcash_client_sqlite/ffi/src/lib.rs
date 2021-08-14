use zcash_client_backend::{
    encoding::{
        decode_extended_spending_key, encode_extended_full_viewing_key,
        encode_extended_spending_key,
    },
    keys::spending_key,
};
use zcash_primitives::{
    consensus::{Network, Parameters},
    zip32::ExtendedFullViewingKey,
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Invalid encoding of a Sapling extended spending key")]
    InvalidExtSk,
    #[error("Key is for the wrong network")]
    WrongNetwork,
}

fn derive_spending_keys(seed: &[u8], network: Network, accounts: u32) -> Vec<String> {
    (0..accounts)
        .map(|account| {
            encode_extended_spending_key(
                network.hrp_sapling_extended_spending_key(),
                &spending_key(seed, network.coin_type(), account),
            )
        })
        .collect()
}

fn derive_extfvk(network: Network, extsk: &str) -> Result<String, Error> {
    match decode_extended_spending_key(network.hrp_sapling_extended_spending_key(), extsk) {
        Ok(Some(extsk)) => Ok(encode_extended_full_viewing_key(
            network.hrp_sapling_extended_full_viewing_key(),
            &ExtendedFullViewingKey::from(&extsk),
        )),
        Ok(None) => Err(Error::WrongNetwork),
        Err(_) => Err(Error::InvalidExtSk),
    }
}

uniffi_macros::include_scaffolding!("zcash_client_sqlite");
