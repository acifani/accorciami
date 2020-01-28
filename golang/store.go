package main

import (
	"fmt"
	"log"
	"os"

	"github.com/mediocregopher/radix/v3"
)

type store struct {
	client *radix.Pool
}

func (s *store) getLongURL(key string) (string, error) {
	var longURL string
	if err := s.client.Do(radix.Cmd(&longURL, "HGET", key, "long_url")); err != nil {
		log.Printf("[store::getLongURL] %v\n", err)
		return "", err
	}
	return longURL, nil
}

func (s *store) incrementVisitCounter(key string) error {
	if err := s.client.Do(radix.Cmd(nil, "HINCRBY", key, "visits", "1")); err != nil {
		log.Printf("[store::incrementVisitCounter] %v\n", err)
		return err
	}
	return nil
}

func (s *store) getNextID() (int64, error) {
	var nextID int64
	if err := s.client.Do(radix.Cmd(&nextID, "INCR", "counter")); err != nil {
		log.Printf("[store::getNextID] %v\n", err)
		return 0, err
	}
	return nextID, nil
}

func (s *store) createNewURL(shortURL, longURL string) error {
	if err := s.client.Do(radix.Cmd(nil, "HMSET", shortURL, "long_url", longURL, "visits", "0")); err != nil {
		log.Printf("[store::createNewURL] %v\n", err)
		return err
	}
	return nil
}

func makeRedisClient() (*radix.Pool, error) {
	host := os.Getenv("REDIS_HOST")
	port := os.Getenv("REDIS_PORT")
	password := os.Getenv("REDIS_PASSWORD")
	uri := fmt.Sprintf("%s:%s", host, port)

	connFunc := func(network, addr string) (radix.Conn, error) {
		return radix.Dial(network, addr, radix.DialAuthPass(password))
	}

	return radix.NewPool("tcp", uri, 10, radix.PoolConnFunc(connFunc))
}
