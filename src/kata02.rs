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

/// Recursive Imperative
fn chop3(needle: i32, haystack: &[i32]) -> Option<usize> {
    fn chop3_rec(needle: i32, haystack: &[i32], first: usize, last: usize) -> Option<usize> {
        if last < first {
            return None;
        }

        let pivot = (last - first) / 2 + first;

        match needle.cmp(&haystack[pivot]) {
            Ordering::Less => {
                if pivot > 0 {
                    chop3_rec(needle, haystack, first, pivot - 1)
                } else {
                    None
                }
            }
            Ordering::Equal => Some(pivot),
            Ordering::Greater => chop3_rec(needle, haystack, pivot + 1, last),
        }
    }

    if haystack.len() == 0 {
        return None;
    }

    chop3_rec(needle, haystack, 0, haystack.len() - 1)
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

    #[test]
    fn test_chop3() {
        test_generic_chop(chop3);
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

        // 2 mutually-exclusive lists
        // generated by https://www.random.org/sequences/
        let long_list1 = &[
            1, 3, 4, 5, 6, 7, 9, 10, 12, 13, 14, 15, 16, 17, 18, 20, 24, 25, 27, 29, 32, 33, 34,
            36, 37, 39, 40, 41, 42, 43, 44, 46, 47, 50, 52, 55, 60, 61, 63, 65, 66, 68, 69, 70, 76,
            77, 79, 80, 83, 84, 87, 90, 93, 99, 100, 101, 103, 105, 107, 108, 111, 112, 113, 115,
            124, 127, 130, 139, 140, 141, 142, 145, 146, 151, 152, 155, 157, 159, 160, 161, 163,
            164, 168, 169, 170, 171, 172, 174, 176, 178, 185, 186, 190, 191, 193, 194, 196, 197,
            198, 200,
        ];

        let long_list2 = &[
            2, 8, 11, 19, 21, 22, 23, 26, 28, 30, 31, 35, 38, 45, 48, 49, 51, 53, 54, 56, 57, 58,
            59, 62, 64, 67, 71, 72, 73, 74, 75, 78, 81, 82, 85, 86, 88, 89, 91, 92, 94, 95, 96, 97,
            98, 102, 104, 106, 109, 110, 114, 116, 117, 118, 119, 120, 121, 122, 123, 125, 126,
            128, 129, 131, 132, 133, 134, 135, 136, 137, 138, 143, 144, 147, 148, 149, 150, 153,
            154, 156, 158, 162, 165, 166, 167, 173, 175, 177, 179, 180, 181, 182, 183, 184, 187,
            188, 189, 192, 195, 199,
        ];

        for (i, x) in long_list1.iter().enumerate() {
            assert_eq!(Some(i), chop(*x, long_list1), "i: {}, x: {}", i, x);
            assert_eq!(None, chop(*x, long_list2), "i: {}, x: {}", i, x);
        }

        for (i, x) in long_list2.iter().enumerate() {
            assert_eq!(Some(i), chop(*x, long_list2), "i: {}, x: {}", i, x);
            assert_eq!(None, chop(*x, long_list1), "i: {}, x: {}", i, x);
        }
    }
}
