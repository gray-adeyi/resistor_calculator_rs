/// Enum of resistor color codes.
#[derive(Debug)]
pub enum ColorCode {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Gray,
    White,
}

impl ColorCode {
    /// Computes the multiplier value a resistor from its
    /// color code.
    pub fn multiplier(&self) -> u32 {
        match self {
            ColorCode::Black => utils::multiplier(0),
            ColorCode::Brown => utils::multiplier(1),
            ColorCode::Red => utils::multiplier(2),
            ColorCode::Orange => utils::multiplier(3),
            ColorCode::Yellow => utils::multiplier(4),
            ColorCode::Green => utils::multiplier(5),
            ColorCode::Blue => utils::multiplier(6),
            ColorCode::Violet => utils::multiplier(7),
            ColorCode::Gray => utils::multiplier(8),
            ColorCode::White => utils::multiplier(9),
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            ColorCode::Black => 0,
            ColorCode::Brown => 1,
            ColorCode::Red => 2,
            ColorCode::Orange => 3,
            ColorCode::Yellow => 4,
            ColorCode::Green => 5,
            ColorCode::Blue => 6,
            ColorCode::Violet => 7,
            ColorCode::Gray => 8,
            ColorCode::White => 9,
        }
    }
}

pub enum ToleranceColorCode {
    Gold,
    Silver,
}

impl ToleranceColorCode {
    pub fn value(&self) -> f64 {
        match self {
            ToleranceColorCode::Gold => 0.05,
            ToleranceColorCode::Silver => 0.1,
        }
    }
}

/// Enum of the number of color bands a resistor has.
pub enum Band {
    Three,
    Four,
    Five,
}

pub struct FourBandResistor {
    pub band1: ColorCode,
    pub band2: ColorCode,
    pub band3: ColorCode,
    pub tolerance: ToleranceColorCode,
}

impl FourBandResistor {
    pub fn ideal_value(&self) -> u32 {
        let prefix_value = format!("{}{}",self.band1.value(),self.band2.value());
        if let Ok(val) = prefix_value.parse::<u32>() {
            val * self.band3.multiplier()
        } else{
            0
        }
    }

    pub fn value_range(&self) -> (f64, f64) {
        let ideal_value: f64 = self.ideal_value().into();
        let min_value = ideal_value - self.tolerance.value();
        let max_value = ideal_value + self.tolerance.value();
        (min_value, max_value)
    }
}

struct FiveBandResistor {
    band1: ColorCode,
    band2: ColorCode,
    band3: ColorCode,
    band4: ColorCode,
    tolerance: ToleranceColorCode,
}

mod utils {
    /// Utility to calculate the multiplier value from the index
    /// of the color in `ColorCode`
    pub fn multiplier(value: u8) -> u32 {
        if value <= 0 {
            1
        } else {
            10 * multiplier(value - 1)
        }
    }
}
