use getopts::Options;
use lopdf::{Document, Object};

use std::env;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.reqopt("f", "find", "original font", "");
    opts.reqopt("r", "replace", "replacement font", "");
    opts.optopt("o", "output", "output file", "FILE");

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("V", "version", "show the program version");

    let opt_matches = opts.parse(&args[1..])?;

    let input_pdf = if opt_matches.free.is_empty() {
        "-"
    } else {
        &opt_matches.free[0]
    };

    let find = opt_matches.opt_str("find").unwrap();
    let replace = opt_matches.opt_str("replace").unwrap();
    let output_pdf = opt_matches.opt_str("output").unwrap_or("-".to_owned());

    let mut doc = if input_pdf == "=" {
        Document::load_from(&mut std::io::stdin()).unwrap()
    } else {
        Document::load(input_pdf).unwrap()
    };

    for (_, mut obj) in &mut doc.objects {
        match &mut obj {
            Object::Dictionary(ref mut d) => {
                for (k, v) in d.iter_mut() {
                    let key = std::str::from_utf8(k).unwrap();

                    if key == "DA" {
                        let properties = v.as_str_mut().unwrap();

                        let new_properties = std::str::from_utf8(properties)
                            .unwrap()
                            .replace(&find, &replace);

                        *properties = new_properties.into_bytes();
                    }
                }
            },
            _ => (),
        }
    }

    if output_pdf == "-" {
        doc.save_to(&mut std::io::stdout()).unwrap();
    } else {
        doc.save(output_pdf).unwrap();
    }

    Ok(())
}
