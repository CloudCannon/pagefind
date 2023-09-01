Feature: Result Sorting
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-asc></p>
            <p data-desc></p>
            """
        Given I have a "public/robe/painte/index.html" file with the body:
            """
            <span data-lumens="14900" data-pagefind-sort="lumens[data-lumens]"></span>
            <h1>Robe <i data-pagefind-sort="model">Painte</i> ™️</h1>
            <i data-pagefind-sort="weight:-0.4"></i>
            <b data-pagefind-sort="mixed">1</b>
            <u data-pagefind-sort="color">black</u>
            """
        Given I have a "public/robe/megapointe/index.html" file with the body:
            """
            <span data-pagefind-sort="lumens:9870"></span>
            <h1>Robe <i data-pagefind-sort="model">MegaPointe</i> ™️</h1>
            <i data-pagefind-sort="weight:9.9"></i>
            <b data-pagefind-sort="mixed">9.5</b>
            <u data-pagefind-sort="color">white</u>
            """
        Given I have a "public/robe/superspikie/index.html" file with the body:
            """
            <span data-pagefind-sort="lumens">5105</span>
            <h1>Robe <i data-pagefind-sort="model">SuperSpikie</i> ™️</h1>
            <i data-pagefind-sort="weight:1234.5678"></i>
            <b data-pagefind-sort="mixed">10</b>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"

    Scenario: Pagefind can sort results by an alphabetical attribute
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let asc_search = await pagefind.search("Robe", { sort: { model: "asc" } });
                let asc_data = await Promise.all(asc_search.results.map(result => result.data()));
                document.querySelector('[data-asc]').innerText = asc_data.map(d => d.url).join(', ');

                let desc_search = await pagefind.search("Robe", { sort: { model: "desc" } });
                let desc_data = await Promise.all(desc_search.results.map(result => result.data()));
                document.querySelector('[data-desc]').innerText = desc_data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-asc]" should contain "/robe/megapointe/, /robe/painte/, /robe/superspikie/"
        Then The selector "[data-desc]" should contain "/robe/superspikie/, /robe/painte/, /robe/megapointe/"

    Scenario: Pagefind can sort results by an automatically detected integer
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let asc_search = await pagefind.search("Robe", { sort: { lumens: "asc" } });
                let asc_data = await Promise.all(asc_search.results.map(result => result.data()));
                document.querySelector('[data-asc]').innerText = asc_data.map(d => d.url).join(', ');

                let desc_search = await pagefind.search("Robe", { sort: { lumens: "desc" } });
                let desc_data = await Promise.all(desc_search.results.map(result => result.data()));
                document.querySelector('[data-desc]').innerText = desc_data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-asc]" should contain "/robe/superspikie/, /robe/megapointe/, /robe/painte/"
        Then The selector "[data-desc]" should contain "/robe/painte/, /robe/megapointe/, /robe/superspikie/"

    Scenario: Pagefind can sort results by an automatically detected float
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let asc_search = await pagefind.search("Robe", { sort: { weight: "asc" } });
                let asc_data = await Promise.all(asc_search.results.map(result => result.data()));
                document.querySelector('[data-asc]').innerText = asc_data.map(d => d.url).join(', ');

                let desc_search = await pagefind.search("Robe", { sort: { weight: "desc" } });
                let desc_data = await Promise.all(desc_search.results.map(result => result.data()));
                document.querySelector('[data-desc]').innerText = desc_data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-asc]" should contain "/robe/painte/, /robe/megapointe/, /robe/superspikie/"
        Then The selector "[data-desc]" should contain "/robe/superspikie/, /robe/megapointe/, /robe/painte/"

    Scenario: Pagefind can sort results by mixed floats and integers
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let asc_search = await pagefind.search("Robe", { sort: { mixed: "asc" } });
                let asc_data = await Promise.all(asc_search.results.map(result => result.data()));
                document.querySelector('[data-asc]').innerText = asc_data.map(d => d.url).join(', ');

                let desc_search = await pagefind.search("Robe", { sort: { mixed: "desc" } });
                let desc_data = await Promise.all(desc_search.results.map(result => result.data()));
                document.querySelector('[data-desc]').innerText = desc_data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-asc]" should contain "/robe/painte/, /robe/megapointe/, /robe/superspikie/"
        Then The selector "[data-desc]" should contain "/robe/superspikie/, /robe/megapointe/, /robe/painte/"

    Scenario: Pagefind excludes pages that aren't tagged with the provided sort option
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let asc_search = await pagefind.search("Robe", { sort: { color: "asc" } });
                let asc_data = await Promise.all(asc_search.results.map(result => result.data()));
                document.querySelector('[data-asc]').innerText = asc_data.map(d => d.url).join(', ');

                let desc_search = await pagefind.search("Robe", { sort: { color: "desc" } });
                let desc_data = await Promise.all(desc_search.results.map(result => result.data()));
                document.querySelector('[data-desc]').innerText = desc_data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-asc]" should contain "/robe/painte/, /robe/megapointe/"
        Then The selector "[data-desc]" should contain "/robe/megapointe/, /robe/painte/"
