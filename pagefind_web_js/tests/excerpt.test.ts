import test from 'ava';
import type * as external from "../types/index";

import { build_excerpt, calculate_excerpt_region } from "../lib/excerpt";

test('empty excerpt regions', t => {
    const weighted_words: PagefindWordLocation[] = []
    const excerpt_start = calculate_excerpt_region(weighted_words, 10);

	t.is(excerpt_start, 0);
});

test('short excerpt regions', t => {
    const weighted_words: PagefindWordLocation[] = [
        { weight: 1, location: 0},
        { weight: 1, location: 5},
    ]
    const excerpt_start = calculate_excerpt_region(weighted_words, 10);

	t.is(excerpt_start, 0);
});

test('single word excerpt', t => {
    const weighted_words: PagefindWordLocation[] = [
        { weight: 1, location: 30},
    ]
    const excerpt_start = calculate_excerpt_region(weighted_words, 30);

	t.is(excerpt_start, 15);
});

test('double word excerpt', t => {
    const weighted_words: PagefindWordLocation[] = [
        { weight: 1, location: 30},
        { weight: 1, location: 40},
    ]
    const excerpt_start = calculate_excerpt_region(weighted_words, 20);

	t.is(excerpt_start, 25);
});

test('better word excerpt', t => {
    const weighted_words: PagefindWordLocation[] = [
        { weight: 1, location: 30},
        { weight: 1, location: 40},
        { weight: 3, location: 90},
    ]
    const excerpt_start = calculate_excerpt_region(weighted_words, 20);

	t.is(excerpt_start, 80);
});

test('build standard excerpt', t => {
    const excerpt = build_excerpt("Donec id elit non mi porta gravida at eget metus.", 1, 5, [2]);

	t.is(excerpt, "id <mark>elit</mark> non mi porta");
});

test('build end excerpt', t => {
    const excerpt = build_excerpt("Donec id elit non mi porta gravida at eget metus.", 8, 5, [9]);

	t.is(excerpt, "porta gravida at eget <mark>metus.</mark>");
});

test('build endcapped excerpt', t => {
    const excerpt = build_excerpt("Nullam id dolor id nibh ultricies vehicula ut id elit. Donec sed odio dui.", 6, 6, [7], undefined, 10);

	t.is(excerpt, "nibh ultricies vehicula <mark>ut</mark> id elit.");
});

test('build narrowcapped excerpt', t => {
    const excerpt = build_excerpt("Nullam id dolor id nibh ultricies vehicula ut id elit. Donec sed odio dui.", 6, 6, [7], 6, 10);

	t.is(excerpt, "vehicula <mark>ut</mark> id elit.");
});

