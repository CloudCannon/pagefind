Feature: Multilingual
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/en/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <title>Document</title>
                </head>
                <body>
                    <p>I am some English documentation</p>
                </body>
            </html>
            """
        Given I have a "public/pt-br/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="pt-br">
                <head>
                    <title>Document</title>
                </head>
                <body>
                    <p>I am a Portugese document (trust me — quilométricas — see?)</p>
                </body>
            </html>
            """

    Scenario: Pagefind searches for English with English stemming
        Given I have a "public/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <title>Document</title>
                </head>
                <body>
                    <p data-result>Nothing</p>
                </body>
            </html>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        Then I should see the file "public/_pagefind/wasm.unknown.pagefind"
        Then I should see the file "public/_pagefind/wasm.en.pagefind"
        Then I should see "en" in "public/_pagefind/pagefind-entry.json"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("documenting");

                let data = search.results[0] ? await search.results[0].data() : "None";
                document.querySelector('[data-result]').innerText = `${search.results.length} — ${data.url}`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "1 — /en/"

    Scenario: Pagefind searches for Portugese with Portugese stemming
        Given I have a "public/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="pt-br">
                <head>
                    <title>Document</title>
                </head>
                <body>
                    <p data-result>Nothing</p>
                </body>
            </html>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        Then I should see the file "public/_pagefind/wasm.unknown.pagefind"
        Then I should see the file "public/_pagefind/wasm.pt.pagefind"
        Then I should see "pt" in "public/_pagefind/pagefind-entry.json"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("quilométricos");

                let data = search.results[0] ? await search.results[0].data() : "None";
                document.querySelector('[data-result]').innerText = `${search.results.length} — ${data.url}`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "1 — /pt-br/"

    Scenario: Pagefind can be configured to lump all languages together
        Given I have a "public/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <title>Document</title>
                </head>
                <body>
                    <p data-result>Nothing</p>
                </body>
            </html>
            """
        When I run my program with the flags:
            | --force-language "en" |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        Then I should see the file "public/_pagefind/wasm.unknown.pagefind"
        Then I should see the file "public/_pagefind/wasm.en.pagefind"
        Then I should not see the file "public/_pagefind/wasm.pt.pagefind"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("documenting");

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = `${search.results.length} — ${data.map(d => d.url).sort().join(', ')}`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "2 — /en/, /pt-br/"