#!/bin/bash

# TODO Update to tail the production log file

clear
tail -f test-ledger/validator.log | grep cronos
