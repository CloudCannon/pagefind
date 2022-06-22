Feature: Search Options

    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |

    Scenario: Base URL can be configured
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
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
                pagefind.options({
                    baseUrl: "/docs/"
                });

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/docs/cat/"

    Scenario: Base URL auto-detects the default directory being moved
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program with the flags:
            | --bundle-dir blog/_pagefind |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/blog/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/blog/_pagefind/pagefind.js");

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/blog/cat/"
