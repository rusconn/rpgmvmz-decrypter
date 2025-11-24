# rpgmvmz-decrypter

A command-line tool to decrypt encrypted assets in RPG Maker MV/MZ games.

## Install

```shell
cargo install --git https://github.com/rusconn/rpgmvmz-decrypter.git
```

## Usage

```shell
decvz <game_dir>
```

## Decryption Rules

| Encrypted File | Decrypted As |
| -------------- | ------------ |
| .rpgmvo        | .ogg         |
| .rpgmvm        | .m4a         |
| .rpgmvp        | .png         |
| .ogg\_         | .ogg         |
| .m4a\_         | .m4a         |
| .png\_         | .png         |
