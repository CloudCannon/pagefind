Feature: Multisite Result Scoring

    Background:
        Given I have a "root/index.html" file with the body:
            """
            <p data-result>Nothing</p>
            """
        Given I have a "root/website_a/twowebs/index.html" file with the body:
            """
            <h1>my page on the web web</h1>
            """
        Given I have a "root/website_b/oneweb/index.html" file with the body:
            """
            <h1>my page on the web</h1>
            """
        Given I have a "root/website_b/threewebs/index.html" file with the body:
            """
            <h1>my page on the web web web</h1>
            """

    Scenario: Pages are scored correctly across indexes
        When I run my program with the flags:
            | --site root/website_a |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "root/website_a/pagefind/pagefind.js"
        When I run my program with the flags:
            | --site root/website_b |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "root/website_b/pagefind/pagefind.js"
        When I serve the "root" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/website_a/pagefind/pagefind.js");
                await pagefind.mergeIndex("/website_b/pagefind/");

                let search = await pagefind.search("web");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/website_b/threewebs/, /website_a/twowebs/, /website_b/oneweb/"

    Scenario: Multiple indexes can be weighted separately
        When I run my program with the flags:
            | --site root/website_a |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "root/website_a/pagefind/pagefind.js"
        When I run my program with the flags:
            | --site root/website_b |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "root/website_b/pagefind/pagefind.js"
        When I serve the "root" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/website_a/pagefind/pagefind.js");
                await pagefind.mergeIndex("/website_b/pagefind/", {
                    indexWeight: 2
                });

                let search = await pagefind.search("web");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/website_b/threewebs/, /website_b/oneweb/, /website_a/twowebs/"
