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

_(Provide specific instructions for installation.  This will likely involve compiling the extension and loading it into your SQLite database.  Include details about dependencies and build steps.)_

Example (Illustrative - Adapt to your actual build process):

```bash
# Example build commands (replace with your actual build process)
cargo build --release

# Load the extension in your SQLite database
sqlite my_database.db
.load ./target/release/libsqlite_decancer.so  # Path may vary

Usage
Once the extension is loaded, you can use the decancer() function in your SQL queries:
SELECT decancer('Héllo Wørld!'); -- Example input with problematic characters
-- Expected output: Hello World!

-- Example within a table update
UPDATE my_table SET name = decancer(name) WHERE id = 1;

-- Example in a SELECT statement with a WHERE clause
SELECT * FROM my_table WHERE decancer(name) = 'john doe';

API
The extension provides a single function:
 * decancer(text TEXT): Takes a string as input and returns the "cured" string after applying the decancer transformations.
Examples
(Provide more detailed examples of how to use the extension in various scenarios.)
Contributing
(If you welcome contributions, explain the process for contributing to your project.)
License
(Specify the license under which your project is distributed.)

