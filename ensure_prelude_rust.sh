#!/usr/bin/env bash

PRELUDE_LINE="use crate::prelude::*";

# Find all Rust files that contain the prelude line but are missing a semicolon
FILES_TO_FIX=$(grep -r -l "${PRELUDE_LINE}" --include "*.rs" . | xargs grep -L "${PRELUDE_LINE};" --include "*.rs")

# Iterate through each file and add a semicolon to the prelude line
for file in $FILES_TO_FIX; do
    echo "Adding semicolon to prelude in $file"
    # Use sed to append a semicolon to lines ending with "use crate::prelude::*"
    sed -i '/use crate::prelude::\*$/s/$/;/' "$file"
done

echo "Prelude semicolon fix complete."
