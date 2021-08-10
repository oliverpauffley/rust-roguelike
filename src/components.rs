pub use crate::prelude::*;
use std::collections::HashSet;

/// Render describes the color and symbol needed to draw an entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

/// Player is a tag component to signal an entity is a player.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

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
