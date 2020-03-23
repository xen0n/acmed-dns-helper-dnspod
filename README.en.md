# acmed-dns-helper-dnspod

Translation: [中文](README.md)

Simple helper binary for [ACMEd][acmed] to handle `dns-01` challenges
with the popular Chinese DNS provider [DNSPod][dnspod].

[acmed]: https://github.com/breard-r/acmed
[dnspod]: https://www.dnspod.cn

You need several environment variables set or the program panics:

* `DNSPOD_ID`: the ID generated along with the API token
* `DNSPOD_TOKEN`: the API token generated on DNSPod
* `DNSPOD_CONTACT_EMAIL`: ideally your DNSPod account email, mandatory according to [DNSPod docs][dnspod-doc-info]

[dnspod-doc-info]: https://www.dnspod.cn/docs/info.html

Not providing the correct `User-Agent` seems working (initial API tests were done with simple `curl` commands),
but it's better to be polite so the email thing is required too.

Usage:

```sh
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
