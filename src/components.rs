use ::pixi::graphics::Graphics;

#[derive(Clone, PartialEq, Debug)]
pub struct Position{pub x: f64, pub y: f64}

#[derive(Clone, PartialEq, Debug)]
pub struct Velocity{pub x: f64, pub y: f64}

#[derive(Clone, PartialEq, Debug)]
pub struct Health{pub health: i32}

#[derive(Clone, PartialEq, Debug)]
pub struct Renderer{pub graphics: Graphics}

#[derive(Clone, PartialEq, Debug)]
pub struct KeyboardControls;

#[derive(Clone, PartialEq, Debug)]
pub struct Player;

#[derive(Clone, PartialEq, Debug)]
pub struct Enemy;
