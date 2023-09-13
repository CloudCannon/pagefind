Feature: UI Hooks
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |

    Scenario: Pagefind UI can provide a hook to process search terms
        Given I have a "public/index.html" file with the body:
            """
            <h1>Search</h1>
            <div id="search"></div>
            <script src="/pagefind/pagefind-ui.js"></script>

            <script>
                window.pui = new PagefindUI({
                    element: "#search",
                    processTerm: (t) => t.replace("word", "search"),
                });
            </script>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("word");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane

                // TODO: Add more web test steps to humane instead of throwing js
                let el = document.querySelector(".pagefind-ui__result-link");
                if (el.getAttribute("href") !== "/") {
                    throw new Error("Search term should have been normalized by processTerm");
                }
            }
            """
        Then There should be no logs

    Scenario: Pagefind UI can provide a hook to process results
        Given I have a "public/index.html" file with the body:
            """
            <h1>Search</h1>
            <img src="my.png" />
            <div id="search"></div>
            <script src="/pagefind/pagefind-ui.js"></script>

            <script>
                window.pui = new PagefindUI({
                    element: "#search",
                    processResult: function (result) {
                        result.meta.image = `/example/absolute/${result.meta.image}`;
                        return result;
                    }
                });
            </script>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("search");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane

                // TODO: Add more web test steps to humane instead of throwing js
                let el = document.querySelector(".pagefind-ui__result-image");
                if (el.getAttribute("src") !== "/example/absolute/my.png") {
                    throw new Error("Path should have been updated by processResult");
                }
            }
            """
        Then There should be no logs
