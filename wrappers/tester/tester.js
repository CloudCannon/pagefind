import * as pagefind from "pagefind";

const run = async () => {
    console.log(`Creating an index`);
    const result = await pagefind.createIndex();
    console.log(result);

    console.log(`\nAdding a file to the index`);
    const page = await result.index.addFile("dogs/index.html", "<html><body><h1>Testing, testing</h1></body></html>");
    console.log(page);
}

run();