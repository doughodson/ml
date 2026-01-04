use ndarray::prelude::*;

fn main() {
    let a = array![
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0]
    ];
    let b = array![
        [10.0, 20.0, 30.0],
        [40.0, 50.0, 60.0]
    ];

    let sum = &a + &b;
    let product = a.dot(&b.t());

    println!("Sum:\n{:?}", sum);
    println!("Product:\n{:?}", product);
}
