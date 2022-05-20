@skip
Feature: Filtering
    Background:
        Given I have a "public/index.html" file with the content:
            """
            <p data-results>
            </p>
            """
        Given I have a "public/cheeka/index.html" file with the content:
            """
            <body>
                <span data-pagefind-filter="color">Black</span>
                <span data-pagefind-filter="color">White</span>
                <h1>Cat</h1>
            </body>
            """
        Given I have a "public/theodore/index.html" file with the content:
            """
            <body>
                <span data-pagefind-filter="color">Orange</span>
                <h1 data-pagefind-filter="color:White">Cat</h1>
            </body>
            """
        Given I have a "public/ali/index.html" file with the content:
            """
            <body>
                <h1 data-pagefind-filter="color:Tabby">Cat</h1>
            </body>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"

    Scenario: All results are returned with no filters
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await pagefind.search("Cat");
                let data = await Promise.all(results.map(result => result.data()));

                document.querySelector('[data-results]').innerText = data.map(d => d.url).sort().join(', ');
            }
            """
        Then The selector "[data-results]" should contain "/ali/, /cheeka/, /theodore/"

    Scenario: Filtering on tagged elements
        When I evaluate:
            """js
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await pagefind.search("Cat", {
                    filter: {
                        color: "Orange"
                    }
                });
                let data = await Promise.all(results.map(result => result.data()));

                document.querySelector('[data-results]').innerText = data.map(d => d.url).sort().join(', ');
            }
            """
        Then The selector "[data-results]" should contain "/theodore/"

    Scenario: Filtering on tagged values
        When I evaluate:
            """js
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await pagefind.search("Cat", {
                    filter: {
                        color: "Tabby"
                    }
                });
                let data = await Promise.all(results.map(result => result.data()));

                document.querySelector('[data-results]').innerText = data.map(d => d.url).sort().join(', ');
            }
            """
        Then The selector "[data-results]" should contain "/ali/"

    Scenario: Filtering returns multiple results
        When I evaluate:
            """js
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await pagefind.search("Cat", {
                    filter: {
                        color: "White"
                    }
                });
                let data = await Promise.all(results.map(result => result.data()));

                document.querySelector('[data-results]').innerText = data.map(d => d.url).sort().join(', ');
            }
            """
        Then The selector "[data-results]" should contain "/cheeka/, /theodore/"
