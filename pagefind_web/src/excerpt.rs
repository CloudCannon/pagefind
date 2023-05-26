use crate::util::*;

// TODO: MVP — Implement something smarter
pub fn calculate_excerpt(word_positions: &[u32], excerpt_length: u32) -> u32 {
    debug!({
        format! {"Calculating a {} word excerpt for the word positions {:#?}", excerpt_length, word_positions}
    });
    let start_distance = excerpt_length / 3;
    if word_positions.is_empty() {
        return 0;
    }
    if word_positions.len() < 2 {
        return word_positions[0]
            .checked_sub(start_distance)
            .unwrap_or_default();
    }

    let mut window_start = 0;
    let mut pair_dist = *word_positions.last().unwrap();

    for pair in word_positions.windows(2) {
        let dist = pair[1] - pair[0];
        if dist < pair_dist {
            pair_dist = dist;
            window_start = pair[0].checked_sub(start_distance).unwrap_or_default();
        }
    }
    debug!({
        format! {"Best excerpt starts at {window_start}"}
    });
    window_start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excerpts() {
        let words = vec![1, 5, 7, 45, 46, 60];
        assert_eq!(calculate_excerpt(&words, 6), 43);

        let words = vec![99, 334, 448, 489, 4366, 4378];
        assert_eq!(calculate_excerpt(&words, 6), 4364);
    }
}
