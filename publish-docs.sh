#!/bin/bash

openssl aes-256-cbc -K $encrypted_c722399e1251_key -iv $encrypted_c722399e1251_iv -in publish-key.enc -out ~/.ssh/publish-key -d
chmod u=rw,og= ~/.ssh/publish-key
echo "Host github.com" >> ~/.ssh/config
echo "  IdentityFile ~/.ssh/publish-key" >> ~/.ssh/config
git remote set-url origin git@github.com:dimbleby/ip-rs
git config user.name "Travis CI"
echo "<meta http-equiv=refresh content=0;url=ip/index.html>" > target/doc/index.html
~/.local/bin/ghp-import -n target/doc
git push origin +gh-pages
