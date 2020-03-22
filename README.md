# acmed-dns-helper-dnspod

Translation: [English](README.en.md)

这是一个简单的 helper 程序，让 [ACMEd][acmed] 能够处理 [DNSPod][dnspod]
所管理域名的 `dns-01` challenge。

[acmed]: https://github.com/breard-r/acmed
[dnspod]: https://www.dnspod.cn

## 前置条件

你需要设置几个环境变量，否则会 panic:

* `DNSPOD_ID`: 跟 API token 一起生成的 ID
* `DNSPOD_TOKEN`: DNSPod 网页上生成的 API token
* `DNSPOD_CONTACT_EMAIL`: 最好填你的 DNSPod 账号邮箱，根据[文档][dnspod-doc-info]描述，必须提供

[dnspod-doc-info]: https://www.dnspod.cn/docs/info.html

不提供正确的 `User-Agent` 貌似也行（最初的 API 测试是用简单的 `curl` 命令进行的），不过做人最好礼貌一点，所以 Email 的部分也做成了必须的。

## Usage

```sh
# challenge
acmed-dns-helper-dnspod --domain "<domain name>" --proof "<proof>"

# clean up
acmed-dns-helper-dnspod --domain "<domain name>" --proof "<proof>" --clean
```

## License

Apache-2.0
