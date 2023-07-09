# Cloudconfig

This describes the format of the configuration file referrred to as cloudconfig. This file is the main configuration file for eduroam and contained the primary differences between the versions for different schools.

## Protection

The file is stored on disk as a signed message in DER format.

To read it, pass the DER formatted input into a tool like `openssl smime` with the following settings:
- read in DER input (`-inform der`)
- output the contents as SMIME (default openssl behavior)
- verify the message signature (`-verify`)
- dont verify the signer's certificate (`-noverify`)

Once you have the SMIME data, decode it as UTF8