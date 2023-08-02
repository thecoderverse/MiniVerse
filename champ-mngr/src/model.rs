use std::fmt::{Display, Formatter};

pub struct GameState {
    pub current_score: u32,
}

#[derive(Debug)]
pub struct Manager {
    pub nick_name: String,
    pub current_level: Level,
    pub score: Score,
}

#[derive(Default, Debug)]
pub struct Score {
    pub win: u32,
    pub loose: u32,
    pub draw: u32,
}

impl Manager {
    pub fn new(nick_name: String, current_level: Level) -> Self {
        Self {
            nick_name,
            current_level,
            score: Score::default(),
        }
    }
    pub fn calc_bonus(&self) -> u16 {
        10
    }
}

impl Display for Manager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-[{}] ({}:{}:{})",
            self.nick_name, self.current_level, self.score.win, self.score.loose, self.score.draw
        )
    }
}

#[derive(Debug)]
pub enum Level {
    Amateur,
    Pro,
    International,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Amateur => {
                write!(f, "Amatör")
            }
            Level::Pro => {
                write!(f, "Profesyonel")
            }
            Level::International => {
                write!(f, "Milli Takım")
            }
        }
    }
}
