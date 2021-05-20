// Karate Chop
// http://codekata.com/kata/kata02-karate-chop/

#![allow(unused)]

use std::cmp::Ordering;

/// Imperative
fn chop1(needle: i32, haystack: &[i32]) -> Option<usize> {
    if haystack.len() == 0 {
        return None;
    }

    let mut low = 0;
    let mut high = haystack.len();

    loop {
        let pivot = (high - low) / 2 + low;
        let value = haystack[pivot];

        let (new_low, new_high) = match needle.cmp(&value) {
            Ordering::Less => (low, pivot),
            Ordering::Equal => return Some(pivot),
            Ordering::Greater => (pivot, high),
        };

        if new_low == low && new_high == high {
            return None;
        }

        low = new_low;
        high = new_high;
    }
}

/// Functional
fn chop2(needle: i32, haystack: &[i32]) -> Option<usize> {
    if haystack.len() == 0 {
        return None;
    }

    let pivot = haystack.len() / 2;
    let value = haystack[pivot];

    match needle.cmp(&value) {
        Ordering::Less => chop2(needle, &haystack[..pivot]),
        Ordering::Equal => Some(pivot),
        Ordering::Greater => chop2(needle, &haystack[pivot + 1..]).map(|i| i + pivot + 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chop1() {
        test_generic_chop(chop1);
    }

    #[test]
    fn test_chop2() {
        test_generic_chop(chop2);
    }

    fn test_generic_chop<F>(chop: F)
    where
        F: Fn(i32, &[i32]) -> Option<usize>,
    {
        assert_eq!(None, chop(3, &[]));
        assert_eq!(None, chop(3, &[1]));
        assert_eq!(Some(0), chop(1, &[1]));

        assert_eq!(Some(0), chop(1, &[1, 3, 5]));
        assert_eq!(Some(1), chop(3, &[1, 3, 5]));
        assert_eq!(Some(2), chop(5, &[1, 3, 5]));
        assert_eq!(None, chop(0, &[1, 3, 5]));
        assert_eq!(None, chop(2, &[1, 3, 5]));
        assert_eq!(None, chop(4, &[1, 3, 5]));
        assert_eq!(None, chop(6, &[1, 3, 5]));

        assert_eq!(Some(0), chop(1, &[1, 3, 5, 7]));
        assert_eq!(Some(1), chop(3, &[1, 3, 5, 7]));
        assert_eq!(Some(2), chop(5, &[1, 3, 5, 7]));
        assert_eq!(Some(3), chop(7, &[1, 3, 5, 7]));
        assert_eq!(None, chop(0, &[1, 3, 5, 7]));
        assert_eq!(None, chop(2, &[1, 3, 5, 7]));
        assert_eq!(None, chop(4, &[1, 3, 5, 7]));
        assert_eq!(None, chop(6, &[1, 3, 5, 7]));
        assert_eq!(None, chop(8, &[1, 3, 5, 7]));

        let long_list = &[
            1, 6, 7, 10, 13, 14, 17, 18, 21, 22, 23, 25, 26, 27, 30, 31, 35, 38, 39, 44, 47, 49,
            52, 53, 54, 55, 56, 58, 59, 60, 63, 64, 65, 66, 67, 69, 71, 74, 79, 81, 85, 87, 90, 95,
            97, 99, 100, 102, 103, 105, 107, 110, 112, 114, 117, 119, 120, 123, 124, 125, 129, 131,
            133, 134, 136, 137, 139, 140, 143, 144, 147, 148, 151, 153, 156, 157, 158, 161, 163,
            166, 171, 172, 174, 175, 177, 178, 183, 185, 187, 188, 189, 190, 191, 192, 193, 195,
            196, 197, 199, 200,
        ];

        assert_eq!(None, chop(2, long_list));
        assert_eq!(None, chop(198, long_list));
        assert_eq!(None, chop(101, long_list));
        assert_eq!(Some(0), chop(1, long_list));
        assert_eq!(Some(99), chop(200, long_list));
        assert_eq!(Some(7), chop(18, long_list));
        assert_eq!(Some(87), chop(185, long_list));
    }
}
