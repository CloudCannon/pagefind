Feature: Debounced Searches
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-types>Nothing</p>
            <p data-last>Nothing</p>
            """

    Scenario: Debounce repeated search calls
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await Promise.all([
                    pagefind.debouncedSearch("a"),
                    pagefind.debouncedSearch("w"),
                    pagefind.debouncedSearch("wo"),
                    pagefind.debouncedSearch("wor"),
                    pagefind.debouncedSearch("worl")
                ]);

                document.querySelector('[data-types]').innerText = results.map(r => (r === null ? "null" : r.results.length)).join(', ');

                let pages = await Promise.all(results[4].results.map(r => r.data()));
                document.querySelector('[data-last]').innerText = pages.map(p => p.url).sort().join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-types]" should contain "null, null, null, null, 1"
        Then The selector "[data-last]" should contain "/cat/"
