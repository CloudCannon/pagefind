Feature: UI Test Strings
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |

    Scenario: Pagefind UI loads automatic translations
        Given I have a "public/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="pt-br">
                <head></head>
                <body>
                    <h1>Search</h1>
                    <div id="search"></div>
                    <script src="/_pagefind/pagefind-ui.js" type="text/javascript"></script>

                    <script>
                        window.pui = new PagefindUI({ element: "#search" });
                    </script>
                </body>
            </html>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("garbage");
                await new Promise(r => setTimeout(r, 200)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__message" should contain "Nenhum resultado encontrado para garbage"

    Scenario: Pagefind UI can load custom translations
        Given I have a "public/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="pt-br">
                <head></head>
                <body>
                    <h1>Search</h1>
                    <div id="search"></div>
                    <script src="/_pagefind/pagefind-ui.js" type="text/javascript"></script>

                    <script>
                        window.pui = new PagefindUI({
                            element: "#search",
                            translations: {
                                zero_results: "[SEARCH_TERM] is oh no"
                            }
                        });
                    </script>
                </body>
            </html>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("garbage");
                await new Promise(r => setTimeout(r, 200)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__message" should contain "garbage is oh no"
