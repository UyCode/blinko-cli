# Blink CLI

A command-line interface tool for managing notes with authentication support.

## Features

- Authentication with username and password
- Create, update, and delete notes
- Session token management
- Secure cookie-based authentication

## Installation

### From Source

```bash
git clone https://github.com/yourusername/blink-cli
cd blink-cli
cargo build --release
```

The executable will be available at `target/release/blink-cli`

## Usage

### Authentication

```bash
# Login with credentials (defaults to admin/123456 if not provided)
blink-cli -o login
blink-cli -o login -u myusername -p mypassword

# Set token manually (if needed)
blink-cli -o set-token -t "your-token-here"
```

### Managing Notes

```bash
# Create a new note
echo "This is a new note" | blink-cli

# Create a note with explicit create operation
echo "Another note" | blink-cli -o create

# Update an existing note
echo "Updated content" | blink-cli -o update -i note_id

# Delete a note
blink-cli -o delete -i note_id
```

### Examples

```bash
# Login first
blink-cli -o login -u admin -p 123456

# Create a note
echo "My first note via CLI" | blink-cli

# Update the note
echo "Updated content" | blink-cli -o update -i 123

# Delete the note
blink-cli -o delete -i 123
```

## Configuration

The CLI stores configuration (including authentication tokens) in:
- Linux: `~/.config/blink-cli/config.json`
- macOS: `~/Library/Application Support/com.blinko.blink-cli/config.json`
- Windows: `%APPDATA%\blinko\blink-cli\config\config.json`

## Error Handling

- If not authenticated, use the login command first
- For update/delete operations, note ID is required
- Check error messages for detailed information about failures

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License

Copyright (c) 2024 [Your Name]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.