# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
# This file represents a backwards-compatible setup as it existed before 1.0  #
# These tests should remain as a permanent regresison check for older sites   #
# It is very unlikely that the tests in this file should be touched           #
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

Feature: Base UI Tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <div id="search"></div>
            <script src="/_pagefind/pagefind-ui.js"></script>

            <script>
                window.pui = new PagefindUI({ element: "#search" });
            </script>
            """

    Scenario: LEGACY Pagefind UI loads
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        Then There should be no logs
        Then The selector ".pagefind-ui" should exist

    Scenario: LEGACY Pagefind UI searches
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "public/_pagefind/pagefind.js"
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
