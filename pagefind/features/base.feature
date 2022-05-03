Feature: Base Tests

  Scenario: CLI is working
    Given I have a "public/index.html" file
    When I run my program
    Then I should see "Running Pagefind" in stdout

  Scenario: Web Test
    Given I have a "public/index.html" file with the content:
      """
      <!doctype html>
      <html>
      <head>
      <title>My Page</title>
      </head>
      <body>
      <h1>Hello!</h1>
      </body>
      </html>
      """
    When I serve the "public" directory
    When I load "/"
    Then The selector "h1" should contain "Hello!"
