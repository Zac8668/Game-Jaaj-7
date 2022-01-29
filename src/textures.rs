use macroquad::prelude::{load_texture, FilterMode, Texture2D};

pub struct Textures {
    pub player_idle: Texture2D,
    pub player_walk: Texture2D,
    pub walls: Texture2D,
    pub floors: Texture2D,
    pub water_1: Texture2D,
    pub water_2: Texture2D,
    pub menu_art: Texture2D,
    pub exit: Texture2D,
    pub new_game: Texture2D,
    pub quadrinho_1: Texture2D,
    pub quadrinho_2: Texture2D,
    pub quadrinho_3: Texture2D,
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
        let water_1 = load_texture("assets/tiles/water_sheet.png").await.unwrap();
        water_1.set_filter(filter);
        let water_2 = load_texture("assets/tiles/water_sprite_sheet.png")
            .await
            .unwrap();
        water_2.set_filter(filter);

        let menu_art = load_texture("assets/menu/menu_art.png").await.unwrap();
        let exit = load_texture("assets/menu/exit.png").await.unwrap();
        let new_game = load_texture("assets/menu/new_game.png").await.unwrap();

        let quadrinho_1 = load_texture("assets/menu/quadrinho_1.png").await.unwrap();
        let quadrinho_2 = load_texture("assets/menu/quadrinho_2.png").await.unwrap();
        let quadrinho_3 = load_texture("assets/menu/quadrinho_3.png").await.unwrap();

        Textures {
            player_idle,
            player_walk,
            walls,
            floors,
            water_1,
            water_2,
            menu_art,
            exit,
            new_game,
            quadrinho_1,
            quadrinho_2,
            quadrinho_3,
        }
    }
}
