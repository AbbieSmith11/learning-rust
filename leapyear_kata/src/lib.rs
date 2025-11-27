pub fn is_leap(year: i32) -> bool {
    let result: bool;

    if is_divisible_by_400(year) {
        result = true;
    } else if is_divisible_by_100(year) && !is_divisible_by_400(year) {
        result = false;
    } else if is_divisible_by_4(year) {
        result = true;
    } else {
        result = false;
    }
    println!("{}", result);
    return result;
}

pub fn is_divisible_by_4(year: i32) -> bool {
    let result_divided_by_four = year % 4;
    return result_divided_by_four == 0;
}

pub fn is_divisible_by_100(year: i32) -> bool {
    let result_divided_by_100 = year % 100;
    return result_divided_by_100 == 0;
}

pub fn is_divisible_by_400(year: i32) -> bool {
    let result_divided_by_400 = year % 400;
    return result_divided_by_400 == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_divisible_by_four() {
        let result = is_leap(1997);
        assert_eq!(result, false);
    }

    #[test]
    fn divisible_by_four() {
        let result = is_leap(1996);
        assert_eq!(result, true);
    }

    #[test]
    fn divisible_by_400() {
        let result = is_leap(1600);
        assert_eq!(result, true);
    }

    #[test]
    fn divisible_by_100_and_not_by_400() {
        let result = is_leap(1800);
        assert_eq!(result, false);
    }
}
