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
                <h1>Cat Post.</h1>
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
        # NB: The HTML encoding below is a test artifact
        Then The selector "[data-result]" should contain "Cat Post. A post about the &lt;mark&gt;'felines'.&lt;/mark&gt; This post has some gnarly things to test the fragment formatting."

    @skip
    Scenario: Search results return tagged filters

    @skip
    Scenario: Search results return tagged metadata

