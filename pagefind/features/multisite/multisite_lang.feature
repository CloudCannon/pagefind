Feature: Multisite Search Languages

    Background:
        Given I have a "root/index.html" file with the body:
            """
            <p data-result>Nothing</p>
            """
        Given I have a "root/website_a/en/1/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="en">
                <head></head>
                <body>
                    <h1>Website site A English</h1>
                </body>
            </html>
            """
        # Make Website B predominantly pt-br
        Given I have a "root/website_b/pt/1/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="pt-br">
                <head></head>
                <body>
                    <h1>Website site B Portugese</h1>
                </body>
            </html>
            """
        Given I have a "root/website_b/pt/2/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="pt-br">
                <head></head>
                <body>
                    <h1>Website site B Portugese</h1>
                </body>
            </html>
            """
        Given I have a "root/website_b/en/1/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="en">
                <head></head>
                <body>
                    <h1>Website site B English</h1>
                </body>
            </html>
            """

    Scenario: Pagefind picks the same language across multiple sites
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

                let search = await pagefind.search("web"); // <-- TODO search for "website"

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/website_a/en/1/, /website_b/en/1/"

    Scenario: Language of merged indexes can be selected
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
                await pagefind.mergeIndex("/website_b/_pagefind/", {
                    language: "pt-br"
                });

                let search = await pagefind.search("web");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).sort().join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/website_a/en/1/, /website_b/pt/1/, /website_b/pt/2/"
