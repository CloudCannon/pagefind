Feature: Graceful Pagefind Errors
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """

    Scenario: Pagefind doesn't error on parsing ambiguities
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>hello world</h1>
            """
        Given I have a "public/dog/index.html" file with the body:
            """
            <h1>hello world</h1>
            <select><xmp><script>"use strict";</script></select>
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

                let search = await pagefind.search("world");
                let results = await Promise.all(search.results.map(r => r.data()));

                document.querySelector('[data-url]').innerText = results.map(r => r.url).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/, /dog/"
