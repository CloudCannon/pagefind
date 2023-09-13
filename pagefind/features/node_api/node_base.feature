Feature: Node API Base Tests
    Background:
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
    Scenario: Build a synthetic index to disk via the api
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";

             const run = async () => {
                 const { index } = await pagefind.createIndex();
                 await index.addHTMLFile({sourcePath: "dogs/index.html", content: "<html><body><h1>Testing, testing</h1></body></html>"});
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

                 let search = await pagefind.search("testing");

                 let data = await search.results[0].data();
                 document.querySelector('[data-url]').innerText = data.url;
             }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/dogs/"

    @platform-unix
    Scenario: Build a synthetic index with overridden urls to disk via the api
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";

             const run = async () => {
                 const { index } = await pagefind.createIndex();
                 await index.addHTMLFile({url: "/my-custom-url/", content: "<html><body><h1>Testing, testing</h1></body></html>"});
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

                 let search = await pagefind.search("testing");

                 let data = await search.results[0].data();
                 document.querySelector('[data-url]').innerText = data.url;
             }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/my-custom-url/"

    @platform-unix
    Scenario: Build a synthetic index to memory via the api
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";

             const run = async () => {
                 const { index } = await pagefind.createIndex();
                 await index.addHTMLFile({sourcePath: "dogs/index.html", content: "<html><body><h1>Testing, testing</h1></body></html>"});
                 const { files } = await index.getFiles();

                 const jsFile = files.filter(file => file.path.includes("pagefind.js"))[0];
                 console.log(jsFile.content.toString());

                 console.log(`JS is at ${jsFile.path}`);

                 const fragments = files.filter(file => file.path.includes("fragment"));
                 console.log(`${fragments.length} fragment(s)`);
             }

             run();
            """
        When I run "cd public && npm i && PAGEFIND_BINARY_PATH={{humane_cwd}}/$TEST_BINARY node index.js"
        Then I should see "pagefind_version=" in stdout
        Then I should see "JS is at pagefind.js" in stdout
        Then I should see "1 fragment(s)" in stdout
        Then I should not see the file "public/pagefind/pagefind.js"

    @platform-unix
    Scenario: Build a true index to disk via the api
        Given I have a "public/custom_files/real/index.html" file with the body:
            """
             <p>A testing file that exists on disk</p>
            """
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";

             const run = async () => {
                 const { index } = await pagefind.createIndex();
                 await index.addDirectory({path: "custom_files"});
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

                 let search = await pagefind.search("testing");

                 let data = await search.results[0].data();
                 document.querySelector('[data-url]').innerText = data.url;
             }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/real/"

    @platform-unix
    Scenario: Build a blended index to memory via the api
        Given I have a "public/custom_files/real/index.html" file with the body:
            """
             <p>A testing file that exists on disk</p>
            """
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";
             import fs from "fs";
             import path from "path";

             const run = async () => {
                 const { index } = await pagefind.createIndex();
                 await index.addDirectory({ path: "custom_files" });
                 await index.addCustomRecord({
                     url: "/synth/",
                     content: "A testing file that doesn't exist.",
                     language: "en"
                 });
                 const { files } = await index.getFiles();

                 for (const file of files) {
                     const output_path = path.join("pagefind", file.path);
                     const dir = path.dirname(output_path);
                     if (!fs.existsSync(dir)){
                         fs.mkdirSync(dir, { recursive: true });
                     }

                     fs.writeFileSync(output_path, file.content);
                 }
                 console.log("Donezo!");
             }

             run();
            """
        When I run "cd public && npm i && PAGEFIND_BINARY_PATH={{humane_cwd}}/$TEST_BINARY node index.js"
        Then I should see "Donezo!" in stdout
        Then I should see the file "public/pagefind/pagefind.js"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
             async function() {
                 let pagefind = await import("/pagefind/pagefind.js");

                 let search = await pagefind.search("testing");

                 let pages = await Promise.all(search.results.map(r => r.data()));
                 document.querySelector('[data-url]').innerText = pages.map(p => p.url).sort().join(", ");
             }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/real/, /synth/"

    @platform-unix
    Scenario: Build an index to a custom disk location via the api
        Given I have a "output/index.html" file with the body:
            """
             <p data-url>Nothing</p>
            """
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";

             const run = async () => {
                 const { index } = await pagefind.createIndex();
                 await index.addHTMLFile({sourcePath: "dogs/index.html", content: "<html><body><h1>Testing, testing</h1></body></html>"});
                 await index.writeFiles({ outputPath: "../output/pagefind" });
                 console.log(`Successfully wrote files`);
             }

             run();
            """
        When I run "cd public && npm i && PAGEFIND_BINARY_PATH={{humane_cwd}}/$TEST_BINARY node index.js"
        Then I should see "Successfully wrote files" in stdout
        Then I should see the file "output/pagefind/pagefind.js"
        When I serve the "output" directory
        When I load "/"
        When I evaluate:
            """
             async function() {
                 let pagefind = await import("/pagefind/pagefind.js");

                 let search = await pagefind.search("testing");

                 let data = await search.results[0].data();
                 document.querySelector('[data-url]').innerText = data.url;
             }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/dogs/"

    @platform-unix
    Scenario: An index is not consumed on write
        Given I have a "output/index.html" file with the body:
            """
             <p data-url>Nothing</p>
            """
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";

             const run = async () => {
                 const { index } = await pagefind.createIndex();
                 await index.addHTMLFile({sourcePath: "dogs/index.html", content: "<html><body><h1>Testing, testing</h1></body></html>"});
                 await index.writeFiles({ outputPath: "../output/pagefind" });

                 await index.addHTMLFile({sourcePath: "rabbits/index.html", content: "<html><body><h1>Testing, testing</h1></body></html>"});
                 const { files } = await index.getFiles();

                 const fragments = files.filter(file => file.path.includes("fragment"));
                 console.log(`${fragments.length} fragment(s)`);

                 await index.addHTMLFile({sourcePath: "cats/index.html", content: "<html><body><h1>Testing, testing</h1></body></html>"});
                 await index.writeFiles({ outputPath: "./pagefind" });

                 console.log(`Successfully wrote files`);
             }

             run();
            """
        When I run "cd public && npm i && PAGEFIND_BINARY_PATH={{humane_cwd}}/$TEST_BINARY node index.js"
        Then I should see "Successfully wrote files" in stdout
        Then I should see "2 fragment(s)" in stdout
        Then I should see the file "output/pagefind/pagefind.js"
        When I serve the "output" directory
        When I load "/"
        When I evaluate:
            """
             async function() {
                 let pagefind = await import("/pagefind/pagefind.js");

                 let search = await pagefind.search("testing");

                 let pages = await Promise.all(search.results.map(r => r.data()));
                 document.querySelector('[data-url]').innerText = pages.map(p => p.url).sort().join(", ");
             }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/dogs/"
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
             async function() {
                 let pagefind = await import("/pagefind/pagefind.js");

                 let search = await pagefind.search("testing");

                 let pages = await Promise.all(search.results.map(r => r.data()));
                 document.querySelector('[data-url]').innerText = pages.map(p => p.url).sort().join(", ");
             }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/cats/, /dogs/, /rabbits/"

    @platform-unix
    Scenario: Pagefind service config
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";

             const run = async () => {
                 const { index } = await pagefind.createIndex({
                     rootSelector: "h1",
                     excludeSelectors: ["span"],
                     keepIndexUrl: true,
                 });
                 await index.addHTMLFile({sourcePath: "dogs/index.html", content: "<h1>Testing, <span>testing</span></h1>"});
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

                 let search = await pagefind.search("testing");

                 let data = await search.results[0].data();
                 document.querySelector('[data-url]').innerText = `${data.url} • ${data.content}`;
             }
            """
        Then There should be no logs
        Then The selector "[data-url]" should contain "/dogs/index.html • Testing,"

    @platform-unix
    Scenario: Pagefind error handling
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";

             const bad = async () => {
                 const { index } = await pagefind.createIndex();
                 await index.deleteIndex();
                 const { errors, files } = await index.getFiles();
                 console.log(JSON.stringify(errors));

                 try {
                     const response = await pagefind.createIndex({
                         rootSelector: 5
                     });
                 } catch(e) {
                     console.log(e.toString());
                 }
             }
             bad();
            """
        When I run "cd public && npm i && PAGEFIND_BINARY_PATH={{humane_cwd}}/$TEST_BINARY node index.js"
        Then I should see "invalid type: integer `5`" in stdout
        Then I should see "Index has been deleted from the Pagefind service and no longer exists" in stdout

    @platform-unix
    Scenario: Pagefind empty index returns assets
        Given I have a "public/index.js" file with the content:
            """
             import * as pagefind from "pagefind";

             const run = async () => {
                 const { index } = await pagefind.createIndex();
                 const { errors, files } = await index.getFiles();
                 console.log(files.map(f => f.path).join(', '));
             }
             run();
            """
        When I run "cd public && npm i && PAGEFIND_BINARY_PATH={{humane_cwd}}/$TEST_BINARY node index.js"
        Then I should see "pagefind.js" in stdout
        Then I should see "pagefind-ui.js" in stdout
        Then I should see "pagefind-ui.css" in stdout
        Then I should see "pagefind-modular-ui.js" in stdout
        Then I should see "pagefind-modular-ui.css" in stdout
        Then I should see "wasm.unknown.pagefind" in stdout
