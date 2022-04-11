
# :spades: :spades: :spades:  Play Crossword Puzzles on NEAR Protocol
<p align="center">
  <a href="https://near.org/">
    <img alt="Near" src="https://github.com/irene-bbox/sc-near-crossword/blob/master/public/near_white.png" width="250" />
  </a>
</p>

---

### Requirements  

node.js and npm (or yarn)

### Getting started 

Run the following command on your local environment:

```
git clone https://github.com/irene-bbox/sc-near-crossword.git <path_to_local_repository>        # clone repo
cd <path_to_local_repository>                                                                   # enter repo

cd contract                                                                                     # enter directory with Smart Contract code                 export PATH="$HOME/.cargo/bin:$PATH"                                                            # (optional) exports path to Cargo Rust
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
