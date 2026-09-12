// crate: gap-closed-wall-deny digest tests
// GAP-OSS-HASH-P0. Closed-wall capsule digest must be sha256-structure.

use gap_closed_wall_deny::{
    seal_capsule, structure_digest, CapsuleSeal, DIGEST_HEX_LEN, DIGEST_MODE,
};

fn require(ok: bool) {
    if ok == false {
        std::process::exit(1);
    }
}

#[test]
fn digest_mode_is_sha256_structure() {
    require(DIGEST_MODE == "sha256-structure");
    require(DIGEST_HEX_LEN == 64usize);
}

#[test]
fn nist_empty_and_abc_match_sha256() {
    let mut d = String::new();
    structure_digest(b"", &mut d);
    require(d.as_str() == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    require(d.len() == DIGEST_HEX_LEN);
    structure_digest(b"abc", &mut d);
    require(d.as_str() == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
}

#[test]
fn golden_instruction_object_digest_is_sha256_structure() {
    require(DIGEST_MODE == "sha256-structure");
    let golden = "{\"address\":{\"domain\":\"com.example.live\",\"id\":\"golden-doc\"},\"pattern\":\"p\",\"action\":{\"type\":\"template\",\"content\":\"c\"},\"weight\":1.0,\"composition\":{\"type\":\"atomic\"},\"metadata\":{\"provenance\":\"system.seed\",\"version\":\"1.0.0\",\"stability\":\"experimental\"}}";
    let mut digest = String::new();
    structure_digest(golden.as_bytes(), &mut digest);
    require(digest.as_str() == "423a6918ab6b1518dcedd04d6f40a1a84dc2628202cdf102def74207704cc7ec");
    require(digest.len() == DIGEST_HEX_LEN);
    let mut seal = CapsuleSeal::default();
    seal_capsule(golden, "0", &mut seal);
    require(seal.digest.as_str() == "7c157893865603e66a709092de57819666950c39e3ab5949b13a676f91dd3b26");
    require(seal.digest.len() == DIGEST_HEX_LEN);
    let mut sealed = String::from(golden);
    sealed.push(char::from(10));
    sealed.push_str("nonce=0");
    let mut sealed_digest = String::new();
    structure_digest(sealed.as_bytes(), &mut sealed_digest);
    require(seal.digest == sealed_digest);
}

#[test]
fn zero_fnv_live_digest_on_this_package() {
    let src = include_str!("../src/lib.rs");
    let mut needle = String::from("fnv");
    needle.push_str("1a64");
    require(src.contains(&needle) == false);
    require(src.contains("1099511628211") == false);
    require(src.contains("0xcbf29ce484222325") == false);
    require(src.contains("sha256-structure"));
    require(DIGEST_MODE == "sha256-structure");
}
