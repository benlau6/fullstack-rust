use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ApiResponse {
    pub count: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<ResponseItem>,
}

#[derive(Deserialize)]
pub struct ResponseItem {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
struct Sprite {
    front_default: Option<String>,
    back_default: Option<String>,
    front_shiny: Option<String>,
    back_shiny: Option<String>,
    other: OtherSprite,
}

#[derive(Deserialize)]
struct OtherSprite {
    dream_world: FrontDefault,
    #[serde(rename = "official-artwork")]
    official_artwork: FrontDefault,
}

#[derive(Deserialize)]
struct FrontDefault {
    front_default: Option<String>,
}

#[derive(Deserialize)]
pub struct MonsterFromApi {
    id: i32,
    name: String,
    sprites: Sprite,
    height: i16,
    weight: i16,
    types: Vec<TypeSlot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeSlot {
    slot: u8,
    #[serde(rename = "type")]
    pub type_: Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Monster {
    pub id: i32,
    pub name: String,
    pub height: i16,
    pub weight: i16,
    pub types: Vec<String>,
    pub image_url: Option<String>,
    pub image_url_game_front: Option<String>,
    pub image_url_game_back: Option<String>,
    pub image_url_game_front_shiny: Option<String>,
    pub image_url_game_back_shiny: Option<String>,
}
impl MonsterFromApi {
    pub fn get_types(&self) -> Vec<String> {
        self.types.iter().map(|t| t.type_.name.clone()).collect()
    }
    pub fn get_image_svg(&self) -> Option<String> {
        self.sprites
            .other
            .dream_world
            .front_default
            .as_ref()
            .cloned()
    }
    pub fn get_image_url(&self) -> Option<String> {
        self.sprites
            .other
            .official_artwork
            .front_default
            .as_ref()
            .cloned()
    }
    pub fn get_image_url_game_front(&self) -> Option<String> {
        self.sprites.front_default.as_ref().cloned()
    }
    pub fn get_image_url_game_back(&self) -> Option<String> {
        self.sprites.back_default.as_ref().cloned()
    }
    pub fn get_image_url_game_front_shiny(&self) -> Option<String> {
        self.sprites.front_shiny.as_ref().cloned()
    }
    pub fn get_image_url_game_back_shiny(&self) -> Option<String> {
        self.sprites.back_shiny.as_ref().cloned()
    }
}

impl From<MonsterFromApi> for Monster {
    fn from(monster: MonsterFromApi) -> Monster {
        let types = monster.get_types();
        let image_url = monster.get_image_url();
        let image_url_game_front = monster.get_image_url_game_front();
        let image_url_game_back = monster.get_image_url_game_back();
        let image_url_game_front_shiny = monster.get_image_url_game_front_shiny();
        let image_url_game_back_shiny = monster.get_image_url_game_back_shiny();
        Monster {
            id: monster.id,
            name: monster.name,
            height: monster.height,
            weight: monster.weight,
            types,
            image_url,
            image_url_game_front,
            image_url_game_back,
            image_url_game_front_shiny,
            image_url_game_back_shiny,
        }
    }
}
