Feature: Graceful Pagefind Errors
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """

    Scenario: Pagefind doesn't error on parsing ambiguities
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>hello world</h1>
            """
        Given I have a "public/dog/index.html" file with the body:
            """
            <h1>hello world</h1>
            <select><xmp><script>"use strict";</script></select>
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
                let results = await Promise.all(search.results.map(r => r.data()));

                document.querySelector('[data-url]').innerText = results.map(r => r.url).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/, /dog/"

    Scenario: Pagefind finds a data-pagefind-body when elements sit outside of the main html element
        Given I have a "public/dog/index.html" file with the body:
            """
            <h1>should not be indexed, no data-pagefind-body</h1>
            """
        Given I have a "public/cat/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="en">

            <head></head>

            <body data-pagefind-body>
                <h1> hello world </h1>
            </body>

            </html>

            <script></script>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see "Found a data-pagefind-body element on the site" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search("world");
                let results = await Promise.all(search.results.map(r => r.data()));

                document.querySelector('[data-url]').innerText = results.map(r => r.url).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cat/"

    Scenario: Pagefind handles non-breaking spaces in segmented language pages
        Given I have a "public/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="ja">
            <body>
                <p data-url>Nothing</p>
            </body>
            </html>
            """
        Given I have a "public/ja/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="ja">
            <body>
                <p>Hello&nbsp;👋</p>
            </body>
            </html>
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

                let search = await pagefind.search("👋");
                let results = await Promise.all(search.results.map(r => r.data()));

                document.querySelector('[data-url]').innerText = results.map(r => r.url).sort().join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/ja/"

    # Previously, headings that didn't match \w would be filtered out
    Scenario: Pagefind multilingual sub-results
        Given I have a "public/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="fa-IR" dir="rtl">
            <body>
                <p data-url>Nothing</p>
            </body>
            </html>
            """
        Given I have a "public/test/index.html" file with the content:
            """
            <!DOCTYPE html>
            <html lang="fa-IR" dir="rtl">
            <body>
                <h1 id="_top">چامه - آصف آشنا</h1>
                <p>هزار سال پس از ماجرای گمشدنت</p>

                <h2 id="از">RTL ID</h2>
                <p>از پیاله‌ای چای سیاه پررنگ</p>

                <h2 id="rtl-content">بیرون نه می‌روی از من</h2>
                <p>بیرون نه می‌روی از من</p>
            </body>
            </html>
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

                let search = await pagefind.search("از");
                let results = await Promise.all(search.results.map(r => r.data()));
                let result = results[0];

                let subs = result.sub_results.map(s => s.url).sort().join(', ');

                document.querySelector('[data-url]').innerText = subs;
            }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/test/#%D8%A7%D8%B2, /test/#_top, /test/#rtl-content"
