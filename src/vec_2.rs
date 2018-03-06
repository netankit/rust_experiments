// Struct Declaration
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

    // structs are immutable by default, use `mut` to make it mutable but doesn't support field level mutability
    let mut link_color = Color {red: 0,green: 0,blue: 251};
    link_color.blue = 238;
    println!("Link Color = rgb({}, {}, {})", link_color.red, link_color.green, link_color.blue); //Link Color = rgb(0, 0, 238)
  
}