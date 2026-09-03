fn main() {
    let toshiba: f64 = 450_000.0;
    let mac: f64 = 1_500_000.0;
    let hp: f64 = 750_000.0;
    let dell: f64 = 2_850_000.0;
    let acer: f64 = 250_000.0;

    // Sum
    let s = toshiba + mac + hp + dell + acer;

    // Average
    let a = s / 10.0;

    println!("Sum of sales is {:.0}", s);
    println!("Average of sales is {:.0}", a);
}