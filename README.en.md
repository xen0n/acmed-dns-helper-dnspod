# acmed-dns-helper-dnspod

Translation: [中文](README.md)

Simple helper binary for [ACMEd][acmed] to handle `dns-01` challenges
with the popular Chinese DNS provider [DNSPod][dnspod].

[acmed]: https://github.com/breard-r/acmed
[dnspod]: https://www.dnspod.cn

## Prerequisites

You need several environment variables set or the program panics:

* `DNSPOD_ID`: the ID generated along with the API token
* `DNSPOD_TOKEN`: the API token generated on DNSPod
* `DNSPOD_CONTACT_EMAIL`: ideally your DNSPod account email, mandatory according to [DNSPod docs][dnspod-doc-info]

[dnspod-doc-info]: https://www.dnspod.cn/docs/info.html

Not providing the correct `User-Agent` seems working (initial API tests were done with simple `curl` commands),
but it's better to be polite so the email thing is required too.

## Compilation

`cargo b --release` works, but the resulting binary links to OpenSSL
dynamically on Linux, which might mean portability problems.
In this case you may use `make rel` to get a static binary instead (Docker is
required).

## Usage

NOTE: It is assumed the `acmed-dns-helper-dnspod` binary is already in your `$PATH`.

### For ACMEd

`[include]` the provided `dnspod_hooks.toml` in your `acmed.toml`, then
reference the hook in the `[[certificate]]` section:

```toml
[[certificate]]
# ...
hooks = [
    # ...
    "dns-01-dnspod",
    # ...
]

# You can put these in the certificate section
# Or in the global section if all your domains share this one token
[certificate.env]
DNSPOD_ID = "xxxxx"
DNSPOD_TOKEN = "xxxxx"
DNSPOD_CONTACT_EMAIL = "your-dnspod-email-address"
```

### In shell

```sh
# export your environment variables first

# challenge
acmed-dns-helper-dnspod --domain "<domain name>" --proof "<proof>"

# clean up
acmed-dns-helper-dnspod --domain "<domain name>" --proof "<proof>" --clean
```

## Code quality sux!

I know, but I'm not in a mood to make a full-fledged SDK for DNSPod, that's all.
At least #itworks#...

## License

Apache-2.0
