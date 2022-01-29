use macroquad::prelude::{load_texture, FilterMode, Texture2D};

pub struct Textures {
    pub player_idle: Texture2D,
    pub player_walk: Texture2D,
    pub walls: Texture2D,
    pub floors: Texture2D,
}

impl Textures {
    pub async fn get() -> Self {
        let filter = FilterMode::Nearest;

        let player_idle = load_texture("assets/player/idle.png").await.unwrap();
        player_idle.set_filter(filter);
        let player_walk = load_texture("assets/player/walking.png").await.unwrap();
        player_walk.set_filter(filter);
        let walls = load_texture("assets/tiles/walls.png").await.unwrap();
        walls.set_filter(filter);
        let floors = load_texture("assets/tiles/floors.png").await.unwrap();
        floors.set_filter(filter);

        Textures {
            player_idle,
            player_walk,
            walls,
            floors,
        }
    }
}
