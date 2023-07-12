# SSO

SSO (Single Sign On) is one very common method of logging into things that is often used when connecting to networks like eduroam. 

## Protocols
Oauth is an incredibly common protocol for doing SSO.
SAML is also a common protocol that is used for sending information about the user.

## Usage as it relates to SecureW2
Thankfully, we dont have to care too much about protocols as these flows are typically handled in a browser through a series of redirects and login prompts. All we have to do is open the first webpage in the process and ensure we can intercept the data coming back at the end of the process.

### The initial URL
The initial URL is provided by the [cloudconfig](cloudconfig.md) file within the `webSSOUrl` entry.

This url looks something like `https://[domain].securew2.com/auth/[uuid]/[different uuid]?response_type=code&amp;client_id=clientid&amp;redirect_uri=urn:ietf:wg:oauth:2.0:oob:auto&amp;scope=test&amp;state=%TRANSACTIONID%&amp;include_granted_scopes=false`.

Here are some interesting parts that we can mostly ignore:
- `response_type=code` - specifies which oauth2 flow/procedure we are using
- `client_id=clientid` - yes it appears this is the real client id for all SecureW2 SSO flows
- `redirect_uri=urn:ietf:wg:oauth:2.0:oob:auto` - Apparrently this is some [now-deprecated](https://developers.google.com/identity/protocols/oauth2/resources/oob-migration) [google standard](https://googleapis.dev/ruby/google-api-client/v0.36.3/file.oauth-installed.html) that essentially tells the auth server to redirect to a page containing the auth codes that also tells the user to close their browser window.
- `scope=test` - lol nothing more permanant than a temporary solution I guess


The `state=%TRANSACTIONID%` is really the only part we have to care about because `%TRANSACTIONID%` is not the real transaction ID - its a placeholder value that we are expected to replace with a newly generated UUID4 value. This value will be sent back to us to confirm we are still in the same authentication session even despite all the redirects.

After filling in the state value with our new transaction ID, we have a URL that is ready to use.


### The resulting data

After the user has been through the rollercoaster of URLs, redirects, and login pages, they will eventually be deposited on a page telling them to close their browser.

This page will likely have a URL thats something like `https://[domain].securew2.com/auth/success?sessionId=<token>&state=<state UUID>&xsrfsign=`.

This URL, and indeed the resulting webpage as a whole, contains both a token (AKA sessionID AKA code) that allows us to authenticate future API calls. In addition to the URL, this token is also present in the HTML `<title>` element in the following format:

`Success state=[state uuid created earlier]&code=[code]&`


The code isn't an access token because typically these codes are sent to the client as query parameters, which is insecure. 



