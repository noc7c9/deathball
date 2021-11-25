pub struct Objective {
    kind: Kind,
    amount: u8,
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
            amount: 0,
            target,
            kind,
        }
    }

    pub fn none() -> Self {
        Self::new(Kind::None, 0)
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
        self.amount >= self.target
    }

    pub fn on_update_death_ball_count(&mut self, amount: u8) {
        if let Kind::SaveAnimals { .. } = self.kind {
            self.amount = amount;
        }
    }

    pub fn on_kill_enemy(&mut self) {
        if let Kind::KillEnemies { .. } | Kind::KillBosses { .. } = self.kind {
            self.amount += 1;
        }
    }

    pub fn on_destroy_building(&mut self) {
        if let Kind::DestroyBuildings { .. } = self.kind {
            self.amount += 1;
        }
    }

    pub fn progress_string(&self) -> String {
        let prefix = match self.kind {
            Kind::None => return "None".to_string(),
            Kind::SaveAnimals => "Save Animals",
            Kind::DestroyBuildings => "Destroy Buildings",
            Kind::KillEnemies => "Kill Enemies",
            Kind::KillBosses => "Kill Bosses",
        };
        format!("{}: {} of {}", prefix, self.amount, self.target)
    }
}
