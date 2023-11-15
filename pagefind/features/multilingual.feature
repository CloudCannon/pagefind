Feature: Multilingual
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
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
                    <p>I am a Portuguese document (trust me — quilométricas — see?)</p>
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
        Then I should see the file "public/pagefind/pagefind.js"
        Then I should see the file "public/pagefind/wasm.unknown.pagefind"
        Then I should see the file "public/pagefind/wasm.en.pagefind"
        Then I should see "en" in "public/pagefind/pagefind-entry.json"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("documenting");

                let data = search.results[0] ? await search.results[0].data() : "None";
                document.querySelector('[data-result]').innerText = `${search.results.length} — ${data.url}`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "1 — /en/"

    Scenario: Pagefind searches for Portuguese with Portuguese stemming
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
        Then I should see the file "public/pagefind/pagefind.js"
        Then I should see the file "public/pagefind/wasm.unknown.pagefind"
        Then I should see the file "public/pagefind/wasm.pt-br.pagefind"
        Then I should see "pt-br" in "public/pagefind/pagefind-entry.json"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("quilométricos");

                let data = search.results[0] ? await search.results[0].data() : "None";
                document.querySelector('[data-result]').innerText = `${search.results.length} — ${data.url}`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "1 — /pt-br/"

    Scenario: Pagefind keeps dialects separate
        Given I have a "public/pt-pt/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="pt-pt">
                <head>
                    <title>Document</title>
                </head>
                <body>
                    <p>I am a different Portuguese document (trust me — quilométricas — see?)</p>
                </body>
            </html>
            """
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
        Then I should see the file "public/pagefind/pagefind.js"
        Then I should see the file "public/pagefind/wasm.unknown.pagefind"
        Then I should see the file "public/pagefind/wasm.pt-pt.pagefind"
        Then I should see the file "public/pagefind/wasm.pt-br.pagefind"
        Then I should see "pt-pt" in "public/pagefind/pagefind-entry.json"
        Then I should see "pt-br" in "public/pagefind/pagefind-entry.json"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

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
        Then I should see the file "public/pagefind/pagefind.js"
        Then I should see the file "public/pagefind/wasm.unknown.pagefind"
        Then I should see the file "public/pagefind/wasm.en.pagefind"
        Then I should not see the file "public/pagefind/wasm.pt-br.pagefind"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("documenting");

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = `${search.results.length} — ${data.map(d => d.url).sort().join(', ')}`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "2 — /en/, /pt-br/"

    Scenario: Pagefind merges omitted languages into the primary language
        Given I have a "public/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Document</title>
                </head>
                <body>
                    <p data-result>Nothing</p>
                </body>
            </html>
            """
        Given I have a "public/mystery/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Document</title>
                </head>
                <body>
                    <p>I am a mystery document</p>
                </body>
            </html>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        Then I should see the file "public/pagefind/wasm.unknown.pagefind"
        Then I should not see "unknown" in "public/pagefind/pagefind-entry.json"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("documenting");

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = `${data.map(d => d.url).sort().join(', ')}`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/en/, /mystery/"

    Scenario: Pagefind searches for unknown languages with no stemming
        Given I have a "public/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="my_cool_language">
                <head>
                    <title>Document</title>
                </head>
                <body>
                    <p data-result>Nothing</p>
                </body>
            </html>
            """
        Given I have a "public/mystery/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="my_cool_language">
                <head>
                    <title>Document</title>
                </head>
                <body>
                    <p>I am a documentation</p>
                </body>
            </html>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        Then I should see the file "public/pagefind/wasm.unknown.pagefind"
        Then I should not see the file "public/pagefind/wasm.my_cool_language.pagefind"
        Then I should see "my_cool_language" in "public/pagefind/pagefind-entry.json"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("documentation");
                let stem_search = await pagefind.search("documenting");

                let data = search.results[0] ? await search.results[0].data() : "None";
                document.querySelector('[data-result]').innerText = `${search.results.length} — ${data.url} — ${stem_search.results.length}`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "1 — /mystery/ — 0"

    Scenario: Pagefind can be destroyed and re-initialized
        Given I have a "public/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <title>Generic Page</title>
                </head>
                <body>
                    <p data-result-a>Nothing</p>
                    <p data-result-b>Nothing</p>
                </body>
            </html>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        Then I should see the file "public/pagefind/wasm.unknown.pagefind"
        Then I should see the file "public/pagefind/wasm.en.pagefind"
        Then I should see the file "public/pagefind/wasm.pt-br.pagefind"
        Then I should see "en" in "public/pagefind/pagefind-entry.json"
        Then I should see "pt-br" in "public/pagefind/pagefind-entry.json"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                await pagefind.init();
                let en_search = await pagefind.search("documenting");

                let en_data = en_search.results[0] ? await en_search.results[0].data() : "None";
                document.querySelector('[data-result-a]').innerText = `${en_search.results.length} — ${en_data.url}`;

                await pagefind.destroy();

                document.querySelector('html').setAttribute("lang", "pt-br");

                await pagefind.init();
                let pt_search = await pagefind.search("quilométricos");

                let pt_data = pt_search.results[0] ? await pt_search.results[0].data() : "None";
                document.querySelector('[data-result-b]').innerText = `${pt_search.results.length} — ${pt_data.url}`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result-a]" should contain "1 — /en/"
        Then The selector "[data-result-b]" should contain "1 — /pt-br/"
