use std::fmt::Display;

use raylib::color::Color;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Iris {
    #[serde(rename = "Iris-setosa")]
    Setosa,
    #[serde(rename = "Iris-virginica")]
    Virginica,
    #[serde(rename = "Iris-versicolour")]
    Versicolour,
}

impl Display for Iris {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Iris::Setosa => "Setosa",
            Iris::Virginica => "Virginica",
            Iris::Versicolour => "Versicolour",
        };
        write!(f, "Iris {}", name)
    }
}

impl Into<f64> for Iris {
    fn into(self) -> f64 {
        match self {
            Iris::Setosa => 0.0,
            Iris::Virginica => 1.0,
            Iris::Versicolour => 1.0,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct IrisData {
    pub id: usize,
    pub sepal_length_cm: f64,
    pub sepal_width_cm: f64,
    pub petal_length_cm: f64,
    pub petal_width_cm: f64,
    pub species: Iris,
}

impl IrisData {
    pub fn read_csv() -> Vec<IrisData> {
        let data: Vec<IrisData> = csv::Reader::from_path("./Iris.csv")
            .unwrap()
            .deserialize()
            .map(|v| v.unwrap())
            .collect();
        data
    }

    pub fn get_data(data: &Vec<Self>, iris_type: Iris) -> Vec<Self> {
        let mut res = vec![];
        for d in data {
            if d.species == iris_type {
                res.push(*d);
            } else if d.species == Iris::Setosa {
                res.push(*d);
            }
        }
        res
    }

    pub fn get_graph_data(data: &Vec<Self>) -> Vec<(f32, f32, f32, f32, Color)> {
        let mut graph_data = vec![];
        for d in data {
            let sepal_length = d.sepal_length_cm as f32;
            let sepal_width = d.sepal_width_cm as f32;
            let petal_length = d.petal_length_cm as f32;
            let petal_width = d.petal_width_cm as f32;
            let color = match d.species {
                Iris::Setosa => Color::RED,
                Iris::Virginica => Color::GREEN,
                Iris::Versicolour => Color::BLUE,
            };
            graph_data.push((sepal_length, sepal_width, petal_length, petal_width, color));
        }
        graph_data
    }

    pub fn get_test_data(data: &Vec<Self>, count: usize) -> Vec<(Vec<f64>, Iris)> {
        let mut test_data = vec![];
        let mut setosa_count = 0;
        let mut other_count = 0;

        for d in data {
            let sepal_length = d.sepal_length_cm;
            let sepal_width = d.sepal_width_cm;
            let petal_length = d.petal_length_cm;
            let petal_width = d.petal_width_cm;
            if d.species == Iris::Setosa && setosa_count < count {
                setosa_count += 1;
                test_data.push((
                    vec![sepal_length, sepal_width, petal_length, petal_width],
                    d.species,
                ));
            } else if other_count < count {
                other_count += 1;
                test_data.push((
                    vec![sepal_length, sepal_width, petal_length, petal_width],
                    d.species,
                ));
            }

            if setosa_count == count && other_count == count {
                break;
            }
        }

        test_data
    }
}
