Feature: UI Scoring
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |

    Scenario: Pagefind UI can customize scoring
        Given I have a "public/unrelated/index.html" file with the body:
            """
            <h1>unrelated</h1>
            <p>Donec id elit non mi porta gravida at eget metus. Fusce dapibus, tellus ac cursus commodo, tortor mauris condimentum nibh, ut fermentum massa justo sit amet risus. Nullam quis risus eget urna mollis ornare vel eu leo. Cras justo odio, dapibus ac facilisis in, egestas eget quam. Donec sed odio dui. Cras mattis consectetur purus sit amet fermentum.</p>
            """
        Given I have a "public/longer/index.html" file with the body:
            """
            <h1>longer</h1>
            <p>This post is quite long, and talks about terracotta at length.</p>
            <p>Fusce dapibus, tellus ac cursus commodo, tortor mauris condimentum nibh, ut fermentum terracotta justo sit amet risus. Donec sed odio dui. Aenean eu leo quam. Pellentesque ornare sem lacinia quam venenatis vestibulum. Nulla vitae elit libero, a pharetra augue. Aenean lacinia bibendum nulla sed consectetur. Donec id elit non mi porta gravida at eget metus. Maecenas faucibus mollis interdum.</p>
            <p>Integer erat a ante venenatis dapibus posuere velit aliquet. Vivamus sagittis lacus vel augue laoreet rutrum faucibus auctor. Nullam quis risus eget urna mollis ornare vel eu leo. Aenean lacinia bibendum nulla sed consectetur.</p>
            <p>Praesent commodo cursus magna, vel scelerisque consectetur et. Fusce dapibus, tellus ac cursus commodo, tortor mauris condimentum nibh, ut fermentum massa justo sit amet risus. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur blandit tempus porttitor.</p>
            """
        Given I have a "public/shorter/index.html" file with the body:
            """
            <h1>shorter</h1>
            <p>This is a shorter terracotta page.</p>
            """
        Given I have a "public/a.html" file with the body:
            """
            <div id="search"></div>
            <script src="/pagefind/pagefind-ui.js"></script>

            <script>
                window.pui = new PagefindUI({
                    element: "#search"
                });
            </script>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/a.html"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("terracotta");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link" should contain "shorter"
        Given I have a "public/b.html" file with the body:
            """
            <div id="search"></div>
            <script src="/pagefind/pagefind-ui.js"></script>

            <script>
                window.pui = new PagefindUI({
                    element: "#search",
                    ranking: {
                        termFrequency: 0.0
                    }
                });
            </script>
            """
        When I load "/b.html"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("terracotta");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link" should contain "longer"
