use minimix::Signal;

pub trait Effect {
    fn reverb(self) -> Self;
    fn echo(self) -> Self;
    fn delay(self) -> Self;
    fn tanh(self) -> Self;
}

impl Effect for Signal {
    
}