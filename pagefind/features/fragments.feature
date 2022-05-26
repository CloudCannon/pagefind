Feature: Fragments
    Background:
        Given I have a "public/index.html" file with the content:
            """
            <p data-result>
            </p>
            """
        Given I have a "public/cat/index.html" file with the content:
            """
            <body>
                <h1 data-pagefind-filter="title">
                    Cat Post.
                </h1>
                <span data-pagefind-ignore data-pagefind-filter="animal">cats</span>
                <p>A post about the 'felines'</p>
                <p>This post has some <span>gnarly<span> things to test the fragment formatting.</p>
            </body>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"

    Scenario: Search results return generic information about the page
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await pagefind.search("cat");

                let data = await results[0].data();
                document.querySelector('[data-result]').innerText = data.title;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "Cat Post."

    Scenario: Search results return nicely formatted content
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await pagefind.search("feline");

                let data = await results[0].data();
                document.querySelector('[data-result]').innerText = data.content;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "Cat Post. A post about the 'felines'. This post has some gnarly things to test the fragment formatting."

    Scenario: Search results return highlighted search exerpt
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await pagefind.search("feline");

                let data = await results[0].data();
                document.querySelector('[data-result]').innerText = data.excerpt;
            }
            """
        Then There should be no logs
        # NB: The HTML encoding below is a test artifact
        Then The selector "[data-result]" should contain "Cat Post. A post about the &lt;mark&gt;'felines'.&lt;/mark&gt; This post has some gnarly things to test the fragment formatting."

    Scenario: Search results return tagged filters
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let results = await pagefind.search("cat");

                let data = await results[0].data();
                document.querySelector('[data-result]').innerText = Object.entries(data.filters).map(([f, v]) => `${f}: ${v}`).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "animal: cats, title: Cat Post."

    @skip
    Scenario: Search results return tagged metadata

