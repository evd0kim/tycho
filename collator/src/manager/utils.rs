#[cfg(not(feature = "gost"))]
use everscale_crypto::ed25519::{KeyPair, PublicKey};
#[cfg(feature = "gost")]
use everscale_crypto::gost256::{KeyPair, PublicKey};
use everscale_types::models::ValidatorDescription;
use tycho_util::FastHashMap;

#[cfg(feature = "gost")]
pub fn find_us_in_collators_set(
    keypair: &KeyPair,
    set: &FastHashMap<[u8; 32], ValidatorDescription>,
) -> Option<PublicKey> {
    let local_pubkey = keypair.public_key;
    if set.contains_key(local_pubkey.as_bytes()) {
        Some(local_pubkey)
    } else {
        None
    }
}

#[cfg(not(feature = "gost"))]
pub fn find_us_in_collators_set(
    keypair: &KeyPair,
    set: &FastHashMap<[u8; 32], ValidatorDescription>,
) -> Option<PublicKey> {
    let local_pubkey = keypair.public_key;
    if set.contains_key(local_pubkey.as_bytes()) {
        Some(local_pubkey)
    } else {
        None
    }
}
