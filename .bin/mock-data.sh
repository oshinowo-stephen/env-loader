#!/bin/bash

# making the mock data for the tests to properly excute
sudo mkdir /run/secrets
sudo touch /run/secrets/akko



sudo printf "Little Witch Academia - Akko" | sudo tee /run/secrets/akko >> /dev/null
