use minimix::Signal;

pub trait Effect {
    fn reverb(self) -> Self;
    fn echo(self) -> Self;
    fn delay(self) -> Self;
    fn tanh(self) -> Self;
    fn phase(self) -> Self;
    fn flange(self) -> Self;
}

impl Effect for Signal {

}