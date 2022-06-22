Feature: Exclusions

    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |

    Scenario: Elements within search regions can be excluded from indexing and excerpts
        Given I have a "public/index.html" file with the body:
            """
            <p data-search-one>Nothing</p>
            <p data-search-two>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <p>Hello World, from <span data-pagefind-ignore>not</span> Pagefind</p>
            <p data-pagefind-ignore>Goodbye</p>
            <div data-pagefind-ignore>
                <p>Nested content</p>
            </div>
            <p>Huzzah!</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let searchone = await pagefind.search("hello");
                let searchonedata = await searchone.results[0].data();
                document.querySelector('[data-search-one]').innerText = searchonedata.content;

                let searchtwo = await pagefind.search("goodbye");
                document.querySelector('[data-search-two]').innerText = `${searchtwo.results.length} result(s)`;
            }
            """
        Then There should be no logs
        Then The selector "[data-search-one]" should contain "Hello World, from Pagefind. Huzzah!"
        Then The selector "[data-search-two]" should contain "0 result(s)"

    Scenario: Some elements are excluded automatically
        Given I have a "public/index.html" file with the body:
            """
            <p data-search-one></p>
            <p data-search-two></p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <p>Hello World, from Pagefind</p>
            <script>let value = "Goodbye";</script>
            <svg>goodbye</svg>
            <form>
                <label>
                    Goodbye
                    <input type="goodbye" />
                </label>
            </form>
            <p>Hooray!</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let searchone = await pagefind.search("hello");
                let searchonedata = await searchone.results[0].data();
                document.querySelector('[data-search-one]').innerText = searchonedata.content;

                let searchtwo = await pagefind.search("goodbye");
                document.querySelector('[data-search-two]').innerText = `${searchtwo.results.length} result(s)`;
            }
            """
        Then There should be no logs
        Then The selector "[data-search-one]" should contain "Hello World, from Pagefind. Hooray!"
        Then The selector "[data-search-two]" should contain "0 result(s)"

    Scenario: Indexing can be limited to a given element
        Given I have a "public/index.html" file with the body:
            """
            <p data-search-one>Nothing</p>
            <p data-search-two>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>Outer Content</h1>
            <div data-pagefind-body>
                <p>Hello World, from Pagefind</p>
                <p>Huzzah!</p>
            </div>
            <p>goodbye content</p>
            <p data-pagefind-body>Little extra body</p>
            """
        # The above data-pagefind-body existing on a page should
        # exclude all pages that do not include it.
        Given I have a "public/dog/index.html" file with the body:
            """
            <h1>No selector</h1>
            <p>goodbye content</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let searchone = await pagefind.search("hello");
                let searchonedata = await searchone.results[0].data();
                document.querySelector('[data-search-one]').innerText = searchonedata.content;

                let searchtwo = await pagefind.search("goodbye");
                document.querySelector('[data-search-two]').innerText = `${searchtwo.results.length} result(s)`;
            }
            """
        Then There should be no logs
        Then The selector "[data-search-one]" should contain "Hello World, from Pagefind. Huzzah! Little extra body."
        Then The selector "[data-search-two]" should contain "0 result(s)"
