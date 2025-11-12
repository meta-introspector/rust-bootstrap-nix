#!/bin/bash

# Log all arguments to a file
echo "$(date): rustc called with arguments: $*" >> rustc_calls.log

# Execute the actual rustc command
exec rustc "$@"
