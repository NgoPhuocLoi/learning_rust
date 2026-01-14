fn add_two(num: u64) -> u64 {
    num + 2
}

fn divided(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        panic!("Dividid by 0");
    }
    num1 / num2
}

mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let result = add_two(10);
        assert_eq!(result, 12);
    }

    #[test]
    fn make_this_test_fail() {
        panic!("It fails");
    }

    #[test]
    fn divided_should_pass() {
        let a = 10.0;
        let b = 2.5;
        let result = divided(a, b);
        assert_eq!(result, 4.0);
    }

    #[test]
    #[should_panic]
    fn divided_by_0_shoud_panic() {
        divided(10.0, 0.0);
    }
}

fn main() {}
