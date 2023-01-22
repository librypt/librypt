#[cfg(feature = "int")]
pub use librypt_int as int;

#[cfg(feature = "kdf")]
pub mod kdf {
    pub use librypt_kdf::*;

    #[cfg(feature = "hkdf")]
    pub use librypt_kdf_hkdf as hkdf;
}

#[cfg(feature = "mac")]
pub mod mac {
    pub use librypt_mac::*;

    #[cfg(feature = "hmac")]
    pub use librypt_mac_hmac as hmac;
}

#[cfg(feature = "hash")]
pub mod hash {
    pub use librypt_hash::*;

    #[cfg(feature = "md5")]
    pub use librypt_hash_md5 as md5;

    #[cfg(feature = "sha1")]
    pub use librypt_hash_sha1 as sha1;

    #[cfg(feature = "sha2")]
    pub use librypt_hash_sha2 as sha2;

    #[cfg(feature = "sha3")]
    pub use librypt_hash_sha3 as sha3;

    #[cfg(feature = "blake2")]
    pub use librypt_hash_blake2 as blake2;
}

#[cfg(feature = "entropy")]
pub use librypt_entropy as entropy;

#[cfg(feature = "hotp")]
pub use librypt_hotp as hotp;

#[cfg(feature = "totp")]
pub use librypt_totp as totp;
