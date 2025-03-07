use std::any::Any;
use std::fmt::Debug;
use crate::traits::traits::Metric;

#[derive(Debug, Clone)]
pub struct Accuracy(pub f32);

impl Accuracy {
    pub fn accuracy(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Distance(pub f32);

impl Distance {
    pub fn dist(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Speed(pub f32);

impl Speed {
    pub fn speed(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Biathlon {
    /// Shooting accuracy
    pub accuracy: Accuracy,
    /// Distance in km
    pub distance: Distance,
    /// Speed in km per hour
    pub speed: Speed,
}

impl Biathlon {
    pub fn new(accuracy: Accuracy, distance: Distance, speed: Speed) -> Self {
        Self {
            accuracy,
            distance,
            speed,
        }
    }
}

/*impl Debug for Biathlon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Accuracy: {}, Distance: {}, Speed: {}", self.accuracy.0, self.distance.0, self.speed.0)?;
        Ok(())
    }
}*/

impl Metric for Biathlon {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Metric> {
        Box::new(self.clone())
    }
}
