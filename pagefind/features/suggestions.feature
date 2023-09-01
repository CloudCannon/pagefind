Feature: Spellcheck

    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <ul>
                <li data-result>
            </ul>
            """

    Scenario: Search results will be returned for all extensions of the word
        Given I have a "public/one/index.html" file with the body:
            """
            <h1>Hello World</h1>
            """
        Given I have a "public/two/index.html" file with the body:
            """
            <h1>Hello Wow</h1>
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

                let search = await pagefind.search("w");

                let results = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = results.map(r => r.url).join(', ');
            }
            """
        Then There should be no logs
        # It should have prioritised the shorter word extension in the results
        Then The selector "[data-result]" should contain "/two/, /one/"

    @skip
    Scenario: Spelling correction can be returned for the unique words in the dataset
