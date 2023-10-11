Feature: URL tests
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-url>Nothing</p>
            """
        Given I have a "public/package.json" file with the content:
            """
            {
                "name": "test",
                "type": "module",
                "version": "1.0.0",
                "main": "index.js",
                "dependencies": {
                    "pagefind": "file:{{humane_cwd}}/../wrappers/node"
                }
            }
            """

    @platform-unix
    Scenario: Tag pages as external URLs
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";

             const run = async () => {
                 const { index } = await pagefind.createIndex();
                 await index.addCustomRecord({
                     url: "https://example.com/external-url/",
                     content: "Hello World!",
                     language: "en"
                 });
                 await index.writeFiles();
                 console.log(`Successfully wrote files`);
             }

             run();
            """
        When I run "cd public && npm i && PAGEFIND_BINARY_PATH={{humane_cwd}}/$TEST_BINARY node index.js"
        Then I should see "Successfully wrote files" in stdout
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
        Then The selector "[data-url]" should contain "https://example.com/external-url/"
