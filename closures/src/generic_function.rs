struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

fn generic_make_noise_full<T>(creature: &T)
    where
        T: NoiseMaker,
{
    // we know the real type at compile-time
    creature.make_noise();
}

fn generic_make_noise_short(creature: &impl NoiseMaker) {
    // we know the real type at compile-time
    creature.make_noise();
}

pub fn run_generic() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    generic_make_noise_full(&creature);
    generic_make_noise_short(&creature);
}