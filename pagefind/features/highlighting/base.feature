Feature: Highlighting Tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |

    Scenario: Highlight script is loaded
        Given I have a "public/page/index.html" file with the body:
            """
            <p class="test">Nothing</p>
            <script type="module" src="/pagefind/pagefind-highlight.js"></script>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind-highlight.js"
        When I serve the "public" directory
        When I load "/page/"
        Then There should be no logs
        Then The selector ".test" should contain "Hello from the highlight script"
