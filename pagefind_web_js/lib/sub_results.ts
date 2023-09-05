import { build_excerpt, calculate_excerpt_region } from "./excerpt";


export const calculate_sub_results = (fragment: PagefindSearchFragment, desired_excerpt_length: number): PagefindSubResult[] => {

    const anchors = fragment.anchors.filter(a => /h\d/i.test(a.element) && a.text?.length && /\w/.test(a.text)).sort((a, b) => a.location - b.location);
    const results: PagefindSubResult[] = [];

    let current_anchor_position = 0;
    let current_anchor: PagefindSubResult = {
        title: fragment.meta["title"],
        url: fragment.url,
        locations: [],
        excerpt: ""
    };

    const add_result = (end_range?: number) => {
        if (current_anchor.locations.length) {
            const relative_locations = current_anchor.locations.map(l => l - current_anchor_position);
            const excerpt_start = calculate_excerpt_region(relative_locations, desired_excerpt_length) + current_anchor_position;
            const excerpt_length = end_range ? Math.min((end_range - excerpt_start), desired_excerpt_length) : desired_excerpt_length;
            current_anchor.excerpt = build_excerpt(fragment, excerpt_start, excerpt_length, current_anchor.locations);

            results.push(current_anchor);
        }
    }

    for (let word of fragment.locations) {

        if (!anchors.length || word < anchors[0].location) {
            current_anchor.locations.push(word);
        } else {
            let next_anchor = anchors.shift()!;

            // Word is in a new sub result, track the previous one.
            add_result(next_anchor.location);

            while (anchors.length && word >= anchors[0].location) {
                next_anchor = anchors.shift()!;
            }
    
            current_anchor_position = next_anchor.location;
            current_anchor = {
                title: next_anchor.text!,
                url: `${fragment.url}#${next_anchor.id}`,
                anchor: next_anchor,
                locations: [word],
                excerpt: "" // TODO: Proper URL handling
            };
        }
    }
    add_result(anchors[0]?.location);
    
    return results;
}