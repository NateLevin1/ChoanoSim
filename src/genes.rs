use crate::{
    randoms::{random, random_float},
    simulator::SimulatorConfig,
};

#[derive(Copy, Clone)]
pub struct Genes {
    pub size: f64,
    pub flagellum_size: f64,
    pub stomach_size: f64,
    pub steps_until_child_born: f64,
    // TODO: gender?
}

impl Genes {
    pub fn default() -> Self {
        Genes {
            size: 27.0 + random(6) as f64,
            stomach_size: 9.0 + random(2) as f64,
            flagellum_size: 4.5 + random_float(),
            steps_until_child_born: 195.0 + random(10) as f64,
        }
    }
    pub fn mix(&self, other: &Self, config: &SimulatorConfig) -> Self {
        let size = pick_with_mutation(self.size, other.size, config);
        return Genes {
            size,
            // prevent stomach from being greater than size
            stomach_size: pick_with_mutation(self.stomach_size, other.stomach_size, config)
                .min(size),
            flagellum_size: pick_with_mutation(self.flagellum_size, other.flagellum_size, config),
            steps_until_child_born: pick_with_mutation(
                self.steps_until_child_born,
                other.steps_until_child_born,
                config,
            ),
        };
    }
}

fn pick_with_mutation(a: f64, b: f64, config: &SimulatorConfig) -> f64 {
    let mut chosen = pick(a, b);

    if random_float() < config.mutation_chance {
        let sign = pick(-1.0, 1.0);
        let max_mutation_amount = ((a + b) / 2.0) * config.mutation_percent_change;
        chosen += sign * max_mutation_amount;
    }

    chosen
}

fn pick(a: f64, b: f64) -> f64 {
    if random_float() > 0.5 {
        a
    } else {
        b
    }
}
