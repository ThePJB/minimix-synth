use minimix::Signal;

pub trait Ops {
    fn fft(self) -> Self;
    fn ifft(self) -> Self;
    fn conv_slow(self) -> Self;
    fn conv_fast(self) -> Self;
}

impl Ops for Signal {

}