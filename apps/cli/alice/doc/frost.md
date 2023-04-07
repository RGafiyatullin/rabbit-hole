
# Frost

## Pregenerate Nonce-Pairs for each key-share

```shell
target/release/alice tss frost nonce -k k0:1 generate --count 1
```

```yaml
- cd: 024ff63b310f8566e240653eb9c0150823f8d5f800bb1c76f206813c172955868a
  ce: 03bb1332fa3fa89d95edc74ae4a0990e63f84831016249bd0aaed653a9c98f2d8b
```

--- 

```shell
target/release/alice tss frost nonce -k k0:2  generate --count 1
```

```yaml
- cd: 024e21deeb55761f78a08a3062e8551289092c8694132cf9167d565ecf39e39ced
  ce: 022a5b0d388314d694baf2bbcdff76548c50182bd94688b75221b4648d2f08c071
```

## Signing

```yaml
target/release/alice tss frost sign -k k0:1 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
shamir_xs:
    - 01
    - 02
commitments:
    - cd: 024ff63b310f8566e240653eb9c0150823f8d5f800bb1c76f206813c172955868a
      ce: 03bb1332fa3fa89d95edc74ae4a0990e63f84831016249bd0aaed653a9c98f2d8b
    - cd: 024e21deeb55761f78a08a3062e8551289092c8694132cf9167d565ecf39e39ced
      ce: 022a5b0d388314d694baf2bbcdff76548c50182bd94688b75221b4648d2f08c071
YAML
```

```yaml
sign:
  r_i: 0344b07ea83ff47fb45f60630c8ce7d3e062d6d4cd2da2e904b67d64116cf220e6
  y_i: 0211c961e9e8ed8442e43c0e1a9e83706871cf28cd4ee6a14943462bbe7df80db0
  z_i: 1f53e7ce25da7790be66e1a8521e77c2fde924270112b7740edc23eb1bf60ca3
```

---

```yaml
target/release/alice tss frost sign -k k0:2 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
shamir_xs:
    - 01
    - 02
commitments:
    - cd: 024ff63b310f8566e240653eb9c0150823f8d5f800bb1c76f206813c172955868a
      ce: 03bb1332fa3fa89d95edc74ae4a0990e63f84831016249bd0aaed653a9c98f2d8b
    - cd: 024e21deeb55761f78a08a3062e8551289092c8694132cf9167d565ecf39e39ced
      ce: 022a5b0d388314d694baf2bbcdff76548c50182bd94688b75221b4648d2f08c071
YAML
```

```yaml
sign:
  r_i: 02158b69fb3c2471cb70dfbee9748afae65fccc723c2214305484c9aaf527ee5ae
  y_i: 021e5ddb262b0cc83730ab650141aed8608072a1d7711099ae866f12747cf8f1ca
  z_i: 8fb687f94ae41bbfa287ecd6ea36d1073c18ed5194686758b75ce264b17a405e
```

## Aggregate

```yaml
target/release/alice tss frost aggregate <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
shamir_xs:
    - 01
    - 02
commitments:
    - cd: 024ff63b310f8566e240653eb9c0150823f8d5f800bb1c76f206813c172955868a
      ce: 03bb1332fa3fa89d95edc74ae4a0990e63f84831016249bd0aaed653a9c98f2d8b
    - cd: 024e21deeb55761f78a08a3062e8551289092c8694132cf9167d565ecf39e39ced
      ce: 022a5b0d388314d694baf2bbcdff76548c50182bd94688b75221b4648d2f08c071
shards:
    - r_i: 0344b07ea83ff47fb45f60630c8ce7d3e062d6d4cd2da2e904b67d64116cf220e6
      y_i: 0211c961e9e8ed8442e43c0e1a9e83706871cf28cd4ee6a14943462bbe7df80db0
      z_i: 1f53e7ce25da7790be66e1a8521e77c2fde924270112b7740edc23eb1bf60ca3
    - r_i: 02158b69fb3c2471cb70dfbee9748afae65fccc723c2214305484c9aaf527ee5ae
      y_i: 021e5ddb262b0cc83730ab650141aed8608072a1d7711099ae866f12747cf8f1ca
      z_i: 8fb687f94ae41bbfa287ecd6ea36d1073c18ed5194686758b75ce264b17a405e
YAML
```


```yaml
signature:
  r: 02484ee5475e0139182eb7a8a334111899f2339b4b352b7e8caa23428d8cc64349
  y: 032189e442f10e6ac256d4e7560b285624e85dfefdd40a308608d4a78b1debb06a
  z: af0a6fc770be935060eece7f3c5548ca3a021178957b1eccc639064fcd704d01
```


