use minimix::Signal;

pub trait FIR {
    fn fir_lp(n: usize, fc: f32) -> Self;
    fn fir_hp(n: usize, fc: f32) -> Self;
}
