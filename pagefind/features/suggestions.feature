Feature: Spellcheck

    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <ul>
                <li data-result>
            </ul>
            """

    Scenario: Search results will be returned for the closest extention of the word
        Given I have a "public/basic/index.html" file with the body:
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

                let search = await pagefind.search("w");

                let data = await search.results[0].data();
                document.querySelector('[data-result]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/basic/"

    @skip
    Scenario: Spelling correction can be returned for the unique words in the dataset
