use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(long)]
    pub player: PlayerType,
    #[arg(long)]
    pub room_generator: RoomGeneratorType,
}

#[derive(clap::ValueEnum, Clone)]
pub enum PlayerType {
    Demo1,
    Player1,
}

#[derive(clap::ValueEnum, Clone)]
pub enum RoomGeneratorType {
    Rooms2,
    Rooms5,
    Rooms25,
}

// todo: let player and room_generator code find these
