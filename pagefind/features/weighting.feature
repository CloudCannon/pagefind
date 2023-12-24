Feature: Word Weighting
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p>no results</p>
            """

    Scenario: Headings are automatically favoured over standard text
        Given I have a "public/r1/index.html" file with the body:
            """
            <p>Antelope</p>
            <p>Antelope Antelope Antelope Antelope</p>
            <p>Other text again</p>
            """
        Given I have a "public/r2/index.html" file with the body:
            """
            <p>Antelope</p>
            <p>Antelope Antelope Antelope</p>
            <p>Other text again</p>
            """
        Given I have a "public/r3/index.html" file with the body:
            """
            <h6>Antelope</h6>
            <p>Antelope Antelope Antelope</p>
            <p>Other text again</p>
            """
        Given I have a "public/r4/index.html" file with the body:
            """
            <h1>Antelope</h1>
            <p>Other text</p>
            """
        Given I have a "public/r5/index.html" file with the body:
            """
            <h2>Antelope</h2>
            <p>Other text again</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search(`antelope`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('p').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "p" should contain "/r4/, /r5/, /r3/, /r1/, /r2/"

    Scenario: Text can be explicitly weighted higher
        Given I have a "public/r1/index.html" file with the body:
            """
            <p>Antelope</p>
            <p>Antelope Antelope Not</p>
            """
        Given I have a "public/r2/index.html" file with the body:
            """
            <p>Antelope</p>
            <p>Antelope Not</p>
            """
        Given I have a "public/r3/index.html" file with the body:
            """
            <p data-pagefind-weight="3">Antelope</p>
            <p>Antelope Not</p>
            """
        Given I have a "public/r4/index.html" file with the body:
            """
            <p>Antelope</p>
            <p>Antelope Antelope Antelope Antelope</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search(`antelope`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('p').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "p" should contain "/r3/, /r4/, /r1/, /r2/"

    Scenario: Text can be explicitly weighted lower
        Given I have a "public/r1/index.html" file with the body:
            """
            <p data-pagefind-weight="0.1">Antelope Antelope all about Antelope</p>
            <p>More text about other stuff</p>
            """
        Given I have a "public/r2/index.html" file with the body:
            """
            <p>Five words that aren't related</p>
            <p>One solitary mention of antelope</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search(`antelope`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('p').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "p" should contain "/r2/, /r1/"

    Scenario: Compound words are implicitly weighted lower
        Given I have a "public/r1/index.html" file with the body:
            """
            <p>A single reference to antelope</p>
            """
        Given I have a "public/r2/index.html" file with the body:
            """
            <p>Two references to ThreeWordAntelope ThreeWordAntelope</p>
            """
        Given I have a "public/r3/index.html" file with the body:
            """
            <p>Three of TwoAntelope TwoAntelope TwoAntelope</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search(`antelope`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('p').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "p" should contain "/r1/, /r3/, /r2/"

    Scenario: Compound words prefixes prioritise the lower weight
        Given I have a "public/r1/index.html" file with the body:
            """
            <p>A single reference to ThreeAntelopes</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search(`three`);
                let data = await search.results[0].data();
                let weights = data.weighted_locations.map(l => `weight:${l.weight}/bal:${l.balanced_score}/loc:${l.location}`).join(' • ');
                document.querySelector('p').innerText = weights;
            }
            """
        Then There should be no logs
        # Treat the bal value here as a snapshot — update the expected value as needed
        Then The selector "p" should contain "weight:0.5/bal:18/loc:4"

    Scenario: Compound words sum to a full weight
        Given I have a "public/r1/index.html" file with the body:
            """
            <p>A single reference to ThreeAntelopes</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search(`three antelopes`);
                let data = await search.results[0].data();
                let weights = data.weighted_locations.map(l => `weight:${l.weight}/bal:${l.balanced_score}/loc:${l.location}`).join(' • ');
                document.querySelector('p').innerText = weights;
            }
            """
        Then There should be no logs
        # Treat the bal value here as a snapshot — update the expected value as needed
        Then The selector "p" should contain "weight:1/bal:72/loc:4"

    Scenario: Compound words matched as full words use the full weight
        Given I have a "public/r1/index.html" file with the body:
            """
            <p>A single reference to ThreeAntelopes</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search(`threea`);
                let data = await search.results[0].data();
                let weights = data.weighted_locations.map(l => `weight:${l.weight}/bal:${l.balanced_score}/loc:${l.location}`).join(' • ');
                document.querySelector('p').innerText = weights;
            }
            """
        Then There should be no logs
        # Treat the bal value here as a snapshot — update the expected value as needed
        Then The selector "p" should contain "weight:1/bal:82.28572/loc:4"

    Scenario: Density weighting can be turned off
        Given I have a "public/single-word.html" file with the body:
            """
            <p>word</p>
            """
        Given I have a "public/three-words.html" file with the body:
            """
            <p>I have a word and a word and another word</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search(`word`);
                let search2 = await pagefind.search(`word`, { ranking: { pageFrequency: 0.0 } });
                let counts = [search, search2].map(s => s.results.map(r => r.words.length));
                document.querySelector('p').innerText = JSON.stringify(counts);
            }
            """
        Then There should be no logs
        # With density weighting, single-word should be the first hit, otherwise three-words
        Then The selector "p" should contain "[[1,3],[3,1]]"
