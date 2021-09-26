pub use crate::prelude::*;
use std::collections::HashSet;

/// Render describes the color and symbol needed to draw an entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
    pub render_order: u32, // the order of rendering from 0 (first).
}

pub const PARTICLE_RENDER_ORDER: u32 = 0;
pub const PLAYER_RENDER_ORDER: u32 = 1;
pub const MONSTER_RENDER_ORDER: u32 = 2;
pub const ITEM_RENDER_ORDER: u32 = 3;
pub const MAP_RENDER_ORDER: u32 = 4;

/// Player is a tag component to signal an entity is a player.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32,
}

/// Enemy is a tag component to signal an entity is a bad guy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

/// MovingRandomly signals an entity should move around at random.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

/// Chasing player signals a monster will chase the player (if in vision).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

/// WantsToMove is a message representing the intent to move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

/// WantsToAttack is a message representing the intent to attack.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
/// Health represents the hit points of a creature.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

/// Name provides an identifer for an entity.
#[derive(Clone, PartialEq)]
pub struct Name(pub String);

/// Item denotes an object that can be carried.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

/// Consumable is a tag for an item that is destroyed after use.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Consumable;

/// AmuletOfYala denotes the win condition of the game. Pick up this item and you have won.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

/// FieldOfView gives an entity vision of the map.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

/// ParticleLifetime sets the duration of a partice effect.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32,
}

/// ProvidesHealing gives an entity the ability to return hp to the player.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

/// ProvidesDungeonMap is a tag for an entity that reveals the dungeon layout.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

/// Carried allows an entity to be picked up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Carried(pub Entity);

/// ActivateItem is a message that an entity wants to use another entity
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

/// Damage decribes the hit point reduction from an entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Damage(pub i32);

/// Ranged describes an entity with a distance limited affect.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ranged {
    pub range: i32,
}

/// Weapon is a tag to set an item as damaging.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weapon;
