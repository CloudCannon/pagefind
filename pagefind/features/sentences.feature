Feature: Sentence Building Tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-result>Nothing</p>
            """

    Scenario: Pagefind joins block elements as sentences
        Given I have a "public/apiary/index.html" file with the body:
            """
            <p>Hello</p><p>World</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("h");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.content).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "Hello. World."

    Scenario: Pagefind doesn't join inline elements as sentences
        Given I have a "public/apiary/index.html" file with the body:
            """
            <span>Hello</span><span>World</span>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("h");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.content).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "HelloWorld"

    Scenario: Pagefind treats br tags as spaces
        Given I have a "public/apiary/index.html" file with the body:
            """
            <p>Hello<br/>World</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("w");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.content).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "Hello World."
