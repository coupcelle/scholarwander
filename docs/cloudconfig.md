# Cloudconfig

This describes the format of the configuration file referrred to as cloudconfig. This file is the main configuration file for eduroam and contained the primary differences between the versions for different schools.

## Signing

The file is stored on disk as a signed message in DER format.

To verify the signature, pass the DER formatted input into a tool like `openssl smime` with the following settings:
- read in DER input (`-inform der`)
- output the contents as SMIME (default openssl behavior)
- verify the message signature (`-verify`)
- dont verify the signer's certificate (`-noverify`)

Once you have the SMIME data, decode it as UTF8

## Structure

At the top level, the cloudconfig mainly contains a list of `configurations` which contain the most important information for conneting to networks, as well as some information about the `organization` issuing this config and information about their `subsciption` to secureW2's service.

Each `configuration` contains some metadata (i.e. name, some config) and defines a series of `reporting` and `action` entries that - presumably - are the reporting and action steps needed to configure the network. Presumably reporting steps are done first, then action steps. It appears as though the information needed for each step is contained within the xml object. 