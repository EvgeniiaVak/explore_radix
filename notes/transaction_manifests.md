# Transaction Manifests

## Generate .rtm file using resim

```shell
resim transfer ["AMOUNT"] ["RESOURCE_ADDRESS"] [COMPONENT_ADDRESS_OF_RECIPIENT] --manifest transfer_tokens.rtm
```

```shell
evgeniia@Evgeniias-MacBook-Air explore_radix % resim new-account
A new account has been created!
Account component address: account_sim1qsxvn2vzgwqc7r3zjhjv7jr9mjq5vwyh56s3enx9ctcs3l2e4j
Public key: 027091275c8acb0f0584f2ae24e81cd3f9fabafddaca271ec58b9a2143b03bbd64
Private key: 6067cb63483549551208e7380eb0414cb0676bb1f67cdbd79e9949ac1b970e18
Owner badge: resource_sim1qgyekk0znhrdwfsfa98eahvzyv5e5ccawpwdzcrcaerst9sqe2:#1#
Account configuration in complete. Will use the above account as default.
evgeniia@Evgeniias-MacBook-Air explore_radix % resim new-account
A new account has been created!
Account component address: account_sim1qsdvrcntfe7twdkxvh2ly65g99as3ya774fl6mfzr2aswrnqfk
Public key: 03e5886bd2b7e20b9f9c93cb48c0dae528b930799825ad0da583a0435f76612067
Private key: 3c94532679c69036b7a65b062a4dd52b348c2d9acaf91ee9b3347e049641aa78
Owner badge: resource_sim1qt69rsc53z3f4zmv7hztvz4uc8vxt560r5k4plm9cuxszpez5y:#1#
```

```shell
resim transfer 500 resource_sim1qyqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqs6d89k account_sim1qsdvrcntfe7twdkxvh2ly65g99as3ya774fl6mfzr2aswrnqfk --manifest transfer_tokens.rtm
```
