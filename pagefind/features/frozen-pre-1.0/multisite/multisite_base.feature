# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
# This file represents a backwards-compatible setup as it existed before 1.0  #
# These tests should remain as a permanent regresison check for older sites   #
# It is very unlikely that the tests in this file should be touched           #
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

Feature: Multisite Search

    Background:
        Given I have a "root/index.html" file with the body:
            """
            <p data-result>Nothing</p>
            """
        Given I have a "root/website_a/hello/index.html" file with the body:
            """
            <link rel="pre-1.0-signal" href="_pagefind" >
            <h1>web web world PAGEFIND_ROOT_SELECTOR</h1>
            """
        Given I have a "root/website_b/lorem/index.html" file with the body:
            """
            <link rel="pre-1.0-signal" href="_pagefind" >
            <h1>web ipsum</h1>
            """

    Scenario: LEGACY Pagefind can search across multiple sites
        When I run my program with the flags:
            | --source root/website_a |
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "root/website_a/_pagefind/pagefind.js"
        When I run my program with the flags:
            | --source root/website_b |
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "root/website_b/_pagefind/pagefind.js"
        When I serve the "root" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/website_a/_pagefind/pagefind.js");
                await pagefind.mergeIndex("/website_b/_pagefind/");

                let search = await pagefind.search("web");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/website_a/hello/, /website_b/lorem/"

    Scenario: LEGACY Pagefind UI can search across multiple sites
        Given I have a "root/index.html" file with the body:
            """
            <div id="search"></div>
            <script src="/website_a/_pagefind/pagefind-ui.js"></script>
            """
        When I run my program with the flags:
            | --source root/website_a |
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "root/website_a/_pagefind/pagefind.js"
        When I run my program with the flags:
            | --source root/website_b |
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "root/website_b/_pagefind/pagefind.js"
        When I serve the "root" directory
        When I load "/"
        Then There should be no logs
        When I evaluate:
            """
            async function() {
                let pui = new PagefindUI({
                    element: "#search",
                    mergeIndex: [{
                        bundlePath: "/website_b/_pagefind/"
                    }]
                });
                pui.triggerSearch("web");
                await new Promise(r => setTimeout(r, 3500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result:nth-of-type(1) .pagefind-ui__result-link" should contain "web web world PAGEFIND_ROOT_SELECTOR"
        Then The selector ".pagefind-ui__result:nth-of-type(2) .pagefind-ui__result-link" should contain "web ipsum"
