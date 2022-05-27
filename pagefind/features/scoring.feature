Feature: Result Scoring
    Background:
        Given I have a "public/index.html" file with the body:
            """
            <ul>
                <li data-count>
                <li data-result>
            </ul>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>Happy cat post, that later mentions dogs in the context of cats</h1>
            """
        Given I have a "public/dog/index.html" file with the body:
            """
            <h1>A post about dogs vs cats (but mainly dogs)</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"

    Scenario: Search results are ranked by word frequency
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search(`cat`);

                document.querySelector('[data-count]').innerText = `${search.results.length} result(s)`;
                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-count]" should contain "2 result(s)"
        Then The selector "[data-result]" should contain "/cat/, /dog/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search(`dog`);

                document.querySelector('[data-count]').innerText = `${search.results.length} result(s)`;
                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-count]" should contain "2 result(s)"
        Then The selector "[data-result]" should contain "/dog/, /cat/"

    @skip
    Scenario: Search terms in close proximity rank higher in results
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search(`cats dogs`);

                document.querySelector('[data-count]').innerText = `${search.results.length} result(s)`;
                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-count]" should contain "2 result(s)"
        Then The selector "[data-result]" should contain "/dog/, /cat/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search(`cats posts`);

                document.querySelector('[data-count]').innerText = `${search.results.length} result(s)`;
                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-count]" should contain "2 result(s)"
        Then The selector "[data-result]" should contain "/cat/, /dog/"
