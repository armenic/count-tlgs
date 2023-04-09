pub fn group_tlg(file_name: &str) -> String {
    let first_two_chars: String = file_name.chars().take(2).collect();

    let temp = match first_two_chars.as_str() {
        "t_" => "table",
        "l_" => "listing",
        "g_" => "graph",
        _ => "other",
    };

    String::from(temp)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(group_tlg("l_abc.out"), "listing");
        assert_eq!(group_tlg("t_abc.out"), "table");
        assert_eq!(group_tlg("g_abc.out"), "graph");
        assert_eq!(group_tlg("ll_abc.out"), "other");
    }
}

use std::collections::HashMap;

pub fn my_hash() -> String {
    let dirs = ["a", "b", "c"];
    let mut outer_map = HashMap::new();

    let text = "one one two three";

    for dir in dirs {
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        outer_map.entry(dir).or_insert(map);
    }

    println!("{:?}", outer_map);

    String::from("baba")
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_has() {
        assert_eq!(my_hash(), "baba")
    }
}
