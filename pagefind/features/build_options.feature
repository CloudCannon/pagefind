Feature: Build Options
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |

    Scenario: Source folder can be configured
        Given I have a "my_website/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "my_website/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program with the flags:
            | --site my_website |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "my_website/pagefind/pagefind.js"
        When I serve the "my_website" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/"

    Scenario: Output path can be configured
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program with the flags:
            | --output-subdir _search |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_search/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_search/pagefind.js");

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/"

    Scenario: Output path can be configured with an absolute path
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        # {{humane_temp_dir}} will be replaced with an absolute path here,
        # making the output-subdir value absolute
        When I run my program with the flags:
            | --output-subdir {{humane_temp_dir}}/other/_search |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "other/_search/pagefind.js"
        When I serve the "." directory
        When I load "/public/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/other/_search/pagefind.js");

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/"

    Scenario: Output path can be configured relative to cwd
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>world</h1>
            """
        When I run my program with the flags:
            | --output-path misc/_search |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "misc/_search/pagefind.js"
        When I serve the "." directory
        When I load "/public/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/misc/_search/pagefind.js");

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/"

    Scenario: Root selector can be configured
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>Ignored</h1>
            <div class="content">
                <h1>Hello</h1>
            </div>
            <p data-pagefind-meta="ignored">Also ignored</p>
            """
        When I run my program with the flags:
            | --root-selector "body > .content" |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("hello");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = `${data.meta.title}, ${data.content} Ignored is ${data.meta.ignored}.`;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "Hello, Hello. Ignored is undefined."

    Scenario: File glob can be configured
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "public/cat/index.htm" file with the body:
            """
            <h1>world</h1>
            """
        Given I have a "pagefind.yml" file with the content:
            """
            glob: "**/*.{htm,html}"
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

                let search = await pagefind.search("world");

                let data = await search.results[0].data();
                document.querySelector('[data-url]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/index.htm"

    Scenario: Complex exclusionary file glob can be configured
        Given I have a "public/index.html" file with the body:
            """
            <p data-result>Nothing</p>
            """
        Given I have a "public/cat/index.htm" file with the body:
            """
            <h1>cat index</h1>
            """
        Given I have a "public/cat/cat.html" file with the body:
            """
            <h1>cat cat</h1>
            """
        Given I have a "public/kitty/cat/index.html" file with the body:
            """
            <h1>kitty cat index</h1>
            """
        Given I have a "public/cat.html" file with the body:
            """
            <h1>cat</h1>
            """
        Given I have a "pagefind.yml" file with the content:
            """
            glob: "{cat/index.htm,kitty/**/*.html,cat.html}"
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

                let search = await pagefind.search("cat");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).sort().join(", ");
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/cat.html, /cat/index.htm, /kitty/cat/"
