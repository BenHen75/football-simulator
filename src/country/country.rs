use crate::core::SimulationContext;
use crate::league::League;

pub struct Country {
    pub name: String,
    pub leagues: Vec<League>,
    pub reputation: u16,
}

impl Country {
    pub fn items_count(&self) -> usize {
        self.leagues.iter().map(|league| league.items_count()).sum()
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        for league in &mut self.leagues {
            league.simulate(context);
            
        }
        
        if context.is_month_beginning() {
            
        }
    }
}