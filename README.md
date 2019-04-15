tl;rl
=====

## SORRY, THIS IS BROKEN RIGHT NOW BECAUSE MERCURY API HAS SHUT DOWN. YOU CAN STILL GET IT TO WORK IF YOU RUN YOUR OWN MERCURY API INSTANCE (see https://github.com/postlight/mercury-parser)

Send a web page to your kindle for reading later from the command line.

## Configuration

1. [Get an api key for the Mercury API](https://mercury.postlight.com/web-parser/).
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
  "mercury_token":"from step 1",
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
