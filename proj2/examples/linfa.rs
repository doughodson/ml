use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::prelude::*;

fn main() {
    let data = array![
        [1.0, 2.0],
        [1.5, 1.8],
        [5.0, 8.0],
        [6.0, 9.0]
    ];

    let targets = array![0, 0, 1, 1];

    let dataset = Dataset::new(data, targets);

    let model = LogisticRegression::default().fit(&dataset).unwrap();

    let new_points = array![[2.0, 2.0], [6.0, 8.0]];
    let predictions = model.predict(&new_points);

    println!("Predictions: {:?}", predictions);
}