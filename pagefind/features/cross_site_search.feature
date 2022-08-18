Feature: Cross Site Search

    # TODO Tests:
    # Loading https://example.com/docs/_pagefind/... gives the baseURL of `/docs/`
    # Sorting the scores of merged indexes correctly
    # Merging filters of multiple sites
    # Adjusting weights of merged index
    # Mapping new filters onto each index
    # Selecting merged index in a different language
    # Pagefind UI configuration

    Scenario: Pagefind can search across multiple sites
        Given I have a "root/index.html" file with the body:
            """
            <p data-result>Nothing</p>
            """
        Given I have a "root/website_a/hello/index.html" file with the body:
            """
            <h1>web world</h1>
            """
        Given I have a "root/website_b/lorem/index.html" file with the body:
            """
            <h1>web ipsum</h1>
            """
        When I run my program with the flags:
            | --source root/website_a |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "root/website_a/_pagefind/pagefind.js"
        When I run my program with the flags:
            | --source root/website_b |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "root/website_b/_pagefind/pagefind.js"
        When I serve the "root" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/website_a/_pagefind/pagefind.js");
                await pagefind.mergeIndex("/website_b/_pagefind/");

                let search = await pagefind.search("web");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/website_a/hello/, /website_b/lorem/"
