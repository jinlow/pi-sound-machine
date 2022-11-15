use rodio::source::Source;
// Generate random value between -1.0, and 1.0.
fn rand_value() -> f32 {
    (fastrand::f32() - 0.5) * 2.0
}

/// White Noise generator
/// The alpha parameter sets the color, the lower the value, the
/// darker the color (brown noise), the higher the value, the
/// brighter the color (white to violet noise).
#[derive(Clone)]
pub struct WhiteNoise {
    value: f32,
    alpha: f32,
}

impl WhiteNoise {
    #[inline]
    pub fn new(alpha: f32) -> Self {
        WhiteNoise {
            value: 0.0,
            alpha,
        }
    }
}

impl Iterator for WhiteNoise {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Adding this clamp, instead of using the sine functions, seemed to sound a little better.
        if (self.value > 1.0) || (self.value < -1.0) {
            self.value *= self.alpha
        }

        // If the previous filter didn't make it small enough, clip back to zero
        if self.value > 1.0 {
            self.value = 0.0
        } else if self.value < -1.0 {
            self.value = 0.0
        }
        self.value += rand_value() * self.alpha;
        Some(self.value)
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
