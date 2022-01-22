use macroquad::prelude::{load_texture, FilterMode, Texture2D};

pub struct Textures {
    pub idle: Texture2D,
}

impl Textures {
    pub async fn get() -> Self {
        let filter = FilterMode::Nearest;

        let idle = load_texture("assets/idle.png").await.unwrap();
        idle.set_filter(filter);

        Textures { idle }
    }
}
