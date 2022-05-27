Feature: Sanity Tests

  Scenario: CLI tests are working
    Given I have a "public/index.html" file
    When I run my program
    Then I should see "Running Pagefind" in stdout

  Scenario: Web tests are working
    Given I have a "public/index.html" file with the body:
      """
      <h1>Hello!</h1>
      """
    When I serve the "public" directory
    When I load "/"
    Then The selector "h1" should contain "Hello!"
