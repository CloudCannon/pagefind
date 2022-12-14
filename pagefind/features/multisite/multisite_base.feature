Feature: Multisite Search

    Background:
        Given I have a "root/index.html" file with the body:
            """
            <p data-result>Nothing</p>
            """
        Given I have a "root/website_a/hello/index.html" file with the body:
            """
            <h1>web web world PAGEFIND_ROOT_SELECTOR</h1>
            """
        Given I have a "root/website_b/lorem/index.html" file with the body:
            """
            <h1>web ipsum</h1>
            """

    Scenario: Pagefind can search across multiple sites
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

    Scenario: Pagefind UI can search across multiple sites
        Given I have a "root/index.html" file with the body:
            """
            <div id="search"></div>
            <script src="/website_a/_pagefind/pagefind-ui.js" type="text/javascript"></script>
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
        Then There should be no logs
        When I evaluate:
            """
            async function() {
                let pui = new PagefindUI({
                    element: "#search",
                    mergeIndex: [{
                        bundlePath: "/website_b/_pagefind/"
                    }]
                });
                pui.triggerSearch("web");
                await new Promise(r => setTimeout(r, 2500)); // TODO: await el in humane
            }
            """
        Then There should be no logs
        Then The selector ".pagefind-ui__result:nth-of-type(1) .pagefind-ui__result-link" should contain "web web world PAGEFIND_ROOT_SELECTOR"
        Then The selector ".pagefind-ui__result:nth-of-type(2) .pagefind-ui__result-link" should contain "web ipsum"

    # Tests that Pagefind can assemble URLs correctly.
    # Remove the @skip tag to run this test (since it makes an external request)
    @skip
    Scenario: Pagefind can search across discrete domains
        When I run my program with the flags:
            | --source root/website_a |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "root/website_a/_pagefind/pagefind.js"
        When I serve the "root" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/website_a/_pagefind/pagefind.js");
                await pagefind.mergeIndex("https://pagefind.app/_pagefind/");

                let search = await pagefind.search("PAGEFIND_ROOT_SELECTOR");

                let pages = await Promise.all(search.results.map(r => r.data()));
                document.querySelector('[data-result]').innerText = pages.map(p => p.url).sort().join(", ");
            }
            """
        # There will be a log since the versions do not match
        # Then There should be no logs
        Then The selector "[data-result]" should contain "/website_a/hello/, https://pagefind.app/docs/config-options/"

    # Tests that Pagefind UI can assemble URLs correctly.
    # Remove the @skip tag to run this test (since it makes an external request)
    @skip
    Scenario: Pagefind UI can search across discrete domains
        Given I have a "root/index.html" file with the body:
            """
            <div id="search"></div>
            <script src="/website_a/_pagefind/pagefind-ui.js" type="text/javascript"></script>
            """
        When I run my program with the flags:
            | --source root/website_a |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "root/website_a/_pagefind/pagefind.js"
        When I serve the "root" directory
        When I load "/"
        Then There should be no logs
        When I evaluate:
            """
            async function() {
                let pui = new PagefindUI({
                    element: "#search",
                    mergeIndex: [{
                        bundlePath: "https://pagefind.app/_pagefind/"
                    }]
                });
                pui.triggerSearch("PAGEFIND_ROOT_SELECTOR");

                let waiting_since = Date.now();
                while (!document.querySelector(".pagefind-ui__result-link:nth-of-type(2)")) {
                    await new Promise(r => setTimeout(r, 100)); // TODO: await el in humane
                    if (Date.now() - waiting_since > 10000) {
                        break;
                    }
                }

                const links = [...document.querySelectorAll(".pagefind-ui__result-link")].map(l => l.getAttribute('href')).sort();
                const expected = ["/website_a/hello/", "https://pagefind.app/docs/config-options/"];
                if (links.length < expected.length) {
                        throw new Error(`Expected ${expected.length} links, found ${links.length}`);
                }
                for (let i = 0; i < links.length; i++) {
                    if (links[i] !== expected[i]) {
                        throw new Error(`${links[i]} !== ${expected[i]}`);
                    }
                }
            }
            """
        # There will be a log since the versions do not match
        # Then There should be no logs
