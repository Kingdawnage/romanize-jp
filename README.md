# Romanize-JP

A server that romanizes Japanese text in API responses.

## Description

Romanize-JP is a lightweight server that provides an API to convert Japanese text (hiragana, katakana, and kanji) into romaji (Latin alphabet). This makes Japanese text more accessible to users who can't read Japanese scripts.

## Installation

### Prerequisites

- Node.js (v12+)
- npm or yarn

### Setup

```bash
# Clone repository
git clone https://github.com/yourusername/romanize-jp.git
cd romanize-jp

# Install dependencies
npm install

# Start server
npm start
```

## Usage

### API Endpoints

#### `POST /romanize`

**Request:**
```json
{
    "text": "こんにちは世界"
}
```

**Response:**
```json
{
    "original_text": "こんにちは世界",
    "romanized_text": "konnichiwa sekai"
}
```

### Examples

**cURL:**
```bash
curl -X POST http://localhost:8080/romanize \
    -H "Content-Type: application/json" \
    -d '{"text": "こんにちは世界"}'
```

**JavaScript:**
```javascript
fetch('http://localhost:8080/romanize', {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({text: 'こんにちは世界'})
})
.then(response => response.json())
.then(data => console.log(data));
```

## License

This project is licensed under the MIT License.