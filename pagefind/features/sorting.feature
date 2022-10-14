Feature: Result Sorting
    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-asc></p>
            <p data-desc></p>
            """
        Given I have a "public/robe/painte/index.html" file with the body:
            """
            <div data-pagefind-sort="released:April 19th 2022">
                <span data-lumens="14900" data-pagefind-sort="lumens[data-lumens]"></span>
                <h1>Robe <i data-pagefind-sort="model">Painte</i> ™️</h1>
            </div>
            """
        Given I have a "public/robe/megapointe/index.html" file with the body:
            """
            <p data-pagefind-sort="released">Sept 05 2017</p>
            <span data-pagefind-sort="lumens">9870</span>
            <h1>Robe <i data-pagefind-sort="model">MegaPointe</i> ™️</h1>
            """
        Given I have a "public/robe/superspikie/index.html" file with the body:
            """
            <p data-pagefind-sort="released">2018/10/16</p>
            <span data-pagefind-sort="lumens">5105</span>
            <h1>Robe <i data-pagefind-sort="model">SuperSpikie</i> ™️</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"

    Scenario: Pagefind can sort results by an ascending alphabetical attribute
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

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
                let pagefind = await import("/_pagefind/pagefind.js");

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

    Scenario: Pagefind can sort results by an automatically detected date string
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/_pagefind/pagefind.js");

                let asc_search = await pagefind.search("Robe", { sort: { released: "asc" } });
                let asc_data = await Promise.all(asc_search.results.map(result => result.data()));
                document.querySelector('[data-asc]').innerText = asc_data.map(d => d.url).join(', ');

                let desc_search = await pagefind.search("Robe", { sort: { released: "desc" } });
                let desc_data = await Promise.all(desc_search.results.map(result => result.data()));
                document.querySelector('[data-desc]').innerText = desc_data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-asc]" should contain "/robe/megapointe/, /robe/superspikie/, /robe/painte/"
        Then The selector "[data-desc]" should contain "/robe/painte/, /robe/superspikie/, /robe/megapointe/"
