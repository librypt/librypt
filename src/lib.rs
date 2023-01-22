#[cfg(feature = "int")]
pub mod int {
    pub use librypt_int::*;
}

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
    pub mod hmac {
        pub use librypt_mac_hmac::*;
    }
}

#[cfg(feature = "hash")]
pub mod hash {
    pub use librypt_hash::*;

    #[cfg(feature = "md5")]
    pub mod md5 {
        pub use librypt_hash_md5::*;
    }

    #[cfg(feature = "sha1")]
    pub mod sha1 {
        pub use librypt_hash_sha1::*;
    }

    #[cfg(feature = "sha2")]
    pub mod sha2 {
        pub use librypt_hash_sha2::*;
    }

    #[cfg(feature = "sha3")]
    pub mod sha3 {
        pub use librypt_hash_sha3::*;
    }

    #[cfg(feature = "blake2")]
    pub mod blake2 {
        pub use librypt_hash_blake2::*;
    }
}

#[cfg(feature = "entropy")]
pub mod entropy {
    pub use librypt_entropy::*;
}

#[cfg(feature = "hotp")]
pub mod hotp {
    pub use librypt_hotp::*;
}

#[cfg(feature = "totp")]
pub mod totp {
    pub use librypt_totp::*;
}
