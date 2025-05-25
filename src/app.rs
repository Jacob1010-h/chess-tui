use dirs::home_dir;
use log::LevelFilter;
use toml::Value;

use crate::{
    constants::{DisplayMode, Pages, Popups},
    game_logic::game::Game,
    pieces::PieceColor,
};

use std::{
    error,
    fs::{self, File},
    io::Write,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Game
    pub game: Game,
    /// Current page to render
    pub current_page: Pages,
    /// Current popup to render
    pub current_popup: Option<Popups>,
    // Selected color when playing against the bot
    pub selected_color: Option<PieceColor>,
    /// menu current cursor
    pub menu_cursor: u8,
    pub log_level: LevelFilter,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            game: Game::default(),
            current_page: Pages::Home,
            current_popup: None,
            selected_color: None,
            menu_cursor: 0,
            log_level: LevelFilter::Off,
        }
    }
}

impl App {
    pub fn toggle_help_popup(&mut self) {
        if self.current_popup == Some(Popups::Help) {
            self.current_popup = None;
        } else {
            self.current_popup = Some(Popups::Help);
        }
    }
    pub fn toggle_credit_popup(&mut self) {
        if self.current_page == Pages::Home {
            self.current_page = Pages::Credit;
        } else {
            self.current_page = Pages::Home;
        }
    }

    pub fn go_to_home(&mut self) {
        self.current_page = Pages::Home;
        self.restart();
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn menu_cursor_up(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
    }
    pub fn menu_cursor_right(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
    }
    pub fn menu_cursor_left(&mut self, l: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = l - 1;
        }
    }
    pub fn menu_cursor_down(&mut self, l: u8) {
        if self.menu_cursor < l - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
    }

    pub fn color_selection(&mut self) {
        self.current_popup = None;
        let color = match self.menu_cursor {
            0 => PieceColor::White,
            1 => PieceColor::Black,
            _ => unreachable!("Invalid color selection"),
        };
        self.selected_color = Some(color);
    }

    pub fn restart(&mut self) {
        self.game = Game::default();
        self.current_popup = None;
    }

    pub fn menu_select(&mut self) {
        match self.menu_cursor {
            0 => self.current_page = Pages::Solo,
            1 => {
                self.game.ui.display_mode = match self.game.ui.display_mode {
                    DisplayMode::ASCII => DisplayMode::DEFAULT,
                    DisplayMode::DEFAULT => DisplayMode::ASCII,
                };
                self.update_config();
            }
            2 => self.toggle_help_popup(),
            3 => self.current_page = Pages::Credit,
            _ => {}
        }
    }

    pub fn update_config(&self) {
        let home_dir = home_dir().expect("Could not get home directory");
        let config_path = home_dir.join(".config/chess-tui/config.toml");
        let mut config = match fs::read_to_string(config_path.clone()) {
            Ok(content) => content
                .parse::<Value>()
                .unwrap_or_else(|_| Value::Table(Default::default())),
            Err(_) => Value::Table(Default::default()),
        };

        if let Some(table) = config.as_table_mut() {
            table.insert(
                "display_mode".to_string(),
                Value::String(self.game.ui.display_mode.to_string()),
            );
            table.insert(
                "log_level".to_string(),
                Value::String(self.log_level.to_string().to_string()),
            );
        }

        let mut file = File::create(config_path.clone()).unwrap();
        file.write_all(config.to_string().as_bytes()).unwrap();
    }

    pub fn reset(&mut self) {
        self.game = Game::default();
        self.current_popup = None;
        self.selected_color = None;
        self.menu_cursor = 0;
    }
}
