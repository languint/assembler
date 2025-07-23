use serde::{Deserialize, Serialize};

use crate::models::force::{self};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerObservation {
    pub position: force::Position,
    pub orientation: force::RealOrientation,
    pub health: f64,
    pub inventory: Vec<force::Item>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityObservation {
    pub name: String,
    pub position: force::Position,
   
    pub direction: Option<force::Direction>,
    pub orientation: Option<force::RealOrientation>,

    pub current_health: f64,
    pub max_health: f64,
    pub prototype_type: String,

    pub has_items_in_output: bool,
    pub can_accept_items: bool,

    pub current_recipe: Option<String>,

    pub is_powered: bool,
    pub power_usage: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameObservation {
    pub game_tick: u32,
    pub player: PlayerObservation,
    pub visible_entities: Vec<EntityObservation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action_type")]
pub enum GameAction {
    #[serde(rename = "move_to_position")]
    MoveToPosition {
        target: force::Position,
    },
    #[serde(rename = "build_entity")]
    BuildEntity {
        entity_name: String,
        target_position: force::Position,
        direction: Option<force::Direction>,
        orientation: Option<force::RealOrientation>,
    },
    #[serde(rename = "craft_item")]
    CraftItem {
        item_name: String,
        amount: u32,
    },
    #[serde(rename = "research_technology")]
    ResearchTechnology {
        tech_name: String,
    },
    #[serde(rename = "set_assembler_recipe")]
    SetAssemblerRecipe {
        entity_id: u32,
        recipe_name: String,
    },
    #[serde(rename = "no_op")]
    NoOp,
}