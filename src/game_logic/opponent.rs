use crate::pieces::{PieceColor, PieceMove};
use log;
use std::{
    io::{Read, Write},
    net::TcpStream,
    panic,
};

pub struct Opponent {
    /// Used to indicate if a Opponent move is following
    pub opponent_will_move: bool,
    // The color of the Opponent
    pub color: PieceColor,
    /// Is Game started
    pub game_started: bool,
}

// Custom Default implementation
impl Default for Opponent {
    fn default() -> Self {
        Opponent {
            opponent_will_move: false,
            color: PieceColor::Black,
            game_started: false,
        }
    }
}

impl Clone for Opponent {
    fn clone(&self) -> Self {
        Opponent {
            opponent_will_move: self.opponent_will_move,
            color: self.color,
            game_started: self.game_started,
        }
    }
}

impl Opponent {
    pub fn copy(&self) -> Self {
        Opponent {
            opponent_will_move: self.opponent_will_move,
            color: self.color,
            game_started: self.game_started,
        }
    }

    pub fn new(color: Option<PieceColor>) -> Opponent {
        log::info!("Creating new opponent with color: {:?}", color);

        let color = match color {
            Some(color) => {
                log::info!("Using provided color: {:?}", color);
                color
            }
            None => {
                log::info!("No color provided");
                // Default to black if no color is provided
                PieceColor::Black
            }
        };

        let opponent_will_move = match color {
            PieceColor::White => true,
            PieceColor::Black => false,
        };
        log::info!(
            "Created opponent with color {:?}, will_move: {}",
            color,
            opponent_will_move
        );
        Opponent {
            opponent_will_move,
            color,
            game_started: false,
        }
    }

    pub fn wait_for_game_start(mut stream: &TcpStream) {
        let mut buffer = [0; 5];
        let bytes_read = stream.read(&mut buffer).unwrap(); // Number of bytes read
        let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        match response.as_str() {
            "s" => (),
            _ => panic!("Failed to get color from stream"),
        }
    }
}
