#!/usr/bin/env node

const yargs = require("yargs/yargs");
const { hideBin } = require("yargs/helpers");
const parse = require("./parse");
const config = require("./config");
const send = require("./email");

const args = yargs(hideBin(process.argv))
  .usage("$0 <url>", "send url to kindle")
  .demandCommand(1)
  .version().argv;

const { url } = args;

const run = async (url) => {
  const article = await parse(url);
  await send(article);
};

run(url);
