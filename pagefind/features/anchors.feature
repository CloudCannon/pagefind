Feature: Anchors

    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-search-one>Nothing</p>
            <p data-search-two>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1 id="outer-heading">Outer Heading</h1>
            <div data-pagefind-body>
                <p>Hello World, from Pagefind</p>
                <h2 id="cats">Cats</h2>
                <ul>
                    <li>Cheeka</li>
                    <li id="ali">Ali</li>
                    <li>Theodore</li>
                    <li>Smudge</li>
                </ul>
                <h2 id="pagefind">Pagefind</h2>
                <p>Hello World, again, from Pagefind</p>
            </div>
            <p id="outer-content">Outer Content</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"

    Scenario: Pagefind returns all word locations in the fragment
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let searchone = await pagefind.search("hello");
                let searchonedata = await searchone.results[0].data();
                document.querySelector('[data-search-one]').innerText = searchonedata.locations.join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-search-one]" should contain "0, 10"

    Scenario: Pagefind returns full content without anchors
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let searchone = await pagefind.search("hello");
                let searchonedata = await searchone.results[0].data();
                document.querySelector('[data-search-one]').innerText = searchonedata.content;
            }
            """
        Then There should be no logs
        Then The selector "[data-search-one]" should contain "Hello World, from Pagefind. Cats. Cheeka. Ali. Theodore. Smudge. Pagefind. Hello World, again, from Pagefind."

    Scenario: Pagefind returns all page anchors in the fragment
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let searchone = await pagefind.search("hello");
                let searchonedata = await searchone.results[0].data();
                document.querySelector('[data-search-one]').innerText = searchonedata.anchors.map(a => `${a.element}#${a.id}: ${a.location}`).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-search-one]" should contain "h2#cats: 4, li#ali: 6, h2#pagefind: 9"
