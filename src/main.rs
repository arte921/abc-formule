fn main() {
    for x in nulpunten(3.0, 4.0, -1.0) {
        println!("{:.2}", x);
    }
}

// ax² + bx + c
fn nulpunten (a: f32, b: f32, c: f32) -> Vec<f32> {
    let d = b.powi(2) - 4.0 * a * c;
    if d < 0.0 {
        vec![]
    } else if d == 0.0 {
        vec![
            -b / (2.0 * a)
        ]
    } else {
        vec![
            (-b + d.sqrt()) / (2.0 * a),
            (-b - d.sqrt()) / (2.0 * a)
        ]
    }
}
