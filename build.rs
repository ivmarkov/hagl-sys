fn main() -> anyhow::Result<()> {
    pio::bindgen::Runner::from_pio()
        .ok_or(anyhow::Error::msg("Cannot build without cargo-pio"))?
        .run(&["src/bindings.h"], pio::bindgen::Language::C)
}
