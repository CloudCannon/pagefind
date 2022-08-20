Feature: Multisite Filters

    Background:
        Given I have a "root/index.html" file with the body:
            """
            <p data-result>Nothing</p>
            """
        Given I have a "root/website_a/hello/index.html" file with the body:
            """
            <h1>web world</h1>
            <span data-pagefind-filter="fruit">apple</span>
            <span data-pagefind-filter="color">red</span>
            """
        Given I have a "root/website_b/lorem/index.html" file with the body:
            """
            <h1>web ipsum</h1>
            <span data-pagefind-filter="fruit">banana</span>
            <span data-pagefind-filter="emote">happy</span>
            """
        When I run my program with the flags:
            | --source root/website_a |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "root/website_a/_pagefind/pagefind.js"
        When I run my program with the flags:
            | --source root/website_b |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "root/website_b/_pagefind/pagefind.js"
        When I serve the "root" directory
        When I load "/"

    Scenario: Pagefind can search across multiple sites with common filters
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/website_a/_pagefind/pagefind.js");
                await pagefind.mergeIndex("/website_b/_pagefind/");

                let search = await pagefind.search("web", {
                    filters: {
                        fruit: "apple"
                    }
                });

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/website_a/hello/"

    Scenario: Pagefind can search across multiple sites with unique filters
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/website_a/_pagefind/pagefind.js");
                await pagefind.mergeIndex("/website_b/_pagefind/");

                let search = await pagefind.search("web", {
                    filters: {
                        emote: "happy"
                    }
                });

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/website_b/lorem/"

    Scenario: Pagefind can search across multiple sites with synthetic filters
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/website_a/_pagefind/pagefind.js");
                pagefind.options({
                    indexFilter: {
                        site: "A"
                    }
                });
                await pagefind.mergeIndex("/website_b/_pagefind/", {
                    indexFilter: {
                        site: ["B", "C"]
                    }
                });

                let search_a = await pagefind.search("web", {
                    filters: {
                        site: "A"
                    }
                });
                let pages_a = await Promise.all(search_a.results.map(r => r.data()));

                let search_b = await pagefind.search("web", {
                    filters: {
                        site: "B"
                    }
                });
                let pages_b = await Promise.all(search_b.results.map(r => r.data()));

                document.querySelector('[data-result]').innerText = [
                    pages_a.map(p => p.url).join(", "),
                    pages_b.map(p => p.url).join(", "),
                ].join(' — ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/website_a/hello/ — /website_b/lorem/"
