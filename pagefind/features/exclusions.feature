Feature: Exclusions

    Scenario: Elements within search regions can be excluded from indexing and excerpts
        Given I have a "public/index.html" file with the content:
            """
            <p data-search-one></p>
            <p data-search-two></p>
            """
        Given I have a "public/cat/index.html" file with the content:
            """
            <body>
                <p>Hello World, from <span data-pagefind-ignore>not</span> Pagefind</p>
                <p data-pagefind-ignore>Goodbye</p>
                <div data-pagefind-ignore>
                    <p>Nested content</p>
                </div>
                <p>Huzzah!</p>
            </body>
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
                let searchonedata = await searchone[0].data();
                document.querySelector('[data-search-one]').innerText = searchonedata.content;

                let searchtwo = await pagefind.search("goodbye");
                document.querySelector('[data-search-two]').innerText = `${searchtwo.length} result(s)`;
            }
            """
        Then There should be no logs
        Then The selector "[data-search-one]" should contain "Hello World, from Pagefind. Huzzah!"
        Then The selector "[data-search-two]" should contain "0 result(s)"

    Scenario: Some elements are excluded automatically
        Given I have a "public/index.html" file with the content:
            """
            <p data-search-one></p>
            <p data-search-two></p>
            """
        Given I have a "public/cat/index.html" file with the content:
            """
            <body>
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
            </body>
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
                let searchonedata = await searchone[0].data();
                document.querySelector('[data-search-one]').innerText = searchonedata.content;

                let searchtwo = await pagefind.search("goodbye");
                document.querySelector('[data-search-two]').innerText = `${searchtwo.length} result(s)`;
            }
            """
        Then There should be no logs
        Then The selector "[data-search-one]" should contain "Hello World, from Pagefind. Hooray!"
        Then The selector "[data-search-two]" should contain "0 result(s)"
