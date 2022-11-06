Feature: Input Quirk Tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE  | public |
            | PAGEFIND_verbose | true   |
        Given I have a "public/index.html" file with the body:
            """
            <p data-title>Nothing</p>
            """

    Scenario: Index gzipped input files
        Given I have a gzipped "public/cat/index.html" file with the body:
            """
            <h1>Hello World</h1>
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

                let data = await search.results[0].data();
                document.querySelector('[data-title]').innerText = data.meta.title;
            }
            """
        Then There should be no logs
        Then The selector "[data-title]" should contain "Hello World"
