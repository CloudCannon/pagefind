# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
# This file represents a backwards-compatible setup as it existed before 1.0  #
# These tests should remain as a permanent regresison check for older sites   #
# It is very unlikely that the tests in this file should be touched           #
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

Feature: Base Tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """

    Scenario: Search for a word
        Given I have a "public/cat/index.html" file with the body:
            """
            <link rel="pre-1.0-signal" href="_pagefind" >
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
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/"

    Scenario: Preload indexes then search for a word
        Given I have a "public/cat/index.html" file with the body:
            """
            <link rel="pre-1.0-signal" href="_pagefind" >
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
                let pagefind = await import("/_pagefind/pagefind.js");

                await pagefind.preload("wo");
                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/"
