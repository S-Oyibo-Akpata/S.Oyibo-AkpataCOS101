use std::io;

// Function to read a floating-point number from user input
fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

// 1. Trapezium, area = height / 2 * (base1 + base2)
fn calculate_trapezium_area() {
    println!("\n--- Trapezium Area ---");
    let height = read_input("Enter height:");
    let base1 = read_input("Enter base 1:");
    let base2 = read_input("Enter base 2:");

    let area = (height / 2.0) * (base1 + base2);
    println!("The area of the trapezium is: {:.2}", area);
}

// 2. Rhombus, area = 1/2 * diagonal1 * diagonal2
fn calculate_rhombus_area() {
    println!("\n--- Rhombus Area ---");
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");

    let area = 0.5 * d1 * d2;
    println!("The area of the rhombus is: {:.2}", area);
}

// 3. Parallelogram, area = base * altitude
fn calculate_parallelogram_area() {
    println!("\n--- Parallelogram Area ---");
    let base = read_input("Enter base:");
    let altitude = read_input("Enter altitude:");

    let area = base * altitude;
    println!("The area of the parallelogram is: {:.2}", area);
}

// 4. Cube, surface area = 6 * side * side
fn calculate_cube_surface_area() {
    println!("\n--- Cube Surface Area ---");
    let side = read_input("Enter side length:");

    let surface_area = 6.0 * side * side;
    println!("The surface area of the cube is: {:.2}", surface_area);
}

// 5. Cylinder, volume = pi * radius * radius * height
fn calculate_cylinder_volume() {
    println!("\n--- Cylinder Volume ---");
    let radius = read_input("Enter radius:");
    let height = read_input("Enter height:");

    let volume = std::f64::consts::PI * radius * radius * height;
    println!("The volume of the cylinder is: {:.2}", volume);
}

fn main() {
    loop {
        println!("\n==============================");
        println!("     THE SHAPE CALCULATOR     ");
        println!("==============================");
        println!("1. Trapezium (Area)");
        println!("2. Rhombus (Area)");
        println!("3. Parallelogram (Area)");
        println!("4. Cube (Surface Area)");
        println!("5. Cylinder (Volume)");
        println!("6. Exit");
        println!("Enter your choice (1-6): ");

        let choice = read_input("");

        if choice == 1.0 {
            calculate_trapezium_area();
        } else if choice == 2.0 {
            calculate_rhombus_area();
        } else if choice == 3.0 {
            calculate_parallelogram_area();
        } else if choice == 4.0 {
            calculate_cube_surface_area();
        } else if choice == 5.0 {
            calculate_cylinder_volume();
        } else if choice == 6.0 {
            println!("Exiting program. Goodbye!");
            break;
        } else {
            println!("Invalid choice! Please choose a number between 1 and 6.");
        }
    }
}