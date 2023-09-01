Feature: Index Chunking

    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <ul>
                <li data-result>
            </ul>
            """

    # Scenario: Browser only loads chunks needed to search for the target word
    # Scenario: Chunk size is configurable

    Scenario: Searches that don't match a chunk will load the closest chunk
        Given I have a "public/one/index.html" file with the body:
            """
            <h1>Hello World</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("h");

                let results = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = results.map(r => r.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/one/"
