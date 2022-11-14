use rodio::source::Source;

pub struct WhiteNoise {
    //rng: ThreadRng
}

impl WhiteNoise {
    #[inline]
    pub fn new() -> Self {
        WhiteNoise {
            //rng: thread_rng()
        }
    }

}

impl Iterator for WhiteNoise {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some((fastrand::f32() - 1.0) * 0.5)
    }
}

impl Source for WhiteNoise {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
