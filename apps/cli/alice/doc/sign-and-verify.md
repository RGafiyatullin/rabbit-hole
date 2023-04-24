```yaml
alice keys add k-ri-1 <<YAML
!full_key
curve: ristretto25519
value: ristretto25519:6ac1a6a71968629c84b7658b3e3d5188073c224dd47c87e06867a84c3800320f
YAML
```

```yaml
# >>>>
alice sign schnorr --key-id k-ri-1 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
YAML
######
y: ristretto25519:3045c2add77b6fed1594d824e37c19d4526fb39fd9d575a89028e256af5bf501
r: ristretto25519:664444fafb348cf9d801ca6bd818649e1f1382c075ad18ec5e1782e8ddddf85a
s: ristretto25519:a11b8960251b738d758dae7c010881f422db2fe3da140b6bab6d14ace43b780b
```

```yaml
# >>>>
alice verify schnorr --curve ristretto25519 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
signature:
  y: ristretto25519:3045c2add77b6fed1594d824e37c19d4526fb39fd9d575a89028e256af5bf501
  r: ristretto25519:664444fafb348cf9d801ca6bd818649e1f1382c075ad18ec5e1782e8ddddf85a
  s: ristretto25519:a11b8960251b738d758dae7c010881f422db2fe3da140b6bab6d14ace43b780b
YAML
######
true

# >>>>
alice verify schnorr --curve ristretto25519 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Somthing, that wasn't signed
        - !hex      9900aa
signature:
  y: ristretto25519:3045c2add77b6fed1594d824e37c19d4526fb39fd9d575a89028e256af5bf501
  r: ristretto25519:664444fafb348cf9d801ca6bd818649e1f1382c075ad18ec5e1782e8ddddf85a
  s: ristretto25519:a11b8960251b738d758dae7c010881f422db2fe3da140b6bab6d14ace43b780b
YAML
######
false
```

