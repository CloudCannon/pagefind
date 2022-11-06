Feature: UI Hooks
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |

    Scenario: Pagefind UI can provide a hook to process results
        Given I have a "public/index.html" file with the body:
            """
            <h1>Search</h1>
            <img src="my.png" />
            <div id="search"></div>
            <script src="/_pagefind/pagefind-ui.js" type="text/javascript"></script>

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
        Then I should see the file "public/_pagefind/pagefind.js"
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
