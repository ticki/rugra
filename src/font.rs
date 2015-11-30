use sfml::graphics::Font as SfmlFont;

pub struct Font {
    font: SfmlFont,
}

impl Font {
    pub fn load(s: &str) -> Font {
        Font {
            font: SfmlFont::new_from_file(s).expect("Couldn't load font"),
        }
    }

    pub fn to_sfml_font(&self) -> &SfmlFont {
        &self.font
    }
}
