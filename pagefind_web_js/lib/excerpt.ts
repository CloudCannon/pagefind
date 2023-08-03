

export const calculate_excerpt_region = (word_positions: number[], excerpt_length: number): number => {
    const start_distance = Math.floor(excerpt_length / 3);
    if (word_positions.length === 0) {
        return 0;
    }
    if (word_positions.length === 1) {
        return Math.max((word_positions[0] - start_distance), 0);
    }

    let window_start = 0;
    let pair_dist = word_positions[word_positions.length - 1];

    for (let i = 0; i < word_positions.length - 1; i += 1) {
        let [p1, p2] = [word_positions[i], word_positions[i+1]];
        let dist = p2 - p1;

        if (dist < pair_dist) {
            pair_dist = dist;
            window_start = Math.max((p1 - start_distance), 0);
        }
    }

    return window_start;
}

export const build_excerpt = (fragment: PagefindSearchFragment, start: number, length: number, locations: number[]): string => {
    const content = fragment.raw_content ?? "";
    let is_zws_delimited = content.includes('\u200B');
    let fragment_words: string[] = [];
    if (is_zws_delimited) {
        // If segmentation was run on the backend, count words by ZWS boundaries
        fragment_words = content.split('\u200B');
    } else {
        fragment_words = content.split(/[\r\n\s]+/g);
    }
    for (let word of locations) {
        fragment_words[word] = `<mark>${fragment_words[word]}</mark>`;
    }
    return fragment_words.slice(start, start + length).join(is_zws_delimited ? '' : ' ').trim();
}

// TODO: Unit test this file
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn excerpts() {
//         let words = vec![1, 5, 7, 45, 46, 60];
//         assert_eq!(calculate_excerpt(&words, 6), 43);

//         let words = vec![99, 334, 448, 489, 4366, 4378];
//         assert_eq!(calculate_excerpt(&words, 6), 4364);
//     }
// }
