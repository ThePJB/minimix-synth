use minimix::Signal;

pub trait Plot {
    fn save(&self, name: &str);
}

impl Plot for Signal {
    
}