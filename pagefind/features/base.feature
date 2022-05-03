Feature: Base Tests

  Scenario: CLI is working
    Given I have a "public/index.html" file
    When I run my program
    Then I should see "Running Pagefind" in stdout

