# sqlite-decancer

This project provides a SQLite extension that leverages the Rust `decancer` crate to sanitize and "cure" strings, removing or replacing potentially problematic Unicode characters.  The extension exposes a function that takes a string as input and returns a cleaned version, suitable for storage and comparison within your SQLite database.

## Purpose

The primary goal of this extension is to easily integrate the powerful string cleaning capabilities of the `decancer` library directly into SQLite.  This allows you to normalize text data within your database, addressing issues like:

* **Unicode normalization:**  Ensuring consistent representation of characters, avoiding issues with visually identical but byte-wise different strings.
* **Character removal/replacement:** Removing or replacing control characters, combining diacritical marks, and other potentially problematic or visually distracting characters.
* **Data consistency:**  Improving the reliability of string comparisons and searches within your database.

This extension simplifies data cleaning workflows by performing the sanitization directly within the database, eliminating the need to preprocess data before insertion or post-process it after retrieval.

## Features

* **Seamless SQLite Integration:**  Provides a user-defined function (UDF) that can be called directly within SQL queries.
* **Rust-Powered:**  Utilizes the `decancer` crate for robust and efficient string sanitization.
* **Simplified Usage:**  Easy to use with a single function call.
* **Focus on Cured String Output:**  The extension focuses on returning the cleaned string, ready for use in your database.

## Installation

Example (Illustrative - Adapt to your actual build process):

```bash
# Example build commands (replace with your actual build process)
cargo build --release

# Load the extension in your SQLite database
sqlite my_database.db
.load ./target/release/libdecancer_sqlite.so  # Path may vary

Usage
Once the extension is loaded, you can use the decancer_string() function in your SQL queries:
SELECT decancer_string('Héllo Wørld!'); -- Example input with problematic characters
-- Expected output: Hello World!

-- Example within a table update
UPDATE my_table SET name = decancer_string(name) WHERE id = 1;

-- Example in a SELECT statement with a WHERE clause
SELECT * FROM my_table WHERE decancer_string(name) = 'john doe';
```

API
The extension provides a single function:
 * decancer_string(text TEXT): Takes a string as input and returns the "cured" string after applying the decancer transformations.
Examples
