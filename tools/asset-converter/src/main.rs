//! Conversion of already decoded images and validation of converted manifests.
use std::{error::Error, path::PathBuf};
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let first = args
        .next()
        .ok_or("usage: asset-converter MANIFEST ROOT | texture INPUT OUTPUT.png")?;
    if first == "texture" {
        let input = PathBuf::from(args.next().ok_or("missing INPUT")?);
        let output = PathBuf::from(args.next().ok_or("missing OUTPUT.png")?);
        if output.exists() {
            return Err("refusing to overwrite output".into());
        }
        let decoded = image::open(input)?;
        decoded.save_with_format(output, image::ImageFormat::Png)?;
        println!("Converted decoded image to PNG; Unity extraction is a separate step.");
    } else {
        let manifest = PathBuf::from(first);
        let root = PathBuf::from(args.next().ok_or("missing ROOT")?);
        let data: sr2_assets::Manifest = serde_json::from_reader(std::fs::File::open(manifest)?)?;
        data.validate(&root)?;
        println!(
            "Validated {} converted asset records (not Unity conversion).",
            data.assets.len()
        );
    }
    Ok(())
}
