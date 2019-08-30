struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// These are called "tuple structs".  They don't have named fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("lincolnsimmons@gmail.com"),
        username: String::from("Lncn"),
        sign_in_count: 1,
        active: true,
    };

    let mut white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);

    user1.username = String::from("Lncln");
    println!("User: {}, {} ({} {})", user1.username, user1.email, user1.active, user1.sign_in_count);

    white.2 = white.0 - 1;

    println!("Color RGB: ({}, {}, {})", white.0, white.1, white.2);
    println!("Origin Pt: ({}, {}, {})", origin.0, origin.1, origin.2);

    example_ch05_02();
}

#[derive(Debug)]
struct Triangle {
    width: u32,
    height: u32,
}

impl Triangle {
    fn area(&self) -> u32 {
        (self.width * self.height) / 2
    }

    fn can_hold(&self, other: &Triangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is an ASSOCIATED FUNCTION of Triangle.  This means it doesn't take a self,
    // so it doesn't act on an existing instance, but is just associated with the
    // type.  For example, String::from()
    fn equilateral(size: u32) -> Triangle {
        let height = 0.8660254 * size as f32; // sqrt(3)/2
        Triangle{width: size, height: height as u32}
    }
}

fn example_ch05_02() {
    let triangle1 = Triangle{width: 20, height: 60};

    println!("The area of the triangle is {} sqpx.", triangle1.area());
    // The {:?} allows you to print the "Debug trait" for structures, however
    // you have to opt-in using the `#[derive(Debug)]` annotation above your struct
    println!("You can print structs like this: {:?}", triangle1);

    let tri2 = Triangle{width: 19, height: 50};
    println!("{:?} can fit in first triangle: {}", tri2, triangle1.can_hold(&tri2));
    let tri3 = Triangle{width: 25, height: 50};
    println!("{:?} can fit in first triangle: {}", tri3, triangle1.can_hold(&tri3));
    let tri4 = Triangle::equilateral(100);
    println!("{:?} was created with the equilateral() associated fn", tri4);
}
