Feature: Anchors Example

    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-search>Nothing</p>
            """
        Given I have a "public/installation/index.html" file with the body:
            """
            <main>
                <h1>Installing and running Pagefind</h1>
                <p>Pagefind is a static binary with no dynamic dependencies, so in most cases will be simple to install and run. Pagefind is currently supported on Windows, macOS, and x86-64 Linux distributions.</p>

                <h2 id="running-via-npx">Running via npx</h2>
                <p>Pagefind publishes a <a href="https://www.npmjs.com/package/pagefind">wrapper package through npm</a>, which is the easiest way to get started. This package will download the correct <a href="https://github.com/CloudCannon/pagefind/releases">binary of the latest release</a> from GitHub for your platform and run it.</p>
                <p>Specific versions can be run by passing a version tag.</p>
                <p>Running Pagefind via npx will download the <code class="inline">pagefind_extended</code> release, which includes specialized support for indexing Chinese and Japanese pages.</p>
                </blockquote>

                <h2 id="downloading-a-precompiled-binary">Downloading a precompiled binary</h2>
                <p>If you prefer to install the tool yourself, you can download a <a href="https://github.com/CloudCannon/pagefind/releases">precompiled release from GitHub</a> and run the binary directly.</p>
                <p>We publish two releases, with one being extended. The extended release is a larger binary, but includes specialized support for indexing Chinese and Japanese pages.</p>

                <h2 id="building-from-source">Building from source</h2>
                <p>You can run <code class="inline">cargo install pagefind</code> to build from source.</p>
            </main>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"

    Scenario: Pagefind returns subresults for an example page
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let search = await pagefind.search("pagefind");
                let searchdata = await search.results[0].data();
                document.querySelector('[data-search]').innerHTML = `
                    <ul>
                        ${searchdata.sub_results.map(r => `<li>${r.url}: ${r.title} / '${r.excerpt}'</li>`).join('')}
                    </ul>
                `;
            }
            """
        Then There should be no logs
        Then The selector "[data-search]>ul>li:nth-of-type(1)" should contain "/installation/: Installing and running Pagefind / 'Installing and running <mark>Pagefind.</mark> <mark>Pagefind</mark> is a static binary with no dynamic dependencies, so in most cases will be simple to install and run. <mark>Pagefind</mark> is currently supported on Windows,'"
        Then The selector "[data-search]>ul>li:nth-of-type(2)" should contain "/installation/#running-via-npx: Running via npx / 'versions can be run by passing a version tag. Running <mark>Pagefind</mark> via npx will download the <mark>pagefind_extended</mark> release, which includes specialized support for indexing Chinese and Japanese pages.'"
        Then The selector "[data-search]>ul>li:nth-of-type(3)" should contain "/installation/#building-from-source: Building from source / 'Building from source. You can run cargo install <mark>pagefind</mark> to build from source.'"
