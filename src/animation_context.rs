
pub struct Oscillators {
    pub o1: usize,
}


pub struct AnimationContext {
    pub frame_count: u32,
    pub oscillators: Oscillators

    // TODO: Consider storing GraphContext in the AnimationContext in order to simplify passing data around

}