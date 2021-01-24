const nodemailer = require("nodemailer");
const config = require("./config");

const transporter = nodemailer.createTransport({
  host: "smtp.gmail.com",
  secure: false, // upgrade later with STARTTLS
  auth: {
    user: config.gmail,
    pass: config.password,
  },
});

const send = async (article) => {
  const { title, content, author, date, domain } = article;
  const head = [title, author, domain, date].filter(Boolean).join(" - ");
  const filename = `${head}.html`;

  const message = {
    from: `${config.gmail}@gmail.com`,
    to: config.kindle,
    subject: "convert",
    text: "Sent from tl;rl",
    attachments: [{ filename, content: new Buffer(content, "utf-8") }],
  };

  await transporter.sendMail(message);
};

module.exports = send;
