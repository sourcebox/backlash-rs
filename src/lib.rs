#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

/// Backlash processor.
#[derive(Debug, Default)]
pub struct Backlash<T>
where
    T: Default,
{
    /// Current output value.
    output_value: T,

    /// Width of the deadband divided by 2.
    deadband_half_width: T,

    /// Lower border before output value changes.
    lower_border: T,

    /// Upper border before output value changes.
    upper_border: T,
}

impl<T> Backlash<T>
where
    T: Default
        + Copy
        + PartialEq
        + PartialOrd
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Div<Output = T>
        + From<u8>,
{
    /// Returns a new instance.
    pub fn new(deadband_width: T) -> Self {
        let mut instance = Self {
            ..Default::default()
        };

        instance.set_deadband_width(deadband_width);

        instance
    }

    /// Processes a new input and returns the output value.
    pub fn update(&mut self, value: T) -> T {
        if value > self.upper_border {
            self.lower_border = value - (self.deadband_half_width + self.deadband_half_width);
            self.upper_border = value;
            self.output_value = value - self.deadband_half_width;
        } else if value < self.lower_border {
            self.lower_border = value;
            self.upper_border = value + (self.deadband_half_width + self.deadband_half_width);
            self.output_value = value + self.deadband_half_width;
        };

        self.output_value
    }

    /// Sets a new output value without any processing.
    pub fn set_value(&mut self, value: T) {
        self.output_value = value;
    }

    /// Returns the last output value.
    pub fn value(&self) -> T {
        self.output_value
    }

    /// Sets a new deadband width and centers the borders around the last output value.
    pub fn set_deadband_width(&mut self, deadband_width: T) {
        self.deadband_half_width = deadband_width / 2.into();
        self.center_borders(self.output_value);
    }

    /// Returns the deadband width.
    pub fn deadband_width(&self) -> T {
        self.deadband_half_width + self.deadband_half_width
    }

    /// Returns the deadband borders as tuple of (lower, upper).
    pub fn borders(&self) -> (T, T) {
        (self.lower_border, self.upper_border)
    }

    /// Center the deadband borders around a specific value.
    pub fn center_borders(&mut self, value: T) {
        self.lower_border = value - self.deadband_half_width;
        self.upper_border = value + self.deadband_half_width;
    }
}

#[cfg(test)]
mod tests;
