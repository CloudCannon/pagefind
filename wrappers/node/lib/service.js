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

    const write = () => {
        // let e = new Encoder({ largeBigIntToFloat: false, useRecords: false });
        let msg = Buffer.from(JSON.stringify({
            message_id: 99458,
            payload: {
                type: 'Other',
                custom: 'We JSON now.'
            }
        })).toString('base64');
        console.log(msg);

        console.log("Writing");
        child.stdin.write(msg + ",", (err) => {
            if (err) { 
                console.error("Service stopped");
                process.exit(1);
            }
        });
    }

    setInterval(() => {
        write();
    }, 1000);
}
run();