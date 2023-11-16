Feature: Highlighting Search Tests

    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "public/a/index.html" file with the body:
            """
            <p>Hello World</p>
            """
        Given I have a "public/b/index.html" file with the body:
            """
            <h2 id="second">Second</h2>
            <p>Second Page</p>
            """

    Scenario: Query parameters can be inserted through the JS API
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");
                await pagefind.options({ highlightParam: "hi" });

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/a/?hi=world"

    Scenario: Multiple query parameters are inserted through the JS API
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");
                await pagefind.options({ highlightParam: "hi" });

                let search = await pagefind.search("hello world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/a/?hi=hello&amp;hi=world"

    Scenario: Query parameters don't conflict with subresult anchors
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");
                await pagefind.options({ highlightParam: "hi" });

                let search = await pagefind.search("second");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.sub_results[0].url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/b/?hi=second#second"