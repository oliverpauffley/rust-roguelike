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
