Feature: Node API Base Tests
# Background:
#     Given I have a "public/index.html" file with the body:
#         """
#         <p data-url>Nothing</p>
#         """
#     Given I have a "public/package.json" file with the content:
#         """
#         {
#             "name": "test",
#             "type": "module",
#             "version": "1.0.0",
#             "main": "index.js",
#             "dependencies": {
#                 "pagefind": "file:{{humane_cwd}}/../wrappers/node"
#             }
#         }
#         """

# Scenario: Build an index to disk via the api
#     Given I have a "public/index.js" file with the content:
#         """
#             import * as pagefind from "pagefind";

#             const run = async () => {
#                 const { index } = await pagefind.createIndex();
#                 await index.addHTMLFile({path: "dogs/index.html", content: "<html><body><h1>Testing, testing</h1></body></html>"});
#                 await index.writeFiles();
#                 console.log(`Successfully wrote files`);
#             }

#             run();
#         """
#     When I run "cd public && npm i && PAGEFIND_BINARY_PATH='{{humane_cwd}}/../target/release/pagefind' node index.js"
#     Then I should see "Successfully wrote files" in stdout
#     Then I should see the file "public/_pagefind/pagefind.js"
#     When I serve the "public" directory
#     When I load "/"
#     When I evaluate:
#         """
#         async function() {
#             let pagefind = await import("/_pagefind/pagefind.js");

#             let search = await pagefind.search("testing");

#             let data = await search.results[0].data();
#             document.querySelector('[data-url]').innerText = data.url;
#         }
#         """
#     Then There should be no logs
#     Then The selector "[data-url]" should contain "/dogs/"

# Scenario: Build an index to memory via the api
#     Given I have a "public/index.js" file with the content:
#         """
#             import * as pagefind from "pagefind";

#             const run = async () => {
#                 const { index } = await pagefind.createIndex();
#                 await index.addHTMLFile({path: "dogs/index.html", content: "<html><body><h1>Testing, testing</h1></body></html>"});
#                 const { files } = await index.getFiles();
#                 const jsFile = files.filter(file => file.path.includes("pagefind.js"))[0];
#                 console.log(jsFile.content.toString());
#             }

#             run();
#         """
#     When I run "cd public && npm i && PAGEFIND_BINARY_PATH='{{humane_cwd}}/../target/release/pagefind' node index.js"
#     Then I should see "pagefind_version=" in stdout
#     Then I should not see the file "public/_pagefind/pagefind.js"
