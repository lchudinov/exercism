pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 if self.level >= 10 => Some(Player {
                health: 100,
                mana: Some(100),
                level: self.level,
            }),
            0 => Some(Player {
                health: 100,
                mana: None,
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana >= mana_cost => {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            }
            Some(_) => 0,
            None if self.health >= mana_cost => {
                self.health = self.health - mana_cost;
                0
            }
            _ => {
                self.health = 0;
                0
            }
        }
    }
}
