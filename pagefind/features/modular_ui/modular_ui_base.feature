Feature: Base Modular UI Tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <div id="search"></div>
            <div id="summary"></div>
            <div id="results"></div>
            <script src="/_pagefind/pagefind-modular-ui.js"></script>

            <script>
                window.pagefind = new PagefindModularUI.Instance();
                pagefind.add(new PagefindModularUI.Input({
                    containerElement: "#search"
                }));
                pagefind.add(new PagefindModularUI.Summary({
                    containerElement: "#summary"
                }));
                pagefind.add(new PagefindModularUI.ResultList({
                    containerElement: "#results"
                }));
            </script>
            """

    Scenario: Pagefind Modular UI loads
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        Then There should be no logs
        Then The selector "#search input" should exist

    Scenario: Pagefind Modular UI searches
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let e = new Event('input', {bubbles:true, cancelable:true});
                document.querySelector("#search input").value = "world";
                document.querySelector("#search input").dispatchEvent(e);
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-modular-list-link" should contain "world"
