// crate: gap-schema-profile-v13 license tests
// GAP-OSS-LIC-P1. Public GAP package license files must be Apache-2.0.

#[test]
fn this_package_license_is_apache_2_0() {
    let toml = include_str!("../Cargo.toml");
    assert_eq ! (toml.contains("license = \"Apache-2.0\""), true);
    assert_eq ! (toml.contains("UNLICENSED"), false);
    let license = include_str!("../LICENSE");
    assert_eq ! (license.contains("Apache License"), true);
    assert_eq ! (license.contains("Version 2.0"), true);
    assert_eq ! (license.contains("UNLICENSED"), false);
}

#[test]
fn packages_have_zero_unlicensed_license_files() {
    let self_toml = include_str!("../Cargo.toml");
    let sibling = include_str!("../../gap-closed-wall-deny/Cargo.toml");
    let self_license = include_str!("../LICENSE");
    let sibling_license = include_str!("../../gap-closed-wall-deny/LICENSE");
    assert_eq ! (self_toml.contains("UNLICENSED"), false);
    assert_eq ! (sibling.contains("UNLICENSED"), false);
    assert_eq ! (self_license.contains("UNLICENSED"), false);
    assert_eq ! (sibling_license.contains("UNLICENSED"), false);
    assert_eq ! (self_toml.contains("license = \"Apache-2.0\""), true);
    assert_eq ! (sibling.contains("license = \"Apache-2.0\""), true);
}
