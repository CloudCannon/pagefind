@skip
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
                <p>Hello World, from Pagefind</p>
                <p data-pagefind-ignore>Goodbye</p>
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
                document.querySelector('[search-one]').innerText = searchonedata.content;

                let searchtwo = await pagefind.search("goodbye");
                document.querySelector('[search-two]').innerText = `${searchtwo.length} result(s)`;
            }
            """
        Then The selector "[data-search-one]" should contain "Hello World, from Pagefind\nHuzzah!"
        Then The selector "[data-search-two]" should contain "0 result(s)"
