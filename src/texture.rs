use sfml::graphics::Texture as SfmlTexture;

/// Texture
pub struct Texture {
    texture: SfmlTexture,
}

impl Texture {
    /// Load a texture from a file
    pub fn load(f: &str) -> Texture {
        Texture {
            texture: SfmlTexture::new_from_file(f).expect("Couldn't load sprite"),
        }

    }

    /// Get a empty texture
    pub fn empty() -> Texture {
        Texture {
            texture: SfmlTexture::new(32, 32).unwrap(),
        }
    }

    /// Convert to an SFML (backend) texture
    pub fn to_sfml_texture(&self) -> &SfmlTexture {
        &self.texture
    }
}
