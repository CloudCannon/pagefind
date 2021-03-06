Feature: Base UI Tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <div id="search"></div>
            <script src="/_pagefind/pagefind-ui.js" type="text/javascript"></script>

            <script>
                new PagefindUI({ element: "#search" });
            </script>
            """

    Scenario: Pagefind UI loads
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
        Then The selector ".pagefind-ui" should exist
