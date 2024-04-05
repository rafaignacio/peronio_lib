use std::fmt::Display;

#[derive(Debug)]
pub struct Level {
    inner: u64,
    xp_points: u128,
    next_level_threshold: u128,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Level {}, XP Points {}, XP To Next Level {}",
            self.inner, self.xp_points, self.next_level_threshold
        )
    }
}

impl Default for Level {
    fn default() -> Self {
        Level {
            inner: u64::default(),
            xp_points: u128::default(),
            next_level_threshold: u128::default(),
        }
    }
}

impl Level {
    pub fn get_current_level(&self) -> u64 {
        self.inner
    }

    pub fn add_experience_points(&mut self, xp: u64) {
        self.xp_points += xp as u128;
    }

    pub fn from(xp: u128) -> Level {
        let mut level = 1_u64;

        while xp >= get_next_level_threshold(level) {
            level += 1;
        }

        Level {
            inner: level,
            xp_points: xp,
            next_level_threshold: get_next_level_threshold(level),
        }
    }
}

fn get_next_level_threshold(level: u64) -> u128 {
    ((50_f64 * f64::powi(level as f64, 3) - 150_f64 * f64::powi(level as f64, 2)
        + 400_f64 * level as f64)
        / 3_f64) as u128
}
