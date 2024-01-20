use std::collections::HashSet;

pub fn dsc(left: &str, right: &str) -> f64 {
    if left == right {
        return 1f64;
    }

    if left.len() < 2 || right.len() < 2 {
        return 0f64;
    }

    let left_bigrams: HashSet<(char, char)> = bigrams(left);

    let right_bigrams: HashSet<(char, char)> = bigrams(right);

    let intersection_count: usize = left_bigrams.intersection(&right_bigrams).count();

    return 2f64 * intersection_count as f64 / (left_bigrams.len() + right_bigrams.len()) as f64;
}

fn bigrams(s: &str) -> HashSet<(char, char)> {
    return s.chars().zip(s.chars().skip(1)).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_bigrams() {
        let text: &str = "night";
        let actual: HashSet<(char, char)> = bigrams(text);
        let expected: HashSet<(char, char)> =
            HashSet::from([('n', 'i'), ('i', 'g'), ('g', 'h'), ('h', 't')]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_return_correct_coefficient() {
        assert_eq!(1f64, dsc("", ""));
        assert_eq!(1f64, dsc("foo", "foo"));
        assert_eq!(1f64, dsc("foo bar", "foo bar"));

        assert_eq!(0f64, dsc("foo", "bar"));
        assert_eq!(0f64, dsc("foo", ""));
        assert_eq!(0f64, dsc("", "bar"));

        assert_eq!(0.25f64, dsc("night", "nacht"));
        assert_eq!(0.8f64, dsc("healed", "sealed"));
    }
}
