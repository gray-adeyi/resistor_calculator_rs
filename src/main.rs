use resistor_calculator::{ColorCode, FourBandResistor, ToleranceColorCode};

fn main() {
    let my_resistor = FourBandResistor{
        band1: ColorCode::Green,
        band2: ColorCode::Blue,
        band3: ColorCode::Yellow,
        tolerance: ToleranceColorCode::Silver
    };
    println!("My resistor is {} ohms", my_resistor.ideal_value());
    println!("My resistor range {:?}", my_resistor.value_range());
}
