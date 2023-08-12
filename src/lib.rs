use std::cmp::{max, min};

pub fn gcd(a: u64, b: u64) -> u64 {
    let (mut g1, mut g2) = (max(a, b), min(a, b));
    while g2 != 0 {
        (g1, g2) = (g2, g1 % g2);
    }
    g1
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_12_15() {
        let result = gcd(12, 15);
        assert_eq!(result, 3);
    }

    #[test]
    fn gcd_15_12() {
        let result = gcd(15, 12);
        assert_eq!(result, 3);
    }

    #[test]
    fn gcd_7_4() {
        let result = gcd(7, 4);
        assert_eq!(result, 1);
    }

    #[test]
    fn gcd_270_192() {
        let result = gcd(270, 192);
        assert_eq!(result, 6);
    }

    #[test]
    fn gcd_7469_2464() {
        let result = gcd(7469, 2464);
        assert_eq!(result, 77);
    }

    #[test]
    fn gcd_55290_115430() {
        let result = gcd(55290, 115430);
        assert_eq!(result, 970);
    }

    #[test]
    fn lcm_55290_115430() {
        let result = lcm(55290, 115430);
        assert_eq!(result, 6579510);
    }

    #[test]
    fn lcm_7469_2464() {
        let result = lcm(7469, 2464);
        assert_eq!(result, 239008);
    }

    #[test]
    fn lcm_270_192() {
        let result = lcm(270, 192);
        assert_eq!(result, 8640);
    }

    #[test]
    fn lcm_7_4() {
        let result = lcm(7, 4);
        assert_eq!(result, 28);
    }

    #[test]
    fn lcm_12_12() {
        let result = lcm(12, 12);
        assert_eq!(result, 12);
    }

    #[test]
    fn lcm_12_18() {
        let result = lcm(12, 18);
        assert_eq!(result, 36);
    }
}
