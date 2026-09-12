// crate: gap-closed-wall-deny license tests
// GAP-OSS-LIC-P1. Public GAP package license files must be Apache-2.0.

fn require(ok: bool) {
    if ok == false {
        std::process::exit(1);
    }
}

#[test]
fn this_package_license_is_apache_2_0() {
    let toml = include_str!("../Cargo.toml");
    require(toml.contains("license = \"Apache-2.0\""));
    require(toml.contains("UNLICENSED") == false);
    let license = include_str!("../LICENSE");
    require(license.contains("Apache License"));
    require(license.contains("Version 2.0"));
    require(license.contains("UNLICENSED") == false);
}

#[test]
fn packages_have_zero_unlicensed_license_files() {
    let self_toml = include_str!("../Cargo.toml");
    let sibling = include_str!("../../gap-schema-profile-v13/Cargo.toml");
    let self_license = include_str!("../LICENSE");
    let sibling_license = include_str!("../../gap-schema-profile-v13/LICENSE");
    require(self_toml.contains("UNLICENSED") == false);
    require(sibling.contains("UNLICENSED") == false);
    require(self_license.contains("UNLICENSED") == false);
    require(sibling_license.contains("UNLICENSED") == false);
    require(self_toml.contains("license = \"Apache-2.0\""));
    require(sibling.contains("license = \"Apache-2.0\""));
}
