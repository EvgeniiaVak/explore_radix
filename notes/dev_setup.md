# Developer Setup

## Install Scrypto tools

```shell
git clone https://github.com/radixdlt/radixdlt-scrypto.git
cd radixdlt-scrypto
cargo install --path ./simulator
```

Executables `resim`, `rtmc`, `scrypto` are now in `~/.cargo/bin`.

## resim

```shell
resim new-account
resim show-configs
resim reset
```

```shell
resim transfer 100 resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k account_sim1qnp8rpyvyuxz8lelmkzdh3ny0fdvhn2j8j3zrrcpxy0q47f2zl
```
where `resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k` is address for XRD