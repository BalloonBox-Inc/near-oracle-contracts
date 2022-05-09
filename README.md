
# :fire: NEAR Oracle Contract 
<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="https://github.com/BalloonBox-Inc/NEARoracle-Contract/blob/main/images/near_white.png" width="700" />
  </a>
</p>

---

### Requirements  

node.js, npm (or yarn), Rust, and Wasm toolchain

##### Install Rust and Wasm toolchain

To [install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) on Linux or MacOS, use the following command:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Then, add the `wasm32-unknown-unknown` toolchain. This toolchain is required because the Rust contract that we will build will be compiled to [Wasm](https://webassembly.org/) (Web Assembly) to run on the NEAR blockchain.

```bash
rustup target add wasm32-unknown-unknown
```
