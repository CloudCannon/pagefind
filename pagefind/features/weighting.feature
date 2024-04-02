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
            <p>Antelope Antelope Antelope Notantelope</p>
            <p>Other text again</p>
            """
        Given I have a "public/r3/index.html" file with the body:
            """
            <h5>Antelope</h5>
            <p>Antelope Antelope Antelope Notantelope</p>
            <p>Other text again</p>
            """
        Given I have a "public/r4/index.html" file with the body:
            """
            <h1>Antelope</h1>
            <p>Other text, totalling eight words of content</p>
            """
        Given I have a "public/r5/index.html" file with the body:
            """
            <h3>Antelope</h3>
            <p>Other antelope antelope text, of a similar length</p>
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
            <p>Two references to SomeLongFiveWordAntelope SomeLongFiveWordAntelope</p>
            """
        Given I have a "public/r3/index.html" file with the body:
            """
            <p>A single reference to the TwoAntelope</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");
                await pagefind.options({
                    ranking: {
                        termFrequency: 0.0
                    }
                });

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
                let weights = data.weighted_locations.map(l => `weight:${l.weight}/bal:${l.balanced_score.toFixed(2)}/loc:${l.location}`).join(' • ');
                document.querySelector('p').innerText = weights;
            }
            """
        Then There should be no logs
        # Treat the bal value here as a snapshot — update the expected value as needed
        Then The selector "p" should contain "weight:0.5/bal:128.04/loc:4"

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
                let weights = data.weighted_locations.map(l => `weight:${l.weight}/bal:${l.balanced_score.toFixed(2)}/loc:${l.location}`).join(' • ');
                document.querySelector('p').innerText = weights;
            }
            """
        Then There should be no logs
        # Treat the bal value here as a snapshot — update the expected value as needed
        Then The selector "p" should contain "weight:1/bal:512.14/loc:4"

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
                let weights = data.weighted_locations.map(l => `weight:${l.weight}/bal:${l.balanced_score.toFixed(2)}/loc:${l.location}`).join(' • ');
                document.querySelector('p').innerText = weights;
            }
            """
        Then There should be no logs
        # Treat the bal value here as a snapshot — update the expected value as needed
        Then The selector "p" should contain "weight:1/bal:212.36/loc:4"
