// @ts-check
const { URL } = require('url');
const express = require('express');
const morgan = require('morgan');
const { Tedis } = require('tedis');

const PORT = process.env.BASE_PORT || 8080;
const BASE_URL = process.env.BASE_URL || `http://localhost:${PORT}`;
const ALPHABET = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'.split(
  ''
);
const BASE = ALPHABET.length;

const tedis = new Tedis({
  host: process.env.REDIS_HOST,
  port: parseInt(process.env.REDIS_PORT || '6379'),
  password: process.env.REDIS_PASSWORD
});
const app = express();
app.use(express.json());
app.use(morgan('combined'));

app.post('/accorcia', async (req, res) => {
  const url = req.body.url;
  if (!url) {
    res.status(400).json({
      status_code: 400,
      error: 'Missing body parameter: url'
    });
  }

  const key = await generateShortURL(url);
  const shortURL = new URL(key, BASE_URL);

  res.json({
    status_code: 200,
    short_url: shortURL.href
  });
});

app.get('/:id', async (req, res) => {
  const shortURL = req.params.id;
  const longURL = await getLongURL(shortURL);

  if (longURL) {
    await tedis.hincrby(shortURL, 'visits', 1);
    res.redirect(longURL);
  } else {
    res.status(404).json({
      status_code: 404,
      error: 'URL not found'
    });
  }
});

app.listen(PORT, () => console.error(`Listening on port ${PORT}!`));

async function generateShortURL(longURL) {
  const id = await tedis.incr('counter');
  const shortURL = encode(id);
  await tedis.hmset(shortURL, {
    long_url: longURL,
    visits: 0
  });

  return shortURL;
}

async function getLongURL(shortURL) {
  return tedis.hget(shortURL, 'long_url');
}

function encode(i) {
  if (i == 0) return ALPHABET[0];

  let s = '';
  while (i > 0) {
    s = s + ALPHABET[i % BASE];
    i = i / BASE;
  }

  return s;
}
