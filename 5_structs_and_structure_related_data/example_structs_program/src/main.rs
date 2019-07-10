fn main() {
    calculate_area();
    calculate_tuple_area();
    calculate_struct_area();
}

fn calculate_area() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "calculate_area: The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_tuple_area() {
    /*
    In one way, this function is better. Tuples let us add a bit of structure, 
    and we’re now passing just one argument. But in another way, this version 
    is less clear: tuples don’t name their elements, so our calculation has 
    become more confusing because we have to index into the parts of the tuple.
    */
    let rect1 = (30, 50);

    println!(
        "calculate_tuple_area: The area of the rectangle is {} square pixels.",
        tuple_area(rect1)
    );
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_struct_area() {

let rect1 = Rectangle { width: 30, height: 50 };

    println!("calculate_struct_area: rect1 is {:?}", rect1);
    println!(
        "calculate_struct_area: The area of the rectangle is {} square pixels.",
        struct_area(&rect1)
    );
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
