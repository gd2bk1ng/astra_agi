#!/bin/bash

AUTHOR="Alex Roussinov"
PROJECT="Astra AGI"
LICENSE_TEXT="//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details."

DATE=$(date +%Y-%m-%d)

find src examples tests -type f -name "*.rs" | while read -r file; do
    # Check if file already has header (look for "Astra" in first 10 lines)
    if head -n 10 "$file" | grep -q "Astra"; then
        echo "Header already present in $file, skipping."
        continue
    fi

    BASENAME=$(basename "$file")

    # Create temporary header file
    HEADER=$(cat <<EOF
// =============================================================================
//  $PROJECT
//  File: $BASENAME
//
//  Description: [Add description here]
//
//  Author:      $AUTHOR
//  Created:     $DATE
//  Updated:     $DATE
//
//  $LICENSE_TEXT
// =============================================================================

EOF
)

    # Prepend header to file
    # Use a temp file to avoid overwriting during read
    TMPFILE=$(mktemp)
    echo "$HEADER" > "$TMPFILE"
    cat "$file" >> "$TMPFILE"
    mv "$TMPFILE" "$file"

    echo "Header added to $file"
done
