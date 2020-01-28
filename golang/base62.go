package main

import "math"

const alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
const base = int64(len(alphabet))

func encodeInBase62(n int64) string {
	if n == 0 {
		return "0"
	}

	b := make([]byte, 0, 512)
	for n > 0 {
		r := math.Mod(float64(n), float64(base))
		n /= base
		b = append([]byte{alphabet[int(r)]}, b...)
	}
	return string(b)
}
