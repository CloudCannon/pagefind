Feature: Base UI Tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <div id="search"></div>
            <script src="/pagefind/pagefind-ui.js"></script>

            <script>
                window.pui = new PagefindUI({ element: "#search" });
            </script>
            """

    Scenario: Pagefind UI loads
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        Then There should be no logs
        Then The selector ".pagefind-ui" should exist

    Scenario: Pagefind UI searches
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("world");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link" should contain "world"

    Scenario: Pagefind UI can programmatically filter
        Given I have a "public/res-zero/index.html" file with the body:
            """
            <h1>title title title</h1>
            """
        Given I have a "public/res-one/index.html" file with the body:
            """
            <h1 data-pagefind-filter="bucket:a">title res one</h1>
            """
        Given I have a "public/res-two/index.html" file with the body:
            """
            <h1 data-pagefind-filter="bucket:b">title res two</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                window.pui.triggerFilters({ "bucket": "a" });
                window.pui.triggerSearch("title");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link" should contain "title res one"
        When I evaluate:
            """
            async function() {
                window.pui.triggerFilters({ "bucket": ["b"] });
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link" should contain "title res two"
