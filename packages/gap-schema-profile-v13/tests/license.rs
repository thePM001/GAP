// crate: gap-schema-profile-v13 license tests
// GAP-OSS-LIC-P1. Public GAP package license files must be Apache-2.0.

#[test]
fn this_package_license_is_apache_2_0() {
    let toml = include_str!("../Cargo.toml");
    assert!(toml.contains("license = \"Apache-2.0\""));
    assert!(!toml.contains("UNLICENSED"));
    let license = include_str!("../LICENSE");
    assert!(license.contains("Apache License"));
    assert!(license.contains("Version 2.0"));
    assert!(!license.contains("UNLICENSED"));
}

#[test]
fn packages_have_zero_unlicensed_license_files() {
    let self_toml = include_str!("../Cargo.toml");
    let sibling = include_str!("../../gap-closed-wall-deny/Cargo.toml");
    let self_license = include_str!("../LICENSE");
    let sibling_license = include_str!("../../gap-closed-wall-deny/LICENSE");
    assert!(!self_toml.contains("UNLICENSED"));
    assert!(!sibling.contains("UNLICENSED"));
    assert!(!self_license.contains("UNLICENSED"));
    assert!(!sibling_license.contains("UNLICENSED"));
    assert!(self_toml.contains("license = \"Apache-2.0\""));
    assert!(sibling.contains("license = \"Apache-2.0\""));
}
