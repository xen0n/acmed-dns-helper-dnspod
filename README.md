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

## 编译

`cargo b --release` 即可。

## Usage

注意：以下均假定 `acmed-dns-helper-dnspod` 二进制已经放在 `$PATH` 下。

### For ACMEd

首先在你的 `acmed.toml` 里 `include` 本项目提供的 `dnspod_hooks.toml`，
然后配置相应的 `[[certificate]]` 项：

```toml
[[certificate]]
# ...
hooks = [
    # ...
    "dns-01-dnspod",
    # ...
]

# 你可以把这些东西写进 certificate section
# 也可以写到 global section 如果你所有域名都用同一个 token
[certificate.env]
DNSPOD_ID = "xxxxx"
DNSPOD_TOKEN = "xxxxx"
DNSPOD_CONTACT_EMAIL = "your-dnspod-email-address"
```

### In shell

```sh
# 设置好你的环境变量然后调用

# challenge
acmed-dns-helper-dnspod --domain "<domain name>" --proof "<proof>"

# clean up
acmed-dns-helper-dnspod --domain "<domain name>" --proof "<proof>" --clean
```

## 代码质量好烂！

我知道，懒得改了，现在没闲情逸致做一个真正的 DNSPod SDK，就先这样吧 #又不是不能用#……

## License

Apache-2.0
