tl;rl
=====

Send a web page to your kindle for reading later from the command line.

## Configuration

1. [Install mercury-parser](https://github.com/postlight/mercury-parser).
2. [Create an application-specific password for gmail](https://myaccount.google.com/apppasswords).
  * Select App -> Other (custom)
3. Make sure you have a kindle email address set up.
  * [Go to Amazon](https://www.amazon.com)
  * -> Account & Lists
  * -> Your Content and Devices
  * -> Settings
  * -> Personal Document Settings
  * -> Send-to-Kindle E-Mail Settings
4. Make sure you have a kindle approved sender email address.
  * -> Approved Personal Document E-mail List
5. Create a `~/.tlrl.json` file:

```json
{
  "gmail_application_password": "from step 2",
  "kindle_email": "from step 3",
  "gmail_username": "from step 4",
}
```

## Usage

```
$ tlrl <url>
```

## Help

```
$ tlrl -h
```
