Feature: Filtering
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
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
            <span data-pagefind-filter="mood">Angry</span>
            <h1 data-pagefind-filter="color:Tabby">Ali Cat</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"

    Scenario: Filters can be retrieved
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let filters = await pagefind.filters();
                let strings = Object.entries(filters).map(([filter, values]) => {
                    values = Object.entries(values).map(([value, count]) => {
                        return `${value}(${count})`;
                    })
                    return `${filter}:[${values.join(", ")}]`;
                });

                document.querySelector('[data-results]').innerText = strings.join(' ');
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "color:[Black(1), Orange(1), Tabby(1), White(2)] mood:[Angry(1)]"

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

    Scenario: Filter counts are returned for a given search term
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                await pagefind.filters(); // Load filters
                let search = await pagefind.search("Ali");
                let strings = Object.entries(search.filters).map(([filter, values]) => {
                    values = Object.entries(values).map(([value, count]) => {
                        return `${value}(${count})`;
                    })
                    return `${filter}:[${values.join(", ")}]`;
                });

                document.querySelector('[data-results]').innerText = strings.join(' ');
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "color:[Black(0), Orange(0), Tabby(1), White(0)] mood:[Angry(1)]"

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

    Scenario: Filtering without search term returns only filter
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search(null, {
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