use minimix::Signal;

// i mean blanked implementations for sample type, is sin a trait or something. probably. num trait?

pub trait Gen {
    fn zero(n: usize) -> Self;
    fn sin(n: usize, wn: f32) -> Self;
    fn saw(n: usize, wn: f32) -> Self;
    fn square(n: usize, wn: f32) -> Self;
    fn white(n: usize) -> Self;
}

impl Gen for Signal {

}
