Feature: Highlighting Result Tests

    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/words/index.html" file with the body:
            """
            <p id="has-highlight">Is this highlighted? It should be!</p>
            <p data-pagefind-ignore>This should not be highlighted</p>
            <p data-pagefind-ignore><span>This</span> should not be highlighted</p>
            <script type="module">
                await import('/pagefind/pagefind-highlight.js');
                new PagefindHighlight();
            </script>
            """
        Given I have a "public/single-body/index.html" file with the body:
            """
            <main data-pagefind-body>
                <p id="has-highlight">This should be highlighted</p>
                <p data-pagefind-ignore>This should not be highlighted</p>
            </main>
            <p id="no-highlight">This should not be highlighted</p>
            <script type="module">
                await import('/pagefind/pagefind-highlight.js');
                new PagefindHighlight();
            </script>
            """
        Given I have a "public/multiple-bodies/index.html" file with the body:
            """
            <main data-pagefind-body>
                <p id="has-highlight">This should be highlighted</p>
                <p data-pagefind-ignore>This should not be highlighted</p>
            </main>
            <p id="no-highlight">This should not be highlighted</p>
            <div data-pagefind-body>
                <p id="has-highlight">This should be highlighted</p>
                <p data-pagefind-ignore>This should not be highlighted</p>
            </div>
            <script type="module">
                await import('/pagefind/pagefind-highlight.js');
                new PagefindHighlight();
            </script>
            """
        Given I have a "public/options/index.html" file with the body:
            """
            <main data-pagefind-body>
                <p id="has-highlight">This should be highlighted</p>
                <p data-pagefind-ignore>This should not be highlighted</p>
            </main>
            <p id="no-highlight">This should not be highlighted</p>
            <div data-pagefind-body>
                <p id="has-highlight">This should be highlighted</p>
                <p class="ignore">This should not be highlighted</p>
                <p data-pagefind-ignore>This should not be highlighted</p>
            </div>
            <script type="module">
                await import('/pagefind/pagefind-highlight.js');
                new PagefindHighlight({
                highlightParam: 'custom-name',
                markOptions: {
                        className: 'custom-class',
                        exclude: [
                            "[data-pagefind-ignore]",
                            "[data-pagefind-ignore] *",
                            ".ignore"
                            ]
                        }
                });
            </script>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory

    Scenario: Highlight script is loaded
        When I load "/words/"
        When I evaluate:
            """
            async function() { await new Promise(r => setTimeout(r, 200)); }
            """
        Then I should see the file "public/pagefind/pagefind-highlight.js"
        Then There should be no logs

    Scenario: Highlight script marks correctly
        When I load "/words/?pagefind-highlight=this"
        When I evaluate:
            """
            async function() { await new Promise(r => setTimeout(r, 200)); }
            """
        Then There should be no logs
        Then The selector "#has-highlight mark" should contain "this"
        Then The selector "#has-highlight mark.pagefind-highlight" should contain "this"
        Then The selector "p[data-pagefind-ignore]:not(:has(span))" should contain "This should not be highlighted"
        Then The selector "p[data-pagefind-ignore]:has(span)" should contain "<span>This</span> should not be highlighted"
        When I load "/words/?pagefind-highlight=this&pagefind-highlight=should"
        Then There should be no logs
        Then The selector "#has-highlight mark:first-of-type" should contain "this"
        Then The selector "#has-highlight mark:nth-of-type(2)" should contain "should"
        When I load "/words/?pagefind-highlight=is+this"
        Then There should be no logs
        Then The selector "#has-highlight mark" should contain "Is this"
        Then The selector "p[data-pagefind-ignore]" should contain "This should not be highlighted"
        When I load "/words/?pagefind-highlight=highlighted%3F"
        Then There should be no logs
        Then The selector "#has-highlight mark" should contain "highlighted?"
        When I load "/words/?pagefind-highlight=this+highlighted%3F"
        Then There should be no logs
        Then The selector "#has-highlight mark:first-of-type" should contain "this highlighted?"

    Scenario: Highlight script stays within pagefind-body
        When I load "/single-body/?pagefind-highlight=this"
        When I evaluate:
            """
            async function() { await new Promise(r => setTimeout(r, 200)); }
            """
        Then There should be no logs
        Then The selector "#has-highlight mark" should contain "This"
        Then The selector "p[data-pagefind-ignore]" should contain "This should not be highlighted"
        Then The selector "#no-highlight" should contain "This should not be highlighted"
        When I load "/multiple-bodies/?pagefind-highlight=this"
        Then There should be no logs
        Then The selector "#has-highlight mark" should contain "This"
        Then The selector "p[data-pagefind-ignore]" should contain "This should not be highlighted"
        Then The selector "#no-highlight" should contain "This should not be highlighted"

    Scenario: Highlight script options work
        When I load "/options/?custom-name=this"
        When I evaluate:
            """
            async function() { await new Promise(r => setTimeout(r, 200)); }
            """
        Then There should be no logs
        Then The selector "#has-highlight mark" should contain "This"
        Then The selector "#has-highlight mark.custom-class" should contain "This"
        Then The selector "p[data-pagefind-ignore]" should contain "This should not be highlighted"
        Then The selector "p.ignore" should contain "This should not be highlighted"
        Then The selector "#no-highlight" should contain "This should not be highlighted"

