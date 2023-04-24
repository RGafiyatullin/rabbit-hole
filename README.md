# `alice` — a swiss-army knife (wannabe) for working with digital signatures


## Keys

Two types of keys are supported:
- full-keys — a key sufficient to produce a signature on its own
- S4 key-shares — key-shards used for threshold signatures (using Shamir Secret Sharing Scheme)

Keys are stored in local-storage and are identified by their names.

---

### Generating keys

A full key can be generated as follows:

```shell
# in
alice keys gen --curve secp256k1 k1-full
# in
alice keys export k1-full
# out
!full_key
curve: secp256k1
value: secp256k1:96e5ce4de67d33d450647ee85f09bc5192703e74bd91c47cfa55baab5e68f628
```

A key-share can be produced from a full-key:

```shell
# in
alice s4 gen --key-id k1-full --threshold 2 k1-scheme-1
# in
alice s4 issue-share --key-id k1-s1:1 k1-scheme-1 secp256k1:0000000000000000000000000000000000000000000000000000000000000101
# in
alice s4 issue-share --key-id k1-s1:2 k1-scheme-1 secp256k1:0000000000000000000000000000000000000000000000000000000000000202
# in
alice s4 issue-share --key-id k1-s1:3 k1-scheme-1 secp256k1:0000000000000000000000000000000000000000000000000000000000000303

# in
alice keys export k1-s1:1
# out
!s4_share
curve: secp256k1
threshold: 2
public_key: secp256k1:022e8ff11b5c98c9dcfc74539c4fcd8d747ed1a4df0f8beed59853f3956c2f5e36
x: secp256k1:0000000000000000000000000000000000000000000000000000000000000101
y: secp256k1:ba8582fa9a0e3c37a56663014acae81fa28213abc07f6facd047e1375458f77a

# in
alice keys export k1-s1:2
# out
!s4_share
curve: secp256k1
threshold: 2
public_key: secp256k1:022e8ff11b5c98c9dcfc74539c4fcd8d747ed1a4df0f8beed59853f3956c2f5e36
x: secp256k1:0000000000000000000000000000000000000000000000000000000000000202
y: secp256k1:9face74205071244d1c767dbff92d79870fc1de0c758cb8bbcf7993f2bb13bcc

# in
alice keys export k1-s1:3
# out
!s4_share
curve: secp256k1
threshold: 2
public_key: secp256k1:022e8ff11b5c98c9dcfc74539c4fcd8d747ed1a4df0f8beed59853f3956c2f5e36
x: secp256k1:0000000000000000000000000000000000000000000000000000000000000303
y: secp256k1:6047b312a7088291d8df1fa89dcb71ab613770dd94a0932314ac9114d4d945cc
```

### Importing the keys


A full key can be imported as follows:

```yaml
# in
alice keys import k2-full <<YAML     
# out
!full_key
curve: secp256k1
value: secp256k1:6ac1a6a71968629c84b7658b3e3d5188073c224dd47c87e06867a84c3800320f
YAML

# in
alice keys list | grep k-k1-1
# out
- k-k1-1
```



S4-shares can be imported as follows:

```yaml
# in
alice keys import k1-2-of-3:1 <<YAML
!s4_share
curve: secp256k1
threshold: 2
public_key: secp256k1:033016cf0bd874d48c1e35d00f5eda3d02cfa0bb4e4b66dc568c7e1cdd7f1c3271
x: secp256k1:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee95f
y: secp256k1:2845f19259449623442c6e9b5a3bf6aff6d5b87dae56d898c23af82f44a4c981
YAML

# in
alice keys import k1-2-of-3:2 <<YAML
!s4_share
curve: secp256k1
threshold: 2
public_key: secp256k1:033016cf0bd874d48c1e35d00f5eda3d02cfa0bb4e4b66dc568c7e1cdd7f1c3271
x: secp256k1:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e929
y: secp256k1:ac47ca09c0887ec3d7524b566e1b3865009eaa03f6e62ca491ebd24ee99f8afa
YAML

# in
alice keys import k1-2-of-3:3 <<YAML
!s4_share
curve: secp256k1
threshold: 2
public_key: secp256k1:033016cf0bd874d48c1e35d00f5eda3d02cfa0bb4e4b66dc568c7e1cdd7f1c3271
x: secp256k1:87ac5d1ddfa64329d8548b34c25ee5edea790e1311eb55467461114c31f1a011
y: secp256k1:4361b6c1e51f47739f4580f7e0047b6baf5c9196f148470f2c0460633b8ea52f
YAML

# in
alice keys list | grep k1-2-of-3
# out
- k1-2-of-3:1
- k1-2-of-3:2
- k1-2-of-3:3
```

