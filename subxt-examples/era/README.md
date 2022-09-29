 [subxt](https://github.com/paritytech/subxt) is a library to **sub**mit e**xt**rinsics to a substrate node via RPC.


### Downloading metadata from a Substrate node

Use the [`subxt-cli`](./cli) tool to download the metadata for your target runtime from a node.

1. Install:
```bash
cargo install subxt-cli
```
2. Save the encoded metadata to a file:
```bash
subxt metadata -f bytes > metadata.scale
```
This defaults to querying the metadata of a locally running node on the default `http://localhost:9933/`.

3. If you are using remote node, fetch metadata with `url` argument
```bash
subxt metadata --url [WSS_URL] -f bytes > metadata.scale
```
find available `wss` ednpoints on Astar docs site. Example can be:
```bash
subxt metadata --url "wss://shiden.public.blastapi.io:443" -f bytes > shiden-metadata.scale
```
