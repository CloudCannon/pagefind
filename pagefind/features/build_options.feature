Feature: Build Options
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |

    Scenario: Source folder can be configured
        Given I have a "my_website/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "my_website/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program with the flags:
            | --source my_website |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "my_website/_pagefind/pagefind.js"
        When I serve the "my_website" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/"

    Scenario: Output path can be configured
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program with the flags:
            | --bundle-dir _search |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_search/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_search/pagefind.js");

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/"

    @skip
    Scenario: Selector used for indexing can be configured
