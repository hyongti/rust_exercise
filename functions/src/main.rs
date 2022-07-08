fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    let area = area_of(width, height);

    println!("Area is {}", area);

    println!("Volume is {}", volume(width, height, depth));
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    // tail expression; 세미콜론 X
    x * y * z
}

fn area_of(x: i32, y: i32) -> i32 {
    // with return; 세미콜론 O
    return x * y;
}