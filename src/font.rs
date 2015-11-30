use sfml::graphics::Font as SfmlFont;

pub struct Font {
    font: SfmlFont,
}

impl Font {
    /// Load the font relative to the current working directory
    pub fn load(s: &str) -> Font {
        Font {
            font: SfmlFont::new_from_file(s).expect("Couldn't load font"),
        }
    }

    /// Convert to an SFML (backend) font
    pub fn to_sfml_font(&self) -> &SfmlFont {
        &self.font
    }
}
