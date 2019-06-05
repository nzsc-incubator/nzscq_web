use super::lerp::Lerper;
use crate::paint::Component;

pub trait Switch {
    fn case(self, completion_factor: f64) -> Option<Vec<Component>>;
}

pub struct Switch4<A, B, C, D>(
    pub (std::ops::Range<f64>, A),
    pub (std::ops::Range<f64>, B),
    pub (std::ops::Range<f64>, C),
    pub (std::ops::RangeInclusive<f64>, D),
)
where
    A: FnOnce(Lerper) -> Vec<Component>,
    B: FnOnce(Lerper) -> Vec<Component>,
    C: FnOnce(Lerper) -> Vec<Component>,
    D: FnOnce(Lerper) -> Vec<Component>;

impl<A, B, C, D> Switch for Switch4<A, B, C, D>
where
    A: FnOnce(Lerper) -> Vec<Component>,
    B: FnOnce(Lerper) -> Vec<Component>,
    C: FnOnce(Lerper) -> Vec<Component>,
    D: FnOnce(Lerper) -> Vec<Component>,
{
    fn case(self, completion_factor: f64) -> Option<Vec<Component>> {
        let lerper = Lerper::from_completion_factor(completion_factor);

        if (self.0).0.contains(&completion_factor) {
            let lerper = lerper.sub_lerper((self.0).0);
            Some((self.0).1(lerper))
        } else if (self.1).0.contains(&completion_factor) {
            let lerper = lerper.sub_lerper((self.1).0);
            Some((self.1).1(lerper))
        } else if (self.2).0.contains(&completion_factor) {
            let lerper = lerper.sub_lerper((self.2).0);
            Some((self.2).1(lerper))
        } else if (self.3).0.contains(&completion_factor) {
            let lerper = lerper.sub_lerper((self.3).0);
            Some((self.3).1(lerper))
        } else {
            None
        }
    }
}
