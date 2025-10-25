#!/bin/bash

PRELUDE_LINE="use crate::prelude::*";
PRELUDE_INSERT="${PRELUDE_LINE}\n\n";

# Get a list of Rust files that do not contain the prelude line
FILES_TO_MODIFY=$(grep -r -L "${PRELUDE_LINE}" --include "*.rs" .)

# Iterate through each file and prepend the prelude line
for file in $FILES_TO_MODIFY; do
    echo "Adding prelude to $file"
    # Use sed to insert the prelude line at the beginning of the file
    # The '1i' command inserts text before the first line
    sed -i "1i${PRELUDE_INSERT}" "$file"
done

echo "Prelude insertion complete."

