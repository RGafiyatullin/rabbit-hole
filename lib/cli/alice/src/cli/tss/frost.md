```yaml
# >>>>
alice keys get ri-2-of-3:0 | grep -e '^x' -e '^public'
######
public_key: ristretto25519:a6a1d84541f4fd2a45bc5c10a2cfca39b1c4079ae67d3d7b400c0134420c626a
x: ristretto25519:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee90f

# >>>>
alice tss frost prepare -k ri-2-of-3:0  -c 1
######
- - ristretto25519:76eaebf043e55faf510f18196eac6676498ae64d10022a3ff3842f460038fe14
  - ristretto25519:fe9a57a38bc96e2cd7909eb580556c2abcc9d068ddba23c25b569d33eb344411
```

```yaml
# >>>>
alice keys get ri-2-of-3:1 | grep -e '^x' -e '^public'
######
public_key: ristretto25519:a6a1d84541f4fd2a45bc5c10a2cfca39b1c4079ae67d3d7b400c0134420c626a
x: ristretto25519:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e909

# >>>>
alice tss frost prepare -k ri-2-of-3:1 -c 1
######
- - ristretto25519:ce83e69191428a3b6f99b843e1fc88286b6ffa8bd50120924e01780752415210
  - ristretto25519:0c9e45be759ea359095bff76dfb2b19f00dd64787a0f9d647a5f75d5589a4a73
```


```yaml
# >>>>
alice tss frost sign -k ri-2-of-3:0 -h sha3-256 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
signers:
    - - ristretto25519:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee90f
      - ristretto25519:76eaebf043e55faf510f18196eac6676498ae64d10022a3ff3842f460038fe14
      - ristretto25519:fe9a57a38bc96e2cd7909eb580556c2abcc9d068ddba23c25b569d33eb344411
    - - ristretto25519:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e909
      - ristretto25519:ce83e69191428a3b6f99b843e1fc88286b6ffa8bd50120924e01780752415210
      - ristretto25519:0c9e45be759ea359095bff76dfb2b19f00dd64787a0f9d647a5f75d5589a4a73
YAML
######
y: ristretto25519:fe6440fe06355f5c835faae4689487e7efe50d05984cee603c4757c0c92d080f
r: ristretto25519:c0404c50461cc3c38b6cd4b9d535f705ffdab52154f39fa6288a3261e702b737
z: ristretto25519:4bc09a4c77f211a603f7a2921de8811a16fa2265951e500f7d1cf37711a67e04
```

```yaml
# >>>>
alice tss frost sign -k ri-2-of-3:1 -h sha3-256 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
signers:
    - - ristretto25519:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee90f
      - ristretto25519:76eaebf043e55faf510f18196eac6676498ae64d10022a3ff3842f460038fe14
      - ristretto25519:fe9a57a38bc96e2cd7909eb580556c2abcc9d068ddba23c25b569d33eb344411
    - - ristretto25519:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e909
      - ristretto25519:ce83e69191428a3b6f99b843e1fc88286b6ffa8bd50120924e01780752415210
      - ristretto25519:0c9e45be759ea359095bff76dfb2b19f00dd64787a0f9d647a5f75d5589a4a73
YAML
######
y: ristretto25519:5ed8d0a6adbc1249ffa9e43bed7dacebe67931db7c3750a95553d55e8b16b67b
r: ristretto25519:b242b91c6a5baa5a0ff61762d190a0d5c84de3265dc088b9ae13b5cb873d4e56
z: ristretto25519:1f010c5ba27550f680c0c2f82055cf6df141ffe73e5e37d1eb4ab5d4265ab30a
```

```yaml
# >>>>
alice tss frost aggregate -c ristretto25519 -h sha3-256 <<YAML
transcript:
    hash_function: sha3-256
    input:
        - !point    Y
        - !point    R
        - !text     Hello There!
        - !hex      48656c6c6f20546865726521
shards:
    ristretto25519:48dda5bbe9171a6656206ec56c595c5834b6cf38c5fe71bcb44fe43833aee90f:
        c: 
           - ristretto25519:76eaebf043e55faf510f18196eac6676498ae64d10022a3ff3842f460038fe14
           - ristretto25519:fe9a57a38bc96e2cd7909eb580556c2abcc9d068ddba23c25b569d33eb344411
        y: ristretto25519:fe6440fe06355f5c835faae4689487e7efe50d05984cee603c4757c0c92d080f
        r: ristretto25519:c0404c50461cc3c38b6cd4b9d535f705ffdab52154f39fa6288a3261e702b737
        z: ristretto25519:4bc09a4c77f211a603f7a2921de8811a16fa2265951e500f7d1cf37711a67e04
    ristretto25519:b875632ccf606eef2397124e6c2febf24e91a89b43c6bf762c8e9ea61a48e909:
        c: 
           - ristretto25519:ce83e69191428a3b6f99b843e1fc88286b6ffa8bd50120924e01780752415210
           - ristretto25519:0c9e45be759ea359095bff76dfb2b19f00dd64787a0f9d647a5f75d5589a4a73
        y: ristretto25519:5ed8d0a6adbc1249ffa9e43bed7dacebe67931db7c3750a95553d55e8b16b67b
        r: ristretto25519:b242b91c6a5baa5a0ff61762d190a0d5c84de3265dc088b9ae13b5cb873d4e56
        z: ristretto25519:1f010c5ba27550f680c0c2f82055cf6df141ffe73e5e37d1eb4ab5d4265ab30a
YAML
######
y: ristretto25519:a6a1d84541f4fd2a45bc5c10a2cfca39b1c4079ae67d3d7b400c0134420c626a
r: ristretto25519:6c8896ea71e7b11de45355e7148b4655efbf8b02bd0bb614e32f1082dd564445
s: ristretto25519:6ac1a6a71968629c84b7658b3e3d5188073c224dd47c87e06867a84c3800320f
```

