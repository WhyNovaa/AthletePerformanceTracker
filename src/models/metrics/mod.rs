use crate::traits::traits::Metric;

pub mod biathlon;
pub mod running;
pub mod weightlifting;

impl Clone for Box<dyn Metric> {
    fn clone(&self) -> Box<dyn Metric> {
        self.clone_box()
    }
}
