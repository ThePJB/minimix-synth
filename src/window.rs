use minimix::Signal;

pub trait Window {
    fn wnd_adsr(self, a: usize, d: usize, s: f32, r: usize, control: Signal) -> Self;    // wait this takes another signal as argument - control input
    fn wnd_exp(self, a: f32) -> Self;
    fn wnd_blackman(self) -> Self;
    fn fade_in(self, t: usize, d: usize) -> Self;
    fn fade_out(self, t: usize, d: usize) -> Self;
}

impl Window for Signal {

}
