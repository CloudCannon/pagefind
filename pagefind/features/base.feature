Feature: Base Tests

  Scenario: Search for a word
    Given I have a "public/index.html" file with the content:
      """
      <p data-url>
      </p>
      """
    Given I have a "public/cat/index.html" file with the content:
      """
      <body>
      <h1>world</h1>
      </body>
      """
    When I run my program
    Then DEBUG I should see "Running Pagefind" in stdout
    Then I should see the file "public/_pagefind/pagefind.js"
    When I serve the "public" directory
    When I load "/"
    When I evaluate:
      """
      async function() {
      let pagefind = await import("/_pagefind/pagefind.js");
      let results = await pagefind.search("world");
      let data = await results[0].data();
      document.querySelector('[data-url]').innerText = data.url;
      }
      """
    Then The selector "[data-url]" should contain "/cat/"
