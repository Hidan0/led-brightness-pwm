// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `toml_cfg` reads cfg.toml at compile time: rebuild when it changes
    println!("cargo:rerun-if-changed=cfg.toml");
    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")?;
    Ok(())
}
