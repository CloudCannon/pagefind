Feature: Word Stemming
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <ul>
                <li data-result>
            </ul>
            """

    Scenario: Searching for a word will match against the stem of that word
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>the cat is meowing</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("meowed");

                let data = await search.results[0].data();
                document.querySelector('[data-result]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/cat/"

    Scenario: Search is case independent
        Given I have a "public/cat/index.html" file with the body:
            """
            <h1>the cat is Meowing</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("meOWings");

                let data = await search.results[0].data();
                document.querySelector('[data-result]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/cat/"

    Scenario: Search is punctuation independent
        Given I have a "public/apple/index.html" file with the body:
            """
            <h1>My laptop doesn't have a USB-A port</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("usb[a");

                let data = await search.results[0].data();
                document.querySelector('[data-result]').innerText = data.url;
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/apple/"

    Scenario: Searching will backtrack a word to find a prefix
        Given I have a "public/doc/index.html" file with the body:
            """
            <p>a doc about installation</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("documentation");

                let results = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = results.map(r => `${r.url} • ${r.excerpt}`).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/doc/ • a &lt;mark&gt;doc&lt;/mark&gt; about installation."

    Scenario: Searching will backtrack a word to find a stem
        Given I have a "public/doc/index.html" file with the body:
            """
            <p>a doc about installation</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("installat");

                let results = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = results.map(r => `${r.url} • ${r.excerpt}`).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/doc/ • a doc about &lt;mark&gt;installation.&lt;/mark&gt;"