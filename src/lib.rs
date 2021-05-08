pub struct Ema {
    num: usize, // Smoothing number
    ema: f64,   // Current result of the Exponetial Moving Average (EMA)
    // These next values are needed until the number of sampels is > num
    full: bool,
    count: usize,
    average: f64,
}

impl Ema {
    pub fn new(num: usize) -> Ema {
        Ema {
            num: num,
            ema: -1.0,
            full: false,
            count: 0,
            average: 0.0,
        }
    }

    pub fn update(&mut self, sample: f64) -> f64 {
        if self.full {
            // We have enough samples to calculate the EMA
            self.ema = sample * 2.0 / ((self.num as f64) + 1.0)
                + self.ema * (1.0 - 2.0 / ((self.num as f64) + 1.0));
        } else {
            // Until we have reached self.num samples, calculate a simple moving average
            self.average =
                (self.average * (self.count as f64) + sample) / ((self.count as f64) + 1.0);
            self.count += 1;
            if self.count == self.num {
                self.full = true;
                self.ema = self.average;
            } else {
                // Need to keep accumulating samples
            }
        }
        // Return the most recent value
        self.ema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn small_series() {
        let mut ema = Ema::new(11);
        assert!(approx_eq!(f64, -1.0, ema.update(55.0), ulps = 2));
        assert!(approx_eq!(f64, -1.0, ema.update(57.0), ulps = 2));
        assert!(approx_eq!(f64, -1.0, ema.update(80.0), ulps = 2));
        assert!(approx_eq!(f64, -1.0, ema.update(74.0), ulps = 2));
        assert!(approx_eq!(f64, -1.0, ema.update(52.0), ulps = 2));
        assert!(approx_eq!(f64, -1.0, ema.update(72.0), ulps = 2));
        assert!(approx_eq!(f64, -1.0, ema.update(90.0), ulps = 2));
        assert!(approx_eq!(f64, -1.0, ema.update(41.0), ulps = 2));
        assert!(approx_eq!(f64, -1.0, ema.update(58.0), ulps = 2));
        assert!(approx_eq!(f64, -1.0, ema.update(16.0), ulps = 2));
        assert!(approx_eq!(f64, 63.0, ema.update(98.0), ulps = 2));
        assert!(approx_eq!(
            f64,
            65.33333333333333,
            ema.update(77.0),
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            65.11111111111111,
            ema.update(64.0),
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            61.92592592592593,
            ema.update(46.0),
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            58.77160493827161,
            ema.update(43.0),
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            58.976337448559676,
            ema.update(60.0),
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            63.14694787379973,
            ema.update(84.0),
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            56.62245656149978,
            ema.update(24.0),
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            50.18538046791649,
            ema.update(18.0),
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            56.32115038993041,
            ema.update(87.0),
            ulps = 2
        ));
    }
}
