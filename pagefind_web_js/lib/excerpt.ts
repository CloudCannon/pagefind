

export const calculate_excerpt_region = (word_positions: PagefindWordLocation[], excerpt_length: number): number => {
    if (word_positions.length === 0) {
        return 0;
    }

    let words: number[] = [];
    for (const word of word_positions) {
        words[word.location] = words[word.location] || 0;
        words[word.location] += word.weight;
    }

    if (words.length <= excerpt_length) {
        return 0;
    }

    let densest = words.slice(0, excerpt_length).reduce((partialSum, a) => partialSum + a, 0);
    let working_sum = densest;
    let densest_at: number[] = [0];

    for (let i = 0; i < words.length; i++) {
        const boundary = i + excerpt_length;
        working_sum += (words[boundary] ?? 0) - (words[i] ?? 0);
        if (working_sum > densest) {
            densest = working_sum;
            densest_at = [i];
        } else if (working_sum === densest && densest_at[densest_at.length -1] === i - 1) {
            densest_at.push(i);
        }
    }

    let midpoint = densest_at[Math.floor(densest_at.length / 2)];
    return midpoint;
}

export const build_excerpt = (content: string, start: number, length: number, locations: number[], not_before?: number, not_from?: number): string => {
    let is_zws_delimited = content.includes('\u200B');
    let fragment_words: string[] = [];
    if (is_zws_delimited) {
        // If segmentation was run on the backend, count words by ZWS boundaries
        fragment_words = content.split('\u200B');
    } else {
        fragment_words = content.split(/[\r\n\s]+/g);
    }
    for (let word of locations) {
        if (fragment_words[word]?.startsWith(`<mark>`)) {
            // It's possible to have a word come up as multiple search hits
            continue;
        }
        fragment_words[word] = `<mark>${fragment_words[word]}</mark>`;
    }

    let endcap = not_from ?? fragment_words.length;
    let startcap = not_before ?? 0;

    if (endcap - startcap < length) {
        length = endcap - startcap;
    }

    if (start + length > endcap) {
        start = endcap - length;
    }
    if (start < startcap) {
        start = startcap;
    }

    return fragment_words.slice(start, start + length).join(is_zws_delimited ? '' : ' ').trim();
}
