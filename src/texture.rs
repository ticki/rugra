use sfml::graphics::Texture as SfmlTexture;

pub struct Texture {
    texture: SfmlTexture,
}

impl Texture {
    pub fn load(f: &str) -> Texture {
        Texture {
            texture: SfmlTexture::new_from_file(f).expect("Couldn't load sprite"),
        }

    }

    pub fn empty() -> Texture {
        Texture {
            texture: SfmlTexture::new(32, 32).unwrap(),
        }
    }

    pub fn to_sfml_texture(&self) -> &SfmlTexture {
        &self.texture
    }
}
