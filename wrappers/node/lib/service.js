import child_process from 'child_process';
import fs from 'fs';
import path from 'path';

const run = async () => {
    let child = child_process.spawn("../../target/debug/pagefind", [`--service`], {
        windowsHide: true,
        stdio: ['pipe', 'pipe', 'inherit'],
        cwd: process.cwd(),
    });

    let partMsg = "";

    const processMsg = (d) => {
        const data = Buffer.from(d.toString(), 'base64');
        const decoded = JSON.parse(data);
        console.log("DATA", decoded);
    }

    child.stdout.on('data', (d) => {
        let chunk = d.toString();
        console.log("RAW", chunk);
        try {
            while (chunk.length) {
                let delim = chunk.indexOf(',');
                if (!delim) {
                    partMsg = partMsg + chunk;
                    return;
                }

                let msg = chunk.slice(0, delim);
                processMsg(partMsg + msg);
                partMsg = "";

                chunk = chunk.slice(delim+1);
            }
        } catch {}
    });

    child.stdin.on('error', (e) => {
        console.error("Service stopped");
        process.exit(1);
    });
    child.on('error', (e) => {
        console.error("Service stopped");
        process.exit(1);
    });

    const write = (msg) => {
        // let e = new Encoder({ largeBigIntToFloat: false, useRecords: false });
        let encoded = Buffer.from(JSON.stringify(msg)).toString('base64') + ",";
        console.log("Writing", encoded);
        child.stdin.write(encoded, (err) => {
            if (err) { 
                console.error("Service stopped");
                process.exit(1);
            }
        });
    }

    write({
        message_id: 1,
        payload: {
            type: 'NewIndex',
            id: 3
        }
    });

    await new Promise(r => setTimeout(r, 1000));

    write({
        message_id: 2,
        payload: {
            type: 'AddFile',
            index_id: 3,
            file_path: 'index.html',
            file_contents: `<html><body><p>Hello World</p></body></html>`
        }
    });

    await new Promise(r => setTimeout(r, 1000));

    write({
        message_id: 3,
        payload: {
            type: 'AddFile',
            index_id: 3,
            file_path: 'cats.html',
            file_contents: `<html><body><p>Hello Cats</p></body></html>`
        }
    });

    await new Promise(r => setTimeout(r, 1000));

    write({
        message_id: 4,
        payload: {
            type: 'WriteFiles',
            index_id: 3
        }
    });

    await new Promise(r => setTimeout(r, 2000));

    process.exit(0);
}
run();