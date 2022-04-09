#!/bin/bash

./build.sh

export NEAR_ACCT=xword.demo.testnet
export PARENT_ACCT=demo.testnet
near delete $NEAR_ACCT $PARENT_ACCT
near create-account $NEAR_ACCT --masterAccount $PARENT_ACCT
near deploy $NEAR_ACCT --wasmFile res/crossword_tutorial_chapter_2.wasm --initFunction new --initArgs '{"owner_id": "'$NEAR_ACCT'"}'

# We're able to determine the public key by visiting a site like:
# https://www.wolframalpha.com/input/?i=sha256+%22paras+rainbowbridge+mintbase+yoctonear+cli%22

near call $NEAR_ACCT new_puzzle 
'{"solution_hash":"cbb4c3096c4bdf8ed58ea4c27edcbc38264bb7af4fa893e0ac2e5b13ff430923","answers":[{"num": 1,"start": {"x": 1,"y": 2},"direction": "Across","length": 10,"clue": "The company I work for is called ______ Technology Inc."},{"num": 2,"start": {"x": 5,"y": 1},"direction": "Down","length": 8,"clue": "One of our team favourite board games is ______ deal"},{"num": 3,"start": {"x": 4,"y": 4},"direction": "Across","length": 7,"clue": "Name of the most dramatic person in our team - hint=David"},{"num": 4,"start": {"x": 6,"y": 1},"direction": "Down","length": 4,"clue": "Name of the best web designer on earth"},{"num": 5,"start": {"x": 0,"y": 6},"direction": "Across","length": 8,"clue": "It is Irene fave Mandalorian character - one word, no hyphen"}, {"num": 6,"start": {"x": 10,"y": 6},"direction": "Down","length": 6,"clue": "The Bbox office is located by ______ Place"}, {"num": 7,"start": {"x": 4,"y": 8},"direction": "Across","length": 6,"clue": "What is the programming language primarily used by the data scientists in our team?"}]}'
 --accountId $NEAR_ACCT
