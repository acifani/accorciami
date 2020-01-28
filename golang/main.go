package main

import (
	"log"
	"net/http"

	"github.com/gorilla/mux"
)

func main() {
	redisClient, err := makeRedisClient()
	if err != nil {
		log.Fatal(err)
	}

	store := &store{
		client: redisClient,
	}

	r := mux.NewRouter()
	r.HandleFunc("/accorcia", accorciaHandler(store)).Methods("POST")
	r.HandleFunc("/{id}", visitFileHandler(store)).Methods("GET")
	r.Handle("/", http.FileServer(http.Dir("static")))
	http.Handle("/", r)
	log.Fatal(http.ListenAndServe(":8080", nil))
}
