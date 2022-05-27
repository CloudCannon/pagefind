Feature: Filtering
    Background:
        Given I have a "public/index.html" file with the body:
            """
            <p data-results>Nothing</p>
            """
        Given I have a "public/cheeka/index.html" file with the body:
            """
            <span data-pagefind-filter="color">Black</span>
            <span data-pagefind-filter="color">White</span>
            <h1>Cat</h1>
            """
        Given I have a "public/theodore/index.html" file with the body:
            """
            <span data-pagefind-filter="color">Orange</span>
            <h1 data-pagefind-filter="color:White">Cat</h1>
            """
        Given I have a "public/ali/index.html" file with the body:
            """
            <h1 data-pagefind-filter="color:Tabby">Cat</h1>
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

                let search = await pagefind.search("Cat");
                let data = await Promise.all(search.results.map(result => result.data()));

                document.querySelector('[data-results]').innerText = data.map(d => d.url).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/ali/, /cheeka/, /theodore/"

    Scenario: Filtering on tagged elements
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("Cat", {
                    filters: {
                        color: "Orange"
                    }
                });
                let data = await Promise.all(search.results.map(result => result.data()));

                document.querySelector('[data-results]').innerText = data.map(d => d.url).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/theodore/"

    Scenario: Filtering on tagged values
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("Cat", {
                    filters: {
                        color: "Tabby"
                    }
                });
                let data = await Promise.all(search.results.map(result => result.data()));

                document.querySelector('[data-results]').innerText = data.map(d => d.url).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/ali/"

    Scenario: Filtering returns multiple results
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("Cat", {
                    filters: {
                        color: "White"
                    }
                });
                let data = await Promise.all(search.results.map(result => result.data()));

                document.querySelector('[data-results]').innerText = data.map(d => d.url).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/cheeka/, /theodore/"

    @skip
    # Currently only an AND filtering is supported. Need to restructure to support boolean logic
    Scenario: Filtering to multiple values
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("Cat", {
                    filters: {
                        color: ["Tabby", "Orange"]
                    }
                });
                let data = await Promise.all(search.results.map(result => result.data()));

                document.querySelector('[data-results]').innerText = data.map(d => d.url).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/ali/, /theodore/"

    @skip
    Scenario: Non-existent filters return no results
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("Cat", {
                    filters: {
                        name: "Ali"
                    }
                });

                document.querySelector('[data-results]').innerText = search.results.length;
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "0"

    @skip
    Scenario: Non-existent values return no results
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("Cat", {
                    filters: {
                        color: "Green"
                    }
                });

                document.querySelector('[data-results]').innerText = search.results.length;
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "0"