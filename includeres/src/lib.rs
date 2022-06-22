use image::io::Reader as ImageReader;
use proc_macro::TokenStream;

#[proc_macro]
pub fn include_raw_image(item: TokenStream) -> TokenStream {
    let mut item_str = item.to_string();
    item_str.pop();
    item_str.remove(0);

    let img = ImageReader::open(item_str)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let mut array = String::from("[");

    for byte in img.as_bytes() {
        array.push_str(&format!("{},", byte));
    }

    array.push(']');

    array.parse().unwrap()
}
