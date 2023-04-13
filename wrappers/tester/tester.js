import * as pagefind from "pagefind";

const run = async () => {
    console.log(`Creating an index`);
    const result = await pagefind.createIndex();
    console.log(result);

    if (!result.index) return;
    let index = result.index;

    console.log(`\nAdding an HTML file to the index`);
    const page = await index.addHTMLFile({path: "dogs/index.html", content: "<html><body><h1>Testing, testing</h1></body></html>"});
    console.log(page);

    console.log(`\nAdding a custom file to the index`);
    const newfile = await index.addCustomRecord({
        url: "/elephants/",
        content: "Some testing content regarding elephants",
        language: "en",
        meta: {
            "title": "Elephants"
        }
    });
    console.log(newfile);

    console.log(`\nWriting files to memory`);
    const memfiles = await index.getFiles();
    console.log("Got files", memfiles);

    console.log(`\nWriting files to disk`);
    const files = await index.writeFiles();
    console.log("Wrote files", files);
}

run();