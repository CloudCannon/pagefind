# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
# This file represents a backwards-compatible setup as it existed before 1.0  #
# These tests should remain as a permanent regresison check for older sites   #
# It is very unlikely that the tests in this file should be touched           #
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

Feature: Sanity Tests

  Scenario: CLI tests are working
    Given I have a "public/index.html" file with the body:
      """
        <link rel="pre-1.0-signal" href="_pagefind" >
        <p data-url>Nothing</p>
      """
    When I run my program with the flags:
      | --source public |
    Then I should see "Running Pagefind" in stdout
    Then I should see "pre-1.0 compatibility mode" in stderr
    Then I should see "The `source` option is deprecated" in stderr

  Scenario: Web tests are working
    Given I have a "public/index.html" file with the body:
      """
      <h1>Hello!</h1>
      """
    When I serve the "public" directory
    When I load "/"
    Then The selector "h1" should contain "Hello!"
