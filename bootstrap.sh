#!/bin/bash

readonly DIR=res

if [ ! -d "$DIR" ]
then
  echo "Creating dir $DIR"
  mkdir $DIR
fi
cd $DIR

echo "[*] Downloading Shakespeare"
wget https://www.gutenberg.org/files/100/100-0.txt --output-document shakespeare.txt

echo "[*] Downloading Wikipedia Index"
wget --output-document  wiki-index.txt.bz2 https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-pages-articles-multistream-index.txt.bz2 
bzip2 -d  wiki-index.txt.bz2 
