use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum FlipVersion {
    A,
    B,
    Control
}

#[derive(Debug)]
pub struct FlipCoin {
    a: i32,
    b: i32,
    control: i32
}

impl FlipCoin {
    pub fn new(a: i32, b: i32, control: i32) -> Self {
        let sum = a + b + control;

        if sum > 100 {
            panic!("Error");
        }

        Self {
            a,
            b,
            control,
        }
    }

    pub fn play(&self) -> FlipVersion {
        let mut rng = rand::thread_rng();
        let flip = rng.gen_range(1..100);
        let start_a = 1;
        let end_a = self.a;
        let start_b = self.a + 1;
        let end_b = self.a + self.b;
        let start_c = end_b + 1;
        let end_c = end_b + self.control;

        if self.is_between(flip, start_a, end_a) {
            FlipVersion::A
        } else if self.is_between(flip, start_b, end_b) {
            FlipVersion::B
        } else if self.is_between(flip, start_c, end_c) {
            FlipVersion::Control
        } else {
            FlipVersion::A
        }
    }

    fn is_between(&self, value: i32, min: i32, max: i32) -> bool {
        max != 0 && value >= min && value <= max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ab_test = FlipCoin::new(25, 50, 25).play();

        println!("{:?}", ab_test);
    }

    #[test]
    fn force_a() {
        let ab_test = FlipCoin::new(100, 0 , 0).play();

        assert_eq!(ab_test, FlipVersion::A);
    }

    #[test]
    fn force_b() {
        let ab_test = FlipCoin::new(0, 100 , 0).play();

        assert_eq!(ab_test, FlipVersion::B);
    }
}
