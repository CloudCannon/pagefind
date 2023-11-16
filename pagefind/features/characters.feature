Feature: Character Tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-result>Nothing</p>
            """

    Scenario: Pagefind matches special characters
        Given I have a "public/apiary/index.html" file with the body:
            """
            <h1>Béës</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("Béës");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/apiary/"

    Scenario: Pagefind matches emoji
        Given I have a "public/fam-separate/index.html" file with the body:
            """
            <h1>Fam 👨‍👩‍👧‍👦</h1>
            """
        Given I have a "public/fam-middled/index.html" file with the body:
            """
            <h1>F👨‍👩‍👧‍👦am</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("👨‍👩‍👧‍👦");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).sort().join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/fam-middled/, /fam-separate/"

    Scenario: Pagefind doesn't match HTML entities as their text
        Given I have a "public/apiary/index.html" file with the body:
            """
            <h1>The &quot;bees&quot;</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("bees");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.content).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain 'The "bees"'

    Scenario: Pagefind handles HTML entities in meta
        Given I have a "public/apiary/index.html" file with the body:
            """
            <h1 data-pagefind-meta="title">The &quot;bees&quot;</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("bees");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.meta.title).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain 'The "bees"'

    Scenario: Pagefind can search for a hyphenated phrase
        Given I have a "public/ds/index.html" file with the body:
            """
            <h1>The beet-root</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("beet-root");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain '/ds/'

    Scenario: Pagefind does not return results for queries that normalize to nothing
        Given I have a "public/bundaberg/index.html" file with the body:
            """
            <h1>Invert can before opening</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                // Preload some pages that Pagefind might then return as "all pages"
                await pagefind.preload("can");
                let search = await pagefind.search("*");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = `[${pages.length}]`;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain '[0]'

    Scenario: Punctuated compound words are indexed per word
        Given I have a "public/hyphen/index.html" file with the body:
            """
            <p>beet-root</p>
            """
        Given I have a "public/period/index.html" file with the body:
            """
            <p>image.png</p>
            """
        Given I have a "public/camel/index.html" file with the body:
            """
            <p>WKWebVIEWComponent</p>
            """
        Given I have a "public/underscore/index.html" file with the body:
            """
            <p>Word_Boundaries</p>
            """
        Given I have a "public/slash/index.html" file with the body:
            """
            <p>sandwich/salad</p>
            """
        Given I have a "public/comma/index.html" file with the body:
            """
            <p>Cloud,Cannon</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let pages = [
                    ...(await Promise.all((await pagefind.search("beet")).results.map(r => r.data()))),
                    ...(await Promise.all((await pagefind.search("root")).results.map(r => r.data()))),
                ];

                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain '/hyphen/, /hyphen/'
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let pages = [
                    ...(await Promise.all((await pagefind.search("image")).results.map(r => r.data()))),
                    ...(await Promise.all((await pagefind.search("png")).results.map(r => r.data()))),
                ];

                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain '/period/, /period/'
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let pages = [
                    ...(await Promise.all((await pagefind.search("WkWebVIEWComponent")).results.map(r => r.data()))),
                    ...(await Promise.all((await pagefind.search("web")).results.map(r => r.data()))),
                    ...(await Promise.all((await pagefind.search("component")).results.map(r => r.data()))),
                ];

                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain '/camel/, /camel/, /camel/'
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let pages = [
                    ...(await Promise.all((await pagefind.search("word")).results.map(r => r.data()))),
                    ...(await Promise.all((await pagefind.search("bound")).results.map(r => r.data()))),
                ];

                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain '/underscore/, /underscore/'
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let pages = [
                    ...(await Promise.all((await pagefind.search("sandwich")).results.map(r => r.data()))),
                    ...(await Promise.all((await pagefind.search("salad")).results.map(r => r.data()))),
                ];

                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain '/slash/, /slash/'
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let pages = [
                    ...(await Promise.all((await pagefind.search("CloudCannon")).results.map(r => r.data()))),
                    ...(await Promise.all((await pagefind.search("cloud")).results.map(r => r.data()))),
                    ...(await Promise.all((await pagefind.search("cannon")).results.map(r => r.data()))),
                ];

                document.querySelector('[data-result]').innerText = pages.map(p => p.url).join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain '/comma/, /comma/, /comma/'
