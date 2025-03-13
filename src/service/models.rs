use crate::models::metrics::biathlon::{Accuracy, Biathlon};
use crate::models::metrics::running::Running;
use crate::models::metrics::weight_lifting::{LiftedWeight, Weight, WeightLifting};
use crate::models::metrics::{biathlon, running};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RunningPerformance {
    distance: f32,
    speed: f32,
}

impl Into<Running> for RunningPerformance {
    fn into(self) -> Running {
        Running::new(running::Distance(self.distance), running::Speed(self.speed))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiathlonPerformance {
    accuracy: f32,
    distance: f32,
    speed: f32,
}

impl Into<Biathlon> for BiathlonPerformance {
    fn into(self) -> Biathlon {
        Biathlon::new(
            Accuracy(self.accuracy),
            biathlon::Distance(self.distance),
            biathlon::Speed(self.speed),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeightLiftingPerformance {
    weight: f32,
    lifted_weight: f32,
}

impl Into<WeightLifting> for WeightLiftingPerformance {
    fn into(self) -> WeightLifting {
        WeightLifting::new(Weight(self.weight), LiftedWeight(self.lifted_weight))
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Id(pub i32);
