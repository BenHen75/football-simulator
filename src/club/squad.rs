use crate::club::tactics::Tactics;
use crate::people::{Player, PlayerPositionType};

#[derive(Debug)]
pub struct Squad {
    pub club_id: u32,
    pub tactics: Tactics,
    pub players: Vec<(PlayerPositionType, Player)>,
}

impl Squad {}