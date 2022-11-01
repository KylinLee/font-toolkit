use fontkit::Error;
use rustbitmap::{BitMap, Rgba};
use std::borrow::Borrow;
use std::fs;
use std::io::Read;

#[test]
pub fn test_font_loading() -> Result<(), Error> {
    let mut buf = vec![];
    let mut f = fs::File::open("examples/OpenSans-Italic.ttf")?;
    f.read_to_end(&mut buf)?;
    let mut fontkit = fontkit::FontKit::new();
    let _ = fontkit.add_font_from_buffer(buf)?;
    Ok(())
}

#[test]
pub fn test_font_stroke_bitmap() -> Result<(), Error> {
    let c = '默';
    let mut buf = vec![];
    let mut f = fs::File::open("examples/NotoSansCJKsc-Bold.otf")?;
    f.read_to_end(&mut buf)?;

    let mut fontkit = fontkit::FontKit::new();
    let font_keys = fontkit.add_font_from_buffer(buf)?;
    let font_key = font_keys[0].borrow();
    let font = fontkit.query(font_key).unwrap();

    let glyph_bitmap = font.bitmap(c, 1000.0, 100.0).unwrap();
    let (stroke_bitmap, width) = glyph_bitmap.stroke_bitmap.unwrap();
    let height = (stroke_bitmap.len() / width as usize) as u32;

    let mut bitmap = vec![];

    stroke_bitmap.into_iter().for_each(|alpha| {
        if alpha == 0 {
            bitmap.push(Rgba::white())
        } else {
            bitmap.push(Rgba::black())
        }
    });

    let bitmap = BitMap::create(width, height, bitmap).unwrap();
    bitmap.save_as("./默.bmp").unwrap();

    Ok(())
}
