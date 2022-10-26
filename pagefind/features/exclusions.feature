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
            <style> * { color: red; } </style>
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

    Scenario: Tagged elements inside ignored elements can be ignored
        Given I have a "public/index.html" file with the body:
            """
            <p data-search>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <p>Hello World, from Pagefind</p>
            <div data-pagefind-ignore>
                <p data-pagefind-meta="elided">Nested content</p>
            </div>
            <div data-pagefind-ignore="index">
                <p data-pagefind-meta="index">Nested content</p>
            </div>
            <div data-pagefind-ignore="all">
                <p data-pagefind-meta="all">Nested content</p>
            </div>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("hello");
                let searchdata = await search.results[0].data();
                document.querySelector('[data-search]').innerText = [
                    searchdata.meta?.elided || "None",
                    searchdata.meta?.index || "None",
                    searchdata.meta?.all || "None",
                ].join(' — ');
            }
            """
        Then There should be no logs
        Then The selector "[data-search]" should contain "Nested content — Nested content — None"
