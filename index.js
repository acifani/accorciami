const { URL } = require('url');
const express = require('express');

const PORT = process.env.BASE_PORT || 8080;
const BASE_URL = `${process.env.BASE_URL || 'http://localhost'}:${PORT}`;

const app = express();
app.use(express.json());

const ALPHABET = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'.split(
  ''
);
const BASE = ALPHABET.length;
let counter = 0;

app.post('/accorcia', (req, res) => {
  const url = req.body.url;
  if (!url) {
    res.status(400).json({
      status_code: 400,
      error: 'Missing body parameter: url'
    });
  }

  const key = generateShortURL(url);
  const shortURL = new URL(key, BASE_URL);

  res.json({
    status_code: 200,
    short_url: shortURL.href
  });
});

app.get('/:id', (req, res) => {
  const shortURL = req.params.id;
  const longURL = getLongURL(shortURL);

  if (longURL) {
    res.redirect(longURL);
  } else {
    res.status(404).json({
      status_code: 404,
      error: 'URL not found'
    });
  }
});

app.listen(PORT, () => console.error(`Listening on port ${PORT}!`));

function generateShortURL(longURL) {
  const id = counter++;
  return encode(id);
}

function getLongURL(shortURL) {
  return keys[shortURL];
}

function encode(i) {
  if (i == 0) return ALPHABET[0];

  let s = '';
  while (i > 0) {
    s = s + ALPHABET[i % BASE];
    i = parseInt(i / BASE, 10);
  }

  return s;
}
