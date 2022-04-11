
# :spades: :spades: :spades:  Play Crossword Puzzles on NEAR Protocol
<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="https://github.com/irene-bbox/sc-near-crossword/blob/master/public/near_white.png" width="250" />
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

### Getting started 

Run the following command on your local environment:

```bash
git clone https://github.com/irene-bbox/sc-near-crossword.git <path_to_local_repository>        # clone repo
cd <path_to_local_repository>                                                                   # enter repo

cd contract                                                                                     # enter directory with Smart Contract code                 export PATH="$HOME/.cargo/bin:$PATH"                                                            # (optional) add the Cargo Rust dir to path
./build.sh                                                                                      # compile the Smart Contract

cd ..                                                                                           # exit Smart Contract directory
env CONTRACT_NAME=crossword.zion.testnet npm run start                                          # launch React dApp
```

Tha last command will runs the app in the development mode.\
Open [http://localhost:1234](http://localhost:1234) to view it in the browser.

### Log into NEAR 

Create a [NEAR wallet](https://wallet.testnet.near.org/) on testnet.\
Log into your NEAR wallet from the browser where you launched the crossword puzzle. :satellite: 

![NEAR Login](https://github.com/irene-bbox/sc-near-crossword/blob/master/public/login.png)


### Solve the puzzle  :ninja:

You're now ready to play!\
Go ahead and solve the crossword puzzle!\
Your solution will be saved to the NEAR blockchain and will be visible in the NEAR Blockchain [Explorer](https://explorer.testnet.near.org/).
