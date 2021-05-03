use getopts::Options;
use lopdf::{Document, Object};

use std::env;


fn main() {
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
        opt_matches.free[0]
    };

    let output_pdf = opt_matches.opt_str("output").unwrap_or("-");

    let mut doc = Document::load("./f1040.pdf").unwrap();

    for (_, mut obj) in &mut doc.objects {
        match &mut obj {
            Object::Dictionary(ref mut d) => {
                for (k, v) in d.iter_mut() {
                    let key = std::str::from_utf8(k).unwrap();

                    if key == "DA" {
                        let properties = v.as_str_mut().unwrap();

                        let new_properties = std::str::from_utf8(properties)
                            .unwrap()
                            .replace("HelveticaLTStd-Bold", "CourierNewPSMT");

                        *properties = new_properties.into_bytes();
                    }
                }
            },
            _ => (),
        }
    }

    doc.save("./new.pdf").unwrap();
}
