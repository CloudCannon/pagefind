Feature: Indexing

    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |

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
            <div>
                <p>More unindexed content</p>
                <main data-pagefind-body="">
                    <p>Body number 3</p>
                </main>
                <p>And yet more unindexed content</p>
            </div>
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
                let pagefind = await import("/pagefind/pagefind.js");

                let searchone = await pagefind.search("hello");
                let searchonedata = await searchone.results[0].data();
                document.querySelector('[data-search-one]').innerText = searchonedata.content;

                let searchtwo = await pagefind.search("goodbye");
                document.querySelector('[data-search-two]').innerText = `${searchtwo.results.length} result(s)`;
            }
            """
        Then There should be no logs
        Then The selector "[data-search-one]" should contain "Hello World, from Pagefind. Huzzah! Little extra body. Body number 3."
        Then The selector "[data-search-two]" should contain "0 result(s)"

    Scenario: HTML attributes can be indexed
        Given I have a "public/index.html" file with the body:
            """
            <p data-search>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>Page Title</h1>
            <img src="/hero.png" alt="Alternate Text" data-pagefind-index-attrs="alt" />
            <p>Hello World, from Pagefind</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("Alternate");
                let searchdata = await search.results[0]?.data();
                document.querySelector('[data-search]').innerText = searchdata?.content;
            }
            """
        Then There should be no logs
        Then The selector "[data-search]" should contain "Page Title. Alternate Text. Hello World, from Pagefind."


