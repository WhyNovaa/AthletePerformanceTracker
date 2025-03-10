use std::any::Any;
use std::fmt::Debug;
use crate::traits::traits::Metric;

#[derive(Debug, Clone)]
pub struct Weight(pub f32);

impl Weight {
    pub fn weight(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct LiftedWeight(pub f32);

impl LiftedWeight {
    pub fn lifted_weight(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct WeightLifting {
    /// Own weight
    pub weight: Weight,
    /// Summary lifted weight
    pub lifted_weight: LiftedWeight,
}

impl WeightLifting {
    pub fn new(weight: Weight, lifted_weight: LiftedWeight) -> Self {
        Self {
            weight,
            lifted_weight,
        }
    }
}

/*impl Debug for WeightLifting {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Weight: {}, Lifted weight: {}", self.weight.0, self.lifted_weight.0)?;
        Ok(())
    }
}*/

impl Metric for WeightLifting {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Metric> {
        Box::new(self.clone())
    }
}

