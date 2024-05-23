#[derive(Debug)]
struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}
#[derive(Debug)]
struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitLikeStruct;

fn main() {
    // Instantiate a classic c struct
    let green = ColorClassicStruct {
        red: 0,
        green: 255,
        blue: 0,
    };

    println!("Green (classic struct): {:?}", green);

    // Instantiate a tuple struct
    let green_tuple = ColorTupleStruct(0, 255, 0);

    println!("Green (tuple struct): {:?}", green_tuple);

    // Instantiate a unit-like struct
    let unit_like_struct = UnitLikeStruct;

    println!("Unit-like struct: {:?}", unit_like_struct);
}
