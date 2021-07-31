pub use crate::prelude::*;

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
