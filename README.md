# nostr-keyconv

nostr の key (npub, nsec, note, hex) を NIP-19 に従って相互変換するコマンド。

```text
npub <-> hex
nsec <-> hex
note <-> hex
```

## Installation

```console
$ # `cargo`
$ git clone https://github.com/bouzuya/nostr-keyconv
$ cd nostr-keyconv/
$ cargo install --path .
```

## Usage

```console
$ nostr-keyconv npub10elfcs4fr0l0r8af98jlmgdh9c8tcxjvz9qkw038js35mp4dma8qzvjptg
7e7e9c42a91bfef19fa929e5fda1b72e0ebc1a4c1141673e2794234d86addf4e

$ nostr-keyconv nsec1vl029mgpspedva04g90vltkh6fvh240zqtv9k0t9af8935ke9laqsnlfe5
67dea2ed018072d675f5415ecfaed7d2597555e202d85b3d65ea4e58d2d92ffa

$ nostr-keyconv note1paq6fdap00vkkxch74hxkkhjldvjtkwa6u23as2cpc92h5ghxnmqf7eyg4
0f41a4b7a17bd96b1b17f56e6b5af2fb5925d9ddd7151ec1580e0aabd11734f6

$ nostr-keyconv --prefix=npub 3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d
npub180cvv07tjdrrgpa0j7j7tmnyl2yr6yr7l8j4s3evf6u64th6gkwsyjh6w6

$ nostr-keyconv --prefix=nsec 67dea2ed018072d675f5415ecfaed7d2597555e202d85b3d65ea4e58d2d92ffa
nsec1vl029mgpspedva04g90vltkh6fvh240zqtv9k0t9af8935ke9laqsnlfe5

$ nostr-keyconv --prefix=note 0f41a4b7a17bd96b1b17f56e6b5af2fb5925d9ddd7151ec1580e0aabd11734f6
note1paq6fdap00vkkxch74hxkkhjldvjtkwa6u23as2cpc92h5ghxnmqf7eyg4
```
