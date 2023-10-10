Feature: Base UI Tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |

    # in this senario I use the css attribute selector to make sure the link has the query param as the end
    # if the link doesn't exist, the check will fail
    # see https://developer.mozilla.org/en-US/docs/Web/CSS/Attribute_selectors#syntax:~:text=%5Battr%24%3Dvalue%5D,by%20value.

    Scenario: Pagefind UI adds highlight query params
        Given I have a "public/index.html" file with the body:
                """
                <div id="search"></div>
                <script src="/pagefind/pagefind-ui.js"></script>

                <script>
                    window.pui = new PagefindUI({ element: "#search" });
                </script>
                """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>hello world</h1>
            <p>Hello world! How are you</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("world");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link[href$='?pagefind-highlight=world']" should contain "hello world"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("hello world");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link[href$='?pagefind-highlight=hello&pagefind-highlight=world']" should contain "hello world"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("hello world!");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link[href$='?pagefind-highlight=hello&pagefind-highlight=world%21']" should contain "hello world"

    Scenario: Pagefind UI does not add highlight query params
        Given I have a "public/index.html" file with the body:
                """
                <div id="search"></div>
                <script src="/pagefind/pagefind-ui.js"></script>

                <script>
                    window.pui = new PagefindUI({ element: "#search", highlightQueryParamName: null });
                </script>
                """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>hello world</h1>
            <p>Hello world! How are you</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("world");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link[href$='/']" should contain "hello world"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("hello world");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link[href$='/']" should contain "hello world"

    Scenario: Pagefind UI uses custom highlight query param name
        Given I have a "public/index.html" file with the body:
                """
                <div id="search"></div>
                <script src="/pagefind/pagefind-ui.js"></script>

                <script>
                    window.pui = new PagefindUI({ element: "#search", highlightQueryParamName: 'custom-param' });
                </script>
                """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>hello world</h1>
            <p>Hello world! How are you</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("world");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link[href$='?custom-param=world']" should contain "hello world"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("hello world");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link[href$='?custom-param=hello&custom-param=world']" should contain "hello world"
        When I evaluate:
            """
            async function() {
                window.pui.triggerSearch("hello world!");
                await new Promise(r => setTimeout(r, 1500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result-link[href$='?custom-param=hello&custom-param=world%21']" should contain "hello world"
