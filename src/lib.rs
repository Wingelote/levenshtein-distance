pub fn levenshtein(s1: &str, s2: &str) -> usize {
    let mut s_vec: Vec<&str> = vec![s2, s1];
    s_vec.sort_by(|a, b| a.len().cmp(&b.len()));

    let mut distances: Vec<usize> = (0..s_vec[0].len()+1).collect();

    for (i2, c2) in s_vec[1].chars().enumerate() {
        let mut new_distances = vec![i2+1];

        for (i1, c1) in s_vec[0].chars().enumerate() {
            if c1 == c2 { new_distances.push(distances[i1]); }
            else { 
                new_distances.push(
                    1 + vec![
                        distances[i1], distances[i1+1],
                        *distances.iter().last().unwrap_or(&0)
                    ].iter().min().unwrap_or(&0)
                );
            }
        }
        distances = new_distances;
    }
    *distances.iter().last().unwrap()
}


#[cfg(test)]
mod levenshtein_test {
    use super::*;

    #[test]
    fn distance_of_0() {
        assert_eq!(levenshtein("aBc", "aBc"), 0);
        assert_eq!(levenshtein("àè ", "àè "), 0);
        assert_eq!(levenshtein("123", "123"), 0);
    }

    #[test]
    fn distance_of_3() {
        assert_eq!(levenshtein("aBc", "cba"), 3);
        assert_eq!(levenshtein(" àè", "àè "), 3);
        assert_eq!(levenshtein("123", "342"), 3);
    }

    #[test]
    fn empty_left() {
        assert_eq!(levenshtein("", "a"), 1);
    }
    
    #[test]
    fn empty_right() {
        assert_eq!(levenshtein("a", ""), 1);
    }

    #[test]
    fn empty_both() {
        assert_eq!(levenshtein("", ""), 0);
    }
}
