Feature: Anchors

    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-search>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1 id="outer-heading">Outer Heading</h1>
            <div data-pagefind-body>
                <p>PageOne, from Pagefind</p>
                <h2 id="cats">Cats</h2>
                <ul id="list">
                    <li>Cheeka</li>
                    <li id="ali">Ali</li>
                    <li>Theodore</li>
                    <li>Smudge</li>
                </ul>
                <h2 id="pagefind">Pagefind</h2>
                <p>PageOne, again, from Pagefind</p>
            </div>
            <p id="outer-content">Outer Content</p>
            """
        Given I have a "public/dog/index.html" file with the body:
            """
            <div data-pagefind-body>
                <h1 id="h1">PageTwo, from Pagefind</h1>
                <p id="p_spans">Words <span>in</span> <span><span>spans</span></span> should be extracted</p>
                <h2 id="h2_hrefs">Links <a href="/">should be extracted</a></h2>
                <span id="span_formatted">Text that is <b>bold</b> or <i>italic</i> should be extracted</span>
                <p id="p_nested_ids">Text containing <span id="span_nested">nested IDs</span> should extract both</p>
                <div id="double_div">Divs containing <div>💀 he he he 💀</div> divs should only take from the top level</div>
            </div>
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

                let search = await pagefind.search("pageone");
                let searchdata = await search.results[0].data();
                document.querySelector('[data-search]').innerText = searchdata.locations.join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-search]" should contain "0, 9"

    Scenario: Pagefind returns full content without anchors
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("pageone");
                let searchdata = await search.results[0].data();
                document.querySelector('[data-search]').innerText = searchdata.content;
            }
            """
        Then There should be no logs
        Then The selector "[data-search]" should contain "PageOne, from Pagefind. Cats. Cheeka. Ali. Theodore. Smudge. Pagefind. PageOne, again, from Pagefind."

    Scenario: Pagefind returns all page anchors in the fragment
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("pageone");
                let searchdata = await search.results[0].data();
                document.querySelector('[data-search]').innerText = searchdata.anchors.map(a => `${a.element}#${a.id}: ${a.location}`).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-search]" should contain "h2#cats: 3, ul#list: 4, li#ali: 5, h2#pagefind: 8"

    Scenario: Pagefind returns page anchor content in the fragment
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("pageone");
                let searchdata = await search.results[0].data();
                document.querySelector('[data-search]').innerText = searchdata.anchors.map(a => `#${a.id}: '${a.text}'`).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-search]" should contain "#cats: 'Cats', #list: '', #ali: 'Ali', #pagefind: 'Pagefind'"

    Scenario: Pagefind extracts page anchor text where it makes sense
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("pagetwo");
                let searchdata = await search.results[0].data();
                document.querySelector('[data-search]').innerHTML = `
                    <ul>
                        ${searchdata.anchors.map(a => `<li>#${a.id}: '${a.text}'</li>`)}
                    </ul>
                `;
            }
            """
        Then There should be no logs
        Then The selector "[data-search]>ul>li:nth-of-type(1)" should contain "#h1: 'PageTwo, from Pagefind'"
        Then The selector "[data-search]>ul>li:nth-of-type(2)" should contain "#p_spans: 'Words in spans should be extracted'"
        Then The selector "[data-search]>ul>li:nth-of-type(3)" should contain "#h2_hrefs: 'Links should be extracted'"
        Then The selector "[data-search]>ul>li:nth-of-type(4)" should contain "#span_formatted: 'Text that is bold or italic should be extracted'"
        Then The selector "[data-search]>ul>li:nth-of-type(5)" should contain "#p_nested_ids: 'Text containing nested IDs should extract both'"
        Then The selector "[data-search]>ul>li:nth-of-type(6)" should contain "#span_nested: 'nested IDs'"
        Then The selector "[data-search]>ul>li:nth-of-type(7)" should contain "#double_div: 'Divs containing divs should only take from the top level'"
