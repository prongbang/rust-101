pub trait FlyBehaviour {
    fn fly(&self);
}

pub struct FlyWithWings;

pub struct FlyNoWay;

impl FlyBehaviour for FlyWithWings {
    fn fly(&self) {
        println!("\ti can fly")
    }
}

impl FlyBehaviour for FlyNoWay {
    fn fly(&self) {
        println!("\tis can't fly")
    }
}

pub trait Duck {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour;
    fn fly(&self) {
        let s = self.get_fly_behaviour();
        s.fly();
    }
}

pub struct MallardDuck {
    fly_behaviour: Box<dyn FlyBehaviour>,
}

impl Duck for MallardDuck {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour {
        return &(*self.fly_behaviour);
    }
}

impl MallardDuck {
    pub fn new(fly_behaviour: Box<dyn FlyBehaviour>) -> Self {
        MallardDuck { fly_behaviour }
    }

    pub fn set_fly_behaviour(&mut self, fly_behaviour: Box<dyn FlyBehaviour>) {
        self.fly_behaviour = fly_behaviour
    }
}

pub struct ModelDuck {
    fly_behaviour: Box<FlyNoWay>,
}

impl Duck for ModelDuck {
    fn get_fly_behaviour(&self) -> &dyn FlyBehaviour {
        return &(*self.fly_behaviour);
    }
}

impl ModelDuck {
    pub fn new(fly_behaviour: Box<FlyNoWay>) -> Self {
        ModelDuck { fly_behaviour }
    }
}