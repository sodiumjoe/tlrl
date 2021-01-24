const xdgBasedir = require("xdg-basedir");
const fs = require("fs");
const path = require("path");

const configPath = path.join(xdgBasedir.config, "tlrl", "tlrl.json");

let config;

try {
  config = JSON.parse(fs.readFileSync(configPath, "utf8"));
} catch (e) {}

const {
  kindle_email: kindle,
  gmail_username: gmail,
  gmail_application_password: password,
} = config;

module.exports = {
  kindle,
  gmail,
  password,
};
