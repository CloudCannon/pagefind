@skip
Feature: Partial Matching

    Scenario: Search will return pages that match 2 out of 3 words
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>hello world</h1>
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

                let search = await pagefind.search("hello there world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/"

