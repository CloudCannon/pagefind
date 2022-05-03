Feature: Base Tests

  Scenario: CLI is working
    Given I have a "public/index.html" file
    When I run my program
    Then I should see "Running Pagefind" in stdout

  Scenario: Web Test
    When I load "https://cloudcannon.com"
    Then The selector "h1" should contain "The collaborative CMS, powered by Git."


