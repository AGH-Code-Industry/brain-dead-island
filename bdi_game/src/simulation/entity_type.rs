//survivor(player)

pub enum EntityType {
    Survivor(SurvivorData),
    Zombie(ZombieData),
}
#[derive(Clone)]
pub struct SurvivorData {
    pub name: String,
    pub health: u64,
    pub hunger: u64,
    pub thirst: u64,
    pub energy: u64,
    //pub inventory: Inventory,
}

impl Default for SurvivorData {
    fn default() -> SurvivorData {
        SurvivorData {
            name: "Survivor".to_string(),
            health: 100,
            hunger: 100,
            thirst: 100,
            energy: 100,
            // inventory: Inventory::new(),
        }
    }
}

//zombie
#[derive(Clone)]
pub struct ZombieData {
    pub health: u64,
}

impl Default for ZombieData {
    fn default() -> ZombieData {
        ZombieData { health: 100 }
    }
}
