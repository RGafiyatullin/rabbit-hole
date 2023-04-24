```yaml
# >>>>
alice keys get ri-2-of-3:0 | grep -e '^x'
######
x: ristretto25519:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee90f

# >>>>
alice tss frost -k ri-2-of-3:0 prepare -c 1
######
- - ristretto25519:6a69ae325184a00f600c73f6d18f6fb3bac5ded2b9758afc45d3bc7076a7841f
  - ristretto25519:a85c6062a42f665effdaf854b63c84871b8ce88cdd95a66a3a3880b1cf945114
```

```yaml
# >>>>
alice keys get ri-2-of-3:1 | grep -e '^x'
######
x: ristretto25519:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e909

# >>>>
alice tss frost -k ri-2-of-3:1 prepare -c 1
######
- - ristretto25519:fe3a1735708e5a73a3e5efbe44d3e1aba72956e0f141321c34c0459cfa693c36
  - ristretto25519:202b93c1f1e0e6b58f946e9c6a8f83b0553162e089baa7cf6a22b20497450d25
```


```yaml
# >>>>
alice tss frost -k ri-2-of-3:0 sign -h sha3-256 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
signers:
    - - ristretto25519:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee90f
      - ristretto25519:6a69ae325184a00f600c73f6d18f6fb3bac5ded2b9758afc45d3bc7076a7841f
      - ristretto25519:a85c6062a42f665effdaf854b63c84871b8ce88cdd95a66a3a3880b1cf945114
    - - ristretto25519:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e909
      - ristretto25519:fe3a1735708e5a73a3e5efbe44d3e1aba72956e0f141321c34c0459cfa693c36
      - ristretto25519:202b93c1f1e0e6b58f946e9c6a8f83b0553162e089baa7cf6a22b20497450d25
YAML
######
y: ristretto25519:fe6440fe06355f5c835faae4689487e7efe50d05984cee603c4757c0c92d080f
r: ristretto25519:18b73525b1b9c6bae13680e39222dac20f04ba5ae02c6cac077f8ee861b2804c
z: ristretto25519:a2f081e6f8b6b1354e7b6090356ec7f48b6a6346ad8648af6a36fafdcdb9d607
```

```yaml
# >>>>
alice tss frost -k ri-2-of-3:1 sign -h sha3-256 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
signers:
    - - ristretto25519:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee90f
      - ristretto25519:6a69ae325184a00f600c73f6d18f6fb3bac5ded2b9758afc45d3bc7076a7841f
      - ristretto25519:a85c6062a42f665effdaf854b63c84871b8ce88cdd95a66a3a3880b1cf945114
    - - ristretto25519:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e909
      - ristretto25519:fe3a1735708e5a73a3e5efbe44d3e1aba72956e0f141321c34c0459cfa693c36
      - ristretto25519:202b93c1f1e0e6b58f946e9c6a8f83b0553162e089baa7cf6a22b20497450d25
YAML
######
y: ristretto25519:5ed8d0a6adbc1249ffa9e43bed7dacebe67931db7c3750a95553d55e8b16b67b
r: ristretto25519:7096ce74b3e4e179dc69a68fb4e5f72ba372e6a324c4c605f97d290703d9ee5f
z: ristretto25519:1d761198888b3f5364dcd5e530048a559ec2f2d208e45f51ffa7866eac760602
```