### Securely generating key-shares

The example above shows how to import key-shares, however this requires that there is a trusted dealer party, which defeats the point of using threshold signatures.

In order to come up with a set of key-shares without ever having the key assembled in any party's hands, some sort of Distributed Key Generation should be used.

Alice implements supports DKG according to the [CSI-RAShi: Distributed key generation for CSIDH](https://ia.cr/2020/1323) by Ward Beullens, Lucas Disson, Robi Pedersen, and Frederik Vercauteren.

The following example will use three separate storages, to demonstrate how a three parties can generate their key-shares.


```shell
# in
for i in {1..3}; do alias alice-$i="alice --storage-path ~/.alice/alice-$i"; done
```

First each party issues their own deals:

```yaml
# in
alice-1 dkg csi-rashi deal --curve secp256k1 key-1:1 <<YAML
threshold: 2
this: 0
shamir_xs:
 - secp256k1:0000000000000000000000000000000000000000000000000000000000000101
 - secp256k1:0000000000000000000000000000000000000000000000000000000000000202
 - secp256k1:0000000000000000000000000000000000000000000000000000000000000303
YAML
# out
# to be broadcast to all parties
commitment:
- secp256k1:029b26f531c2035a9017c906b739fd86a7725c5d888eec9fa397c16365a09b2a06
- secp256k1:033ebe3ec35e3211eddf80bb9518f101a3773e88bda427bbf59d0e11e091922496
deals:
  # to be sent privately to the alice-3
  secp256k1:0000000000000000000000000000000000000000000000000000000000000303: secp256k1:495d149f6e4a7ce20a8976dde8264b453b129630953921a01ee35d353a78e717
  # to be sent privately to the alice-2
  secp256k1:0000000000000000000000000000000000000000000000000000000000000202: secp256k1:932d4f54ed4ce200db860d685c6b6946f3d760601da831dcc377ca89a00c65a6

# in
alice-2 dkg csi-rashi deal --curve secp256k1 key-1:2 <<YAML
threshold: 2
this: 1
shamir_xs:
 - secp256k1:0000000000000000000000000000000000000000000000000000000000000101
 - secp256k1:0000000000000000000000000000000000000000000000000000000000000202
 - secp256k1:0000000000000000000000000000000000000000000000000000000000000303
YAML
# out
# to be broadcast to all parties
commitment:
- secp256k1:03e1274ac7d6de672f16134aa9ed3fd0b07c9349c40cb4a00c5494f88a7dd39000
- secp256k1:031a7ce8eba6698a69fdbff73f5822d28ac512014ca5abcc5b9885ea3c40342993
deals:
  # to be sent privately to alice-1
  secp256k1:0000000000000000000000000000000000000000000000000000000000000101: secp256k1:65a862b4ab103f8ae058f97a4cb3a01e926d9a6861ce91ce0edf7b950221bcde
  # to be sent privately to alice-3
  secp256k1:0000000000000000000000000000000000000000000000000000000000000303: secp256k1:a4340a82d64f0927580a9d194cd3a48bf0a2404f4b4a18353460f2193c7f8550

# in
alice-3 dkg csi-rashi deal --curve secp256k1 key-1:3 <<YAML
threshold: 2
this: 2
shamir_xs:
 - secp256k1:0000000000000000000000000000000000000000000000000000000000000101
 - secp256k1:0000000000000000000000000000000000000000000000000000000000000202
 - secp256k1:0000000000000000000000000000000000000000000000000000000000000303
YAML
# out
# to be broadcast to all parties
commitment:
- secp256k1:03550df4a601dafea95a4251539eb4b799fa872441b8268368578d688e94417263
- secp256k1:021cb6ea27fe7603e7350de2da01f9986e2d2770866c48870582df1787b756e8cc
deals:
  # to be sent privately to alice-2
  secp256k1:0000000000000000000000000000000000000000000000000000000000000202: secp256k1:a6835aebacb2bcb8ef287ef8f08d72d76cc3141c4cb94ed5e05decefff378ddb
  # to be sent privately to alice-1
  secp256k1:0000000000000000000000000000000000000000000000000000000000000101: secp256k1:78831238da6ea9993e22968d88e46888115808e84966469f8e4645c945265540
```

The output contains two fields:
- commitment;
- deals.

Commitment — is broadcast to every other party.
Each Deal — is sent to the corresponding party privately.

Each party aggregates its own key share based on the received Commitments and Deals.

`alice-1`:

```yaml
# in
alice-1 dkg csi-rashi aggregate key-1:1 <<YAML
commitments:
  # received from alice-2
  secp256k1:0000000000000000000000000000000000000000000000000000000000000202:
    - secp256k1:03e1274ac7d6de672f16134aa9ed3fd0b07c9349c40cb4a00c5494f88a7dd39000
    - secp256k1:031a7ce8eba6698a69fdbff73f5822d28ac512014ca5abcc5b9885ea3c40342993
  # received from alice-3
  secp256k1:0000000000000000000000000000000000000000000000000000000000000303:
    - secp256k1:03550df4a601dafea95a4251539eb4b799fa872441b8268368578d688e94417263
    - secp256k1:021cb6ea27fe7603e7350de2da01f9986e2d2770866c48870582df1787b756e8cc
deals:
  # received from alice-2
  secp256k1:0000000000000000000000000000000000000000000000000000000000000202: secp256k1:65a862b4ab103f8ae058f97a4cb3a01e926d9a6861ce91ce0edf7b950221bcde
  # received from alice-3
  secp256k1:0000000000000000000000000000000000000000000000000000000000000303: secp256k1:78831238da6ea9993e22968d88e46888115808e84966469f8e4645c945265540
YAML

# in
alice-1 keys export key-1:1
# out
!s4_share
curve: secp256k1
threshold: 2
# public-key is supposed to be the same for all parties
public_key: secp256k1:03812a6cdbb812f050eb6d3fcc591d7ce48995db101ed6ae8fc22a7e540ec23829
x: secp256k1:0000000000000000000000000000000000000000000000000000000000000101
y: secp256k1:bb28fef7f1ce3043cafe33faa6488ff095b2f0f9a2037a4b455f9aaf7cb1b512
```

`alice-2`:

```yaml
# in
alice-2 dkg csi-rashi aggregate key-1:2 <<YAML
commitments:
  # received from alice-1
  secp256k1:0000000000000000000000000000000000000000000000000000000000000101:
    - secp256k1:029b26f531c2035a9017c906b739fd86a7725c5d888eec9fa397c16365a09b2a06
    - secp256k1:033ebe3ec35e3211eddf80bb9518f101a3773e88bda427bbf59d0e11e091922496
  # received from alice-3
  secp256k1:0000000000000000000000000000000000000000000000000000000000000303:
    - secp256k1:03550df4a601dafea95a4251539eb4b799fa872441b8268368578d688e94417263
    - secp256k1:021cb6ea27fe7603e7350de2da01f9986e2d2770866c48870582df1787b756e8cc
  
deals:
  # received from alice-1
  secp256k1:0000000000000000000000000000000000000000000000000000000000000101: secp256k1:932d4f54ed4ce200db860d685c6b6946f3d760601da831dcc377ca89a00c65a6
  # received from alice-2
  secp256k1:0000000000000000000000000000000000000000000000000000000000000303: secp256k1:a6835aebacb2bcb8ef287ef8f08d72d76cc3141c4cb94ed5e05decefff378ddb
YAML

# in
alice-2 keys export key-1:2
# out
!s4_share
curve: secp256k1
threshold: 2
# public-key is supposed to be the same for all parties
public_key: secp256k1:03812a6cdbb812f050eb6d3fcc591d7ce48995db101ed6ae8fc22a7e540ec23829
x: secp256k1:0000000000000000000000000000000000000000000000000000000000000202
y: secp256k1:be9ee0dc5aaf4312e6e057ab19bc7e74e77384f191a5357885a38fc3ee5e5357
```

`alice-3`:

```yaml
# in
alice-3 dkg csi-rashi aggregate key-1:3 <<YAML
commitments:
  # received from alice-1
  secp256k1:0000000000000000000000000000000000000000000000000000000000000101:
    - secp256k1:029b26f531c2035a9017c906b739fd86a7725c5d888eec9fa397c16365a09b2a06
    - secp256k1:033ebe3ec35e3211eddf80bb9518f101a3773e88bda427bbf59d0e11e091922496
  # received from alice-2
  secp256k1:0000000000000000000000000000000000000000000000000000000000000202:
    - secp256k1:03e1274ac7d6de672f16134aa9ed3fd0b07c9349c40cb4a00c5494f88a7dd39000
    - secp256k1:031a7ce8eba6698a69fdbff73f5822d28ac512014ca5abcc5b9885ea3c40342993
  
deals:
  # received from alice-1
  secp256k1:0000000000000000000000000000000000000000000000000000000000000101: secp256k1:78831238da6ea9993e22968d88e46888115808e84966469f8e4645c945265540
  # received from alice-2
  secp256k1:0000000000000000000000000000000000000000000000000000000000000202: secp256k1:78831238da6ea9993e22968d88e46888115808e84966469f8e4645c945265540
YAML

# in
alice-3 keys export key-1:3
# out
!s4_share
!s4_share
curve: secp256k1
threshold: 2
# public-key is supposed to be the same for all parties
public_key: secp256k1:03812a6cdbb812f050eb6d3fcc591d7ce48995db101ed6ae8fc22a7e540ec23829
x: secp256k1:0000000000000000000000000000000000000000000000000000000000000303
y: secp256k1:c589c81033d4230b1c73947f69ff4e38302f543a3390440f8f2fc11c735f2fb5
```

As a result, each party has a key-share:
```shell
# ➜  rabbit-hole git:(doc) ✗ 
# in
alice-1 keys list
# out
- key-1:1

# ➜  rabbit-hole git:(doc) ✗ 
# in
alice-2 keys list
# out
- key-1:2

# ➜  rabbit-hole git:(doc) ✗ 
# in 
alice-3 keys list
# out
- key-1:3
```

Now any two of the parties can produce a valid signature for the same public key.


## Signatures

### Signing with a full-key

A full-key can be used to produce a signature.

```yaml
# in
alice sign schnorr --key-id k1-full <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
YAML

# out
y: secp256k1:022e8ff11b5c98c9dcfc74539c4fcd8d747ed1a4df0f8beed59853f3956c2f5e36
r: secp256k1:02c392d1a48e02bb0f1cc201e55bdfcb12ae98fc7ba77d2dfaabfcaa33a8e454fe
s: secp256k1:f752a99b95c7ca7b0de75debb5155638fb17dda05fd5ed7b1b786d6a2ee41099
```

### Verifying a signature

Verifying a valid signature:

```yaml
# in
alice verify schnorr --curve secp256k1 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
signature:
  y: secp256k1:022e8ff11b5c98c9dcfc74539c4fcd8d747ed1a4df0f8beed59853f3956c2f5e36
  r: secp256k1:02c392d1a48e02bb0f1cc201e55bdfcb12ae98fc7ba77d2dfaabfcaa33a8e454fe
  s: secp256k1:f752a99b95c7ca7b0de75debb5155638fb17dda05fd5ed7b1b786d6a2ee41099
YAML

# out
true
```

Verifying an invalid signature:

```yaml
# in
alice verify schnorr --curve secp256k1 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     We have signed no such thing!
signature:
  y: secp256k1:022e8ff11b5c98c9dcfc74539c4fcd8d747ed1a4df0f8beed59853f3956c2f5e36
  r: secp256k1:02c392d1a48e02bb0f1cc201e55bdfcb12ae98fc7ba77d2dfaabfcaa33a8e454fe
  s: secp256k1:f752a99b95c7ca7b0de75debb5155638fb17dda05fd5ed7b1b786d6a2ee41099
YAML

# out
false
```

### Producing a signature using S4-shares.

We are going to use two out of three key-shares we produced earlier:

```shell
# in
alice keys list k1-2-of | head -n 2

# out
- k1-2-of-3:1
- k1-2-of-3:2
```

We will export these keys into the separate storages for the purity of the experiment:

```shell
# in
for i in {1..3}; do alias alice-$i="alice --storage-path ~/.alice/alice-$i"; done

# in
alice keys export k1-2-of-3:1 | alice-1 keys import the-key

# in
alice keys export k1-2-of-3:2 | alice-2 keys import the-key
```

To start the process, every participant generates a set of instance-keys and commits to them. These commitments are subsequently made public. 
To reduce the number of steps required for each signature, it is advisable to produce a substantial number of instance-key pairs.

```shell
# in
alice-1 keys export the-key | grep '^x'
# out
x: secp256k1:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee95f

# in
alice-1 tss frost prepare --key-id the-key --count 1 

# out
- - secp256k1:02e5ebb4c36c0bb9f1a4a78fbe21575a356e3388704737a161d91d4deddaf1753f
  - secp256k1:03baed8a470e92e03830b9468ea0f604f755e55988bac28c03e406f9a1d9fe8c82


# in 
alice-2 keys export the-key | grep '^x'
# out
x: secp256k1:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e929

# in
alice-2 tss frost prepare --key-id the-key --count 1 

# out
- - secp256k1:032ccb846ed0a4460f0609465ce939f8195ab094dbe02718013702ba4ede9351bc
  - secp256k1:03ffe606d807650d25d99bc4849292d65a94dd04fdd2f7f8a51822b19b79122dd3

```

Using their respective SSSS key-shares, each participant creates a signature-shard. 
For the signature to be valid, all parties must agree on two things: the transcript and the set of commitments established in the previous step.
In addition, each participant should be aware of the SSSS X-values of all participants.

```yaml
# in
alice-1 tss frost sign --key-id the-key --hash-function sha3-256 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      800d1dea
signers:
    - - secp256k1:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee95f
      - secp256k1:02e5ebb4c36c0bb9f1a4a78fbe21575a356e3388704737a161d91d4deddaf1753f
      - secp256k1:03baed8a470e92e03830b9468ea0f604f755e55988bac28c03e406f9a1d9fe8c82
    - - secp256k1:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e929
      - secp256k1:032ccb846ed0a4460f0609465ce939f8195ab094dbe02718013702ba4ede9351bc
      - secp256k1:03ffe606d807650d25d99bc4849292d65a94dd04fdd2f7f8a51822b19b79122dd3
YAML

# out
y: secp256k1:03743b56682d5dbc0a7f4aff309a3d4635e2ab671bab878f910c194d56b89a2016
r: secp256k1:02fe6252553099ad890d22213d44a922b10d4e6c094aaf4a0c9276b9825ac3b307
z: secp256k1:a813c87cd529385b88088cb365d6381025880420ca1922fb0a9982cb9d8abc19
```

```yaml
# in
alice-2 tss frost sign --key-id the-key --hash-function sha3-256 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      800d1dea
signers:
    - - secp256k1:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee95f
      - secp256k1:02e5ebb4c36c0bb9f1a4a78fbe21575a356e3388704737a161d91d4deddaf1753f
      - secp256k1:03baed8a470e92e03830b9468ea0f604f755e55988bac28c03e406f9a1d9fe8c82
    - - secp256k1:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e929
      - secp256k1:032ccb846ed0a4460f0609465ce939f8195ab094dbe02718013702ba4ede9351bc
      - secp256k1:03ffe606d807650d25d99bc4849292d65a94dd04fdd2f7f8a51822b19b79122dd3
YAML

# out
y: secp256k1:02ce4ba085a39def301cc2cf893436a4e1fcc36054c944401e9999ec8c8413355d
r: secp256k1:020c3417efce15a463222dcdd6bf5b6d797189dcdcc77ecc064e497fea94ab92a9
z: secp256k1:e99ec4bcaa7a82cb1743de5cc8d5c62eca6909ad1417e8b972bdd0ba26e6a2b5
```

Finally, the signature is aggregated from the shards produced in the previous step.

```yaml
# in
alice-3 tss frost aggregate --curve secp256k1 --hash-function sha3-256 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      800d1dea
shards:
  secp256k1:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee95f:
    c:
      - secp256k1:02e5ebb4c36c0bb9f1a4a78fbe21575a356e3388704737a161d91d4deddaf1753f
      - secp256k1:03baed8a470e92e03830b9468ea0f604f755e55988bac28c03e406f9a1d9fe8c82
    y: secp256k1:03743b56682d5dbc0a7f4aff309a3d4635e2ab671bab878f910c194d56b89a2016
    r: secp256k1:02fe6252553099ad890d22213d44a922b10d4e6c094aaf4a0c9276b9825ac3b307
    z: secp256k1:a813c87cd529385b88088cb365d6381025880420ca1922fb0a9982cb9d8abc19
  secp256k1:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e929:
    c:
      - secp256k1:032ccb846ed0a4460f0609465ce939f8195ab094dbe02718013702ba4ede9351bc
      - secp256k1:03ffe606d807650d25d99bc4849292d65a94dd04fdd2f7f8a51822b19b79122dd3
    y: secp256k1:02ce4ba085a39def301cc2cf893436a4e1fcc36054c944401e9999ec8c8413355d
    r: secp256k1:020c3417efce15a463222dcdd6bf5b6d797189dcdcc77ecc064e497fea94ab92a9
    z: secp256k1:e99ec4bcaa7a82cb1743de5cc8d5c62eca6909ad1417e8b972bdd0ba26e6a2b5
YAML

# out
y: secp256k1:033016cf0bd874d48c1e35d00f5eda3d02cfa0bb4e4b66dc568c7e1cdd7f1c3271
r: secp256k1:029b4cf67866119af8e72240c80ce4315db9bf1bb3119c749ac7e0979145053c82
s: secp256k1:91b28d397fa3bb269f4c6b102eabfe40354230e72ee86b78bd84f4f8f43b1d8d
```

We can verify the produced signature:

```yaml
# in
alice verify schnorr --curve secp256k1 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      800d1dea
signature:
  y: secp256k1:033016cf0bd874d48c1e35d00f5eda3d02cfa0bb4e4b66dc568c7e1cdd7f1c3271
  r: secp256k1:029b4cf67866119af8e72240c80ce4315db9bf1bb3119c749ac7e0979145053c82
  s: secp256k1:91b28d397fa3bb269f4c6b102eabfe40354230e72ee86b78bd84f4f8f43b1d8d
YAML

# out
true
```

