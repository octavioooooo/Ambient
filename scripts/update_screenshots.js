#!/bin/node
const util = require('node:util');
const exec = util.promisify(require('node:child_process').exec);
const { exit } = require("process");

const samples = [
    ["guest/rust/examples/basics/async", 1],
    ["guest/rust/examples/basics/input", 1],
    ["guest/rust/examples/basics/image", 3],
    ["guest/rust/examples/basics/primitives", 1],
    ["guest/rust/examples/basics/raw_text", 1],
    ["guest/rust/examples/games/tictactoe", 1],
    ["guest/rust/examples/ui/button", 60],
    ["guest/rust/examples/ui/dock_layout", 60],
    ["guest/rust/examples/ui/editors", 60],
    ["guest/rust/examples/ui/flow_layout", 60],
    ["guest/rust/examples/ui/rect", 60],
    ["guest/rust/examples/ui/screens", 60],
    ["guest/rust/examples/ui/slider", 60],
    ["guest/rust/examples/ui/text", 60],
]

async function run() {
    let errors = (await Promise.all(samples.map(async ([path, seconds], index) => {
        console.log(path, "running..");
        try {
            let res = await exec(`cargo run -- run ${path} --headless --screenshot-test ${seconds} --quic-interface-port ${9000 + index} --http-interface-port ${8999 + index}`);
            console.log(path, "was ok");
        } catch (err) {
            console.log(path, "failed");
            return { path, err };
        }
    }))).filter(x => x);
    for (const { path, err } of errors) {
        console.log(path, 'Error:', err);
    }
    if (errors.length > 0) {
        console.log("Exiting with status code 1");
        exit(1);
    }
}
run();
