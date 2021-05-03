use lopdf::{Document, Object};


fn main() {
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
