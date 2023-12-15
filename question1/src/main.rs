//Question1
//Collaborators: None

use std::ops::{Neg};
#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
struct Point<T> {
    x: T,
    y: T
}

impl <T: Copy + Neg<Output = T>>Point<T> {
    fn clockwise_rotation (&self) -> Point<T>{
        Point {
            x: self.y,
            y: -self.x
        }
    }
    fn counterclockwise_rotation (&self) -> Point<T>{
        Point{
            x: -self.y,
            y: self.x
        }
    }
}

fn create_point<T> (x: T, y: T) -> Point<T> {
    Point {x , y}
}

fn main () {
    //Clockwise rotation on f64 point
    let point_f64 = create_point(3.0, 2.0);
    println!("The point with type f64 is: {:?}", point_f64);
    let rotated_f64 = point_f64.clockwise_rotation();
    println!("After rotating clockwise, the point with type f64 is: {:?}", rotated_f64);

    //Counterclockwise rotation on i32 point
    let point_i32 = create_point(3, 2);
    println!("The point with type i32 is: {:?}", point_i32);
    let rotated_i32 = point_i32.counterclockwise_rotation();
    println!("After rotating counterclockwise, the point with type i32 is: {:?}", rotated_i32);

    //Functionality tests, will say failed if not equal
    assert_eq!(create_point(2.0, -3.0), rotated_f64, "Clockwise function failed");
    assert_eq!(create_point(-2, 3), rotated_i32, "Counterclockwise function failed");
}