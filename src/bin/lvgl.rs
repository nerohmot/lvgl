use std::{path::Path, process};

use clap::{Arg, Command, crate_version, crate_authors, ArgGroup};
// use std::{fs::File, io::{Seek, SeekFrom}};
use lvgl::types::DfmaReader;


fn main() {
    let description = env!("CARGO_PKG_DESCRIPTION");

    let matches = Command::new("lvgl")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(description)
        .arg(Arg::new("dmfa.xlsx")
            .short('d')
            .long("dmfa")
            .required(true)
            .help("Path to the DMFA document in XLSX format."),
        )
        .arg(Arg::new("bosa.xlsx")
            .short('b')
            .long("bosa")
            .help("Path to the BOSA document in XLSX format."),
        )
        .arg(Arg::new("cipal.xlsx")
            .short('c')
            .long("cipal")
            .help("Path to the CIPAL document in XLSX format."),
        )
        .group(ArgGroup::new("exclusive")
            .args(&["bosa.xlsx", "cipal.xlsx"])
            .required(true)
        )
        .get_matches();

        let dmfa = matches.get_one::<String>("dmfa.xlsx").unwrap();
        let bosa = matches.get_one::<String>("bosa.xlsx");
        let cipal = matches.get_one::<String>("cipal.xlsx");
    
        let dfma_reader = DfmaReader::new(&dmfa);

        let dmfa_path = Path::new(&dmfa);

        match is_valid(dmfa_path) {
            Ok(true) => (),
            Ok(false) => {
                eprintln!("DMFA document must be in XLSX format.");
                process::exit(1);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }

        if let Some(bosa) = bosa {
            let bosa_path = Path::new(&bosa);
            match is_valid(bosa_path) {
                Ok(true) => (),
                Ok(false) => {
                    eprintln!("BOSA document must be in XLSX format.");
                    process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
            println!("DMFA-BOSA: {} ↔ {}", dmfa, bosa);
            compare_dmfa_bosa(dmfa_path, bosa_path);
        }
        if let Some(cipal) = cipal {
            let cipal_path = Path::new(&cipal);
            match is_valid(cipal_path) {
                Ok(true) => (),
                Ok(false) => {
                    eprintln!("CIPAL document must be in XLSX format.");
                    process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
            println!("DMFA-CIPAL: {} ↔ {}", dmfa, cipal);
            compare_dmfa_cipal(dmfa_path, cipal_path);
        }
    




    // match matches.subcommand() {
    //     Some(("dfma", _)) => {
    //         let dmfa = matches.value_of("dfma").unwrap();
    //         let bosa = matches.value_of("bosa").unwrap();
    //         println!("DMFA: {}", dmfa);
    //         println!("BOSA: {}", bosa);
    //     }
    //     _ => {
    //         eprintln!("Invalid subcommand. Use --help for more information.");
    //         process::exit(1);
    //     }
    // }
}