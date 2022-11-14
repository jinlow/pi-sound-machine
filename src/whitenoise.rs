use rodio::source::Source;

// Generate random value between -1.0, and 1.0.
fn rand_value() -> f32 {
    (fastrand::f32() - 0.5) * 2.0
}

pub struct WhiteNoise {
    //rng: ThreadRng
    value: f32,
    alpha: f32,
}

impl WhiteNoise {
    #[inline]
    pub fn new(alpha: f32) -> Self {
        WhiteNoise {
            value: rand_value(),
            alpha,
        }
    }
}

impl Iterator for WhiteNoise {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.value += rand_value() * self.alpha;
        Some(self.value.sin())
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
