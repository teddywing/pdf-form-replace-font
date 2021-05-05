// Copyright (c) 2021  Teddy Wing
//
// This file is part of PDF Form Replace Font.
//
// PDF Form Replace Font is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// PDF Form Replace Font is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with PDF Form Replace Font. If not, see
// <https://www.gnu.org/licenses/>.


use anyhow::{self, Context};
use exitcode;
use getopts::Options;
use lopdf::{Document, Object};

use std::env;
use std::process;


fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("error: {}", e);

            process::exit(exitcode::SOFTWARE);
        },
    };
}

fn run () -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("f", "find", "original font", "");
    opts.optopt("r", "replace", "replacement font", "");
    opts.optopt("o", "output", "output file", "FILE");

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("V", "version", "show the program version");

    let opt_matches = opts.parse(&args[1..])?;

    if opt_matches.opt_present("h") {
        print!(
            "{}",
            opts.usage("usage: pdf-form-replace-font --fill ORIGINAL_FONT --replace REPLACEMENT_FONT [-o FILE] [PDF_FILE]"),
        );

        process::exit(exitcode::USAGE);
    }

    if opt_matches.opt_present("V") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        process::exit(exitcode::OK);
    }

    let input_pdf = if opt_matches.free.is_empty() {
        "-"
    } else {
        &opt_matches.free[0]
    };

    let find = opt_matches.opt_str("find")
        .ok_or(anyhow::anyhow!("required option 'find' missing"))?;
    let replace = opt_matches.opt_str("replace")
        .ok_or(anyhow::anyhow!("required option 'replace' missing"))?;
    let output_pdf = opt_matches.opt_str("output")
        .unwrap_or("-".to_owned());

    let mut doc = if input_pdf == "=" {
        Document::load_from(&mut std::io::stdin())
            .context("failed reading from stdin")?
    } else {
        Document::load(input_pdf)
            .with_context(|| format!("failed to read PDF '{}'", input_pdf))?
    };

    for (_, mut obj) in &mut doc.objects {
        match &mut obj {
            Object::Dictionary(ref mut d) => {
                for (k, v) in d.iter_mut() {
                    let key = std::str::from_utf8(k)
                        .context("unable to convert PDF object key to UTF-8")?;

                    if key == "DA" {
                        let properties = v.as_str_mut()
                            .context("unable to get properties of form field")?;

                        let new_properties = std::str::from_utf8(properties)
                            .context("unable to convert form field properties to UTF-8")?
                            .replace(&find, &replace);

                        *properties = new_properties.into_bytes();
                    }
                }
            },
            _ => (),
        }
    }

    if output_pdf == "-" {
        doc.save_to(&mut std::io::stdout())
            .context("failed writing to stdout")?;
    } else {
        doc.save(&output_pdf)
            .with_context(|| format!("failed to write PDF '{}'", output_pdf))?;
    }

    Ok(())
}
