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

### Instantiate a component (calling a function)

```shell
resim call-function [package_address] [blueprint_name] [function]
```

### Use the component (calling a method)

```shell
resim call-method [component_address] [method]
```

Example with parameters:

```shell
resim call-method component_sim1qw2a4jxqpn05zp7hyy54gdan788ct5xgjwj8n02292js0f3yx2 burn_banana resource_sim1qx2a4jxqpn05zp7hyy54gdan788ct5xgjwj8n02292js2hflez:1
```


FIXME:
```
vgeniia@evgeniias-air explore_radix % resim call-method component_sim1qvr58y4d7k49c9resqtc4vt0uywd8vvlznqv79f3a72sld2neg request_amount 120
Transaction Status: COMMITTED FAILURE: KernelError(DropNodeFailure(Object("0d5d1a3a96f86b893c6d0a484fea7d0e115c62170e19efa884a2ed01000000")))
Transaction Fee: 0.0801745 XRD used for execution, 0 XRD used for royalty, 0 XRD in bad debt
Cost Units: 100000000 limit, 801745 consumed, 0.0000001 XRD per cost unit, 0% tip
Logs: 1
└─ [INFO ] Moving 120 HelloToken to another account
Events: 0
Balance Changes: 0
Direct Vault Updates: 1
└─ Vault: 0e81e47a19e6b29b0a65b9591762ce5143ed30d0261e5d24a320172d000000, Address: resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k, Delta: -0.0801745
New Entities: 0
Error: TransactionFailed(KernelError(DropNodeFailure(Object("0d5d1a3a96f86b893c6d0a484fea7d0e115c62170e19efa884a2ed01000000"))))
```
