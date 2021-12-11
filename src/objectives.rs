pub struct Objective {
    kind: Kind,
    current: u8,
    target: u8,
}

enum Kind {
    None,
    SaveAnimals,
    DestroyBuildings,
    KillEnemies,
    KillBosses,
}

impl Objective {
    fn new(kind: Kind, target: u8) -> Self {
        Self {
            current: 0,
            target,
            kind,
        }
    }

    pub fn none() -> Self {
        Self::new(Kind::None, 1)
    }

    pub fn save_animals(target: u8) -> Self {
        Self::new(Kind::SaveAnimals, target)
    }

    pub fn destroy_buildings(target: u8) -> Self {
        Self::new(Kind::DestroyBuildings, target)
    }

    pub fn kill_enemies(target: u8) -> Self {
        Self::new(Kind::KillEnemies, target)
    }

    pub fn kill_bosses(target: u8) -> Self {
        Self::new(Kind::KillBosses, target)
    }

    pub fn is_complete(&self) -> bool {
        crate::debug::AUTO_COMPLETE_OBJECTIVES || self.current >= self.target
    }

    pub fn on_update_death_ball_count(&mut self, current: u8) {
        if let Kind::SaveAnimals { .. } = self.kind {
            self.current = current;
        }
    }

    pub fn on_kill_enemy(&mut self) {
        if let Kind::KillEnemies { .. } | Kind::KillBosses { .. } = self.kind {
            self.current += 1;
        }
    }

    pub fn on_destroy_building(&mut self) {
        if let Kind::DestroyBuildings { .. } = self.kind {
            self.current += 1;
        }
    }

    pub fn current(&self) -> u8 {
        self.current
    }
}

impl std::fmt::Display for Objective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (prefix, suffix_singular, suffix_plural) = match self.kind {
            Kind::None => return f.write_str("None"),
            Kind::SaveAnimals => ("Save", "Animal", "Animals"),
            Kind::DestroyBuildings => ("Destroy", "Building", "Buildings"),
            Kind::KillEnemies => ("Kill", "Enemy", "Enemies"),
            Kind::KillBosses => ("Kill", "Boss", "Bosses"),
        };
        let suffix = if self.target == 1 {
            suffix_singular
        } else {
            suffix_plural
        };
        write!(f, "{} {} {}", prefix, self.target, suffix)
    }
}
