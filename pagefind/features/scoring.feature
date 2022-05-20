@skip
Feature: Result Scoring

    Scenario: Search terms in close proximity rank higher in results
        Given I have a "public/cat/index.html" file with the content:
            """
            <body>
                <h1>Happy cats post, that later mentions dogs</h1>
            </body>
            """
        Given I have a "public/dog/index.html" file with the content:
            """
            <body>
                <h1>A post about dogs vs cats</h1>
            </body>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await pagefind.search(`cats dogs`);

                document.querySelector('[data-count]').innerText = `${results.length} result(s)`;
                let data = await Promise.all(results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then The selector "[data-count]" should contain "2 result(s)"
        Then The selector "[data-result]" should contain "/dog/, /cat/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await pagefind.search(`cats posts`);

                document.querySelector('[data-count]').innerText = `${results.length} result(s)`;
                let data = await Promise.all(results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then The selector "[data-count]" should contain "2 result(s)"
        Then The selector "[data-result]" should contain "/cat/, /dog/"
