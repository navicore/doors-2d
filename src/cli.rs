use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(long)]
    pub player: Option<PlayerType>,
    #[arg(long)]
    pub room_generator: Option<RoomGeneratorType>,
}

#[derive(clap::ValueEnum, Clone, Default)]
pub enum PlayerType {
    #[default]
    Demo1,
    Player1,
}

#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoomGeneratorType {
    Rooms2,
    Rooms5,
    #[default]
    Rooms25,
}

// todo: let player and room_generator code find these
