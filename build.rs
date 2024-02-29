use starknet::core::utils::get_selector_from_name;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::{BufWriter, Write};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // Write to codegen

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut map = phf_codegen::Map::new();

    if let Ok(lines) = read_lines("./wordlist.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let selector = get_selector_from_name(&line);
            if let Ok(selector) = selector {
                map.entry(
                    format!("{:#064x}", selector),
                    format!(r#""{}""#, line).as_str(),
                );
            } else {
                continue;
            }
        }
    }

    write!(
        &mut file,
        "static SELECTORS: phf::Map<&'static str, &'static str> = {}",
        map.build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();
}
