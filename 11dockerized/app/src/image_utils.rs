use image::{ImageOutputFormat, open};
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};


pub fn image_to_base64(path: String) -> String {
    let img = open(path).unwrap();
    let mut image_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = general_purpose::STANDARD_NO_PAD.encode(image_data);

    return format!("data:image/png;base64,{}", res_base64);
}