Feature: Metadata
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the content:
            """
            <html>
            <head></head>
            <body>
                <p data-result>Nothing</p>
                <p data-result-two>Nothing</p>
            </body>
            </html>
            """
        # Covering all filter and metadata syntaxes
        Given I have a "public/cat/index.html" file with the content:
            """
            <html>
            <head>
                <meta data-pagefind-meta="social-image[content]" content="/kitty.jpg" property="og:image">
            </head>
            <body>
                <img src="/logo.png" />
                <h1 data-pagefind-filter="title">
                    Cat Post.
                </h1>
                <span data-pagefind-ignore data-pagefind-filter="animal">cats</span>
                <img src="/cat.png" alt="cat picture" />
                <p>A post about the 'felines'</p>
                <p>This post has some <span data-pagefind-meta="adjective">gnarly</span> things to test the fragment formatting.</p>
                <img data-pagefind-meta="footer[src], footer_alt[alt]" src="/cat-footer.png" alt="cat footer picture" />
            </body>
            </html>
            """
        # Covering metadata hierarchy
        Given I have a "public/dog/index.html" file with the content:
            """
            <html>
            <head>
                <meta data-pagefind-default-meta="image[content]" content="/puppy.jpg" property="og:image">
                <title data-pagefind-default-meta="title">Website | Dogs</title>
            </head>
            <body>
                <h1>Dog Post.</h1>
                <span data-pagefind-meta="animal">dog</span>
                <span data-pagefind-default-meta="animal,vegetable,mineral">generic</span>
            </body>
            </html>
            """
        # Covering fallback titles
        Given I have a "public/kea/index.html" file with the content:
            """
            <html>
            <head>
                <title>Website | Kea</title>
            </head>
            <body>
                <p>NZ Kea Post.</p>
            </body>
            </html>
            """
        Given I have a "public/kaka/index.html" file with the content:
            """
            <html>
            <head>
                <title>Website | Kaka</title>
            </head>
            <body>
                <h1></h1>
                <p>NZ Kaka Post.</p>
            </body>
            </html>
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

                let search = await pagefind.search("cat");

                let data = await search.results[0].data();
                document.querySelector('[data-result]').innerText = data.meta.title;
                document.querySelector('[data-result-two]').innerText = `${data.meta.image} | ${data.meta.image_alt}`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "Cat Post."
        Then The selector "[data-result-two]" should contain "/cat.png | cat picture"

    Scenario: Search results return nicely formatted content
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("feline");

                let data = await search.results[0].data();
                document.querySelector('[data-result]').innerText = data.content;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "Cat Post. A post about the 'felines'. This post has some gnarly things to test the fragment formatting."

    Scenario: Search results return highlighted search excerpt
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("feline");

                let data = await search.results[0].data();
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

                let search = await pagefind.search("cat");

                let data = await search.results[0].data();
                document.querySelector('[data-result]').innerText = Object.entries(data.filters).map(([f, v]) => `${f}: ${v}`).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "animal: cats, title: Cat Post."

    Scenario: Search results return tagged metadata
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("cat");

                let data = await search.results[0].data();
                document.querySelector('[data-result]').innerText = data.meta["social-image"] + " — " + data.meta.adjective;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/kitty.jpg — gnarly"

    Scenario: Search results return complex metadata
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("cat");

                let data = await search.results[0].data();
                document.querySelector('[data-result]').innerText = data.meta.footer + " — " + data.meta.footer_alt;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/cat-footer.png — cat footer picture"

    Scenario: Default metadata can be defined
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("dog");

                let data = await search.results[0].data();
                document.querySelector('[data-result]').innerText = [
                    data.meta.title,
                    data.meta.image,
                    data.meta.animal,
                    data.meta.vegetable,
                    data.meta.mineral,
                ].join(' | ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "Dog Post. | /puppy.jpg | dog | generic | generic"

    Scenario: Title metadata falls back to the title element
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("NZ");

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.meta.title).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "Website | Kaka, Website | Kea"
