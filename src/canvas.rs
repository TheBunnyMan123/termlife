const BRAILLE_BASE: u32 = 0x2800;

pub struct Canvas {
    pub pixels: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize
}
 
pub fn bool_to_braille(
    d1: &bool, d2: &bool,
    d3: &bool, d4: &bool,
    d5: &bool, d6: &bool,
    d7: &bool, d8: &bool
) -> char {
    let mut dots: u8 = 0;

    if d1.clone() { dots |= 1 << 0; }
    if d2.clone() { dots |= 1 << 1; }
    if d3.clone() { dots |= 1 << 2; }
    if d4.clone() { dots |= 1 << 3; }
    if d5.clone() { dots |= 1 << 4; }
    if d6.clone() { dots |= 1 << 5; }
    if d7.clone() { dots |= 1 << 6; }
    if d8.clone() { dots |= 1 << 7; }

    let codepoint = BRAILLE_BASE + dots as u32;
    char::from_u32(codepoint).unwrap_or('\u{2800}')
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut new = Self {
            pixels: vec![],
            width, height
        };

        for _ in 0..width {
            new.pixels.push(vec![false; height]);
        }

        new
    }

    pub fn push(&self) {
        let chars_horizontal = self.width / 2;
        let chars_vertical = self.height / 4;

        let empty_vec = Vec::<bool>::new();

        let mut final_text: String = String::new();

        for y in 0..chars_vertical {
            for x in 0..chars_horizontal {
                let x_pixel = x * 2;
                let y_pixel = y * 4;
                final_text += bool_to_braille(
                    &self.pixels.get(x_pixel).unwrap_or(&empty_vec).get(y_pixel).unwrap_or(&false),
                    &self.pixels.get(x_pixel).unwrap_or(&empty_vec).get(y_pixel + 1).unwrap_or(&false),
                    &self.pixels.get(x_pixel).unwrap_or(&empty_vec).get(y_pixel + 2).unwrap_or(&false),
                    &self.pixels.get(x_pixel + 1).unwrap_or(&empty_vec).get(y_pixel).unwrap_or(&false),
                    &self.pixels.get(x_pixel + 1).unwrap_or(&empty_vec).get(y_pixel + 1).unwrap_or(&false),
                    &self.pixels.get(x_pixel + 1).unwrap_or(&empty_vec).get(y_pixel + 2).unwrap_or(&false),
                    &self.pixels.get(x_pixel).unwrap_or(&empty_vec).get(y_pixel + 3).unwrap_or(&false),
                    &self.pixels.get(x_pixel + 1).unwrap_or(&empty_vec).get(y_pixel + 3).unwrap_or(&false),
                ).to_string().as_str();
            }

            final_text += "\n";
        }

        print!("{}{}", termion::clear::All.to_string(), termion::cursor::Goto::default().to_string());
        println!("{}", final_text);
    }
}

