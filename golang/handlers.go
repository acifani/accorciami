package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"

	"github.com/gorilla/mux"
)

func accorciaHandler(store *store) func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		var err error
		var url string
		if postValue := r.PostFormValue("url"); postValue != "" {
			url = postValue
		} else if r.Header.Get("Content-Type") == "application/json" {
			var req accorciaRequest
			err = json.NewDecoder(r.Body).Decode(&req)
			if err == nil {
				url = req.URL
			}
		}

		if err != nil || url == "" {
			answerWithError(w, http.StatusBadRequest, "Missing parameter: url")
			return
		}

		id, err := store.getNextID()
		if err != nil {
			answerWithError(w, http.StatusInternalServerError, "We're having issues")
			return
		}

		shortURL := encodeInBase62(id)
		store.createNewURL(shortURL, url)

		responseURL := fmt.Sprintf("%s/%s", os.Getenv("BASE_URL"), shortURL)
		w.Header().Set("Content-Type", "application/json")
		response := accorciaSuccessResponse{
			StatusCode: http.StatusOK,
			ShortURL:   responseURL,
		}
		json.NewEncoder(w).Encode(response)
	}
}

type accorciaRequest struct {
	URL string `json:"url"`
}

func visitFileHandler(store *store) func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		vars := mux.Vars(r)
		shortURL := vars["id"]
		longURL, _ := store.getLongURL(shortURL)

		if longURL == "" {
			answerWithError(w, http.StatusNotFound, "URL not found")
			return
		}

		store.incrementVisitCounter(longURL)
		http.Redirect(w, r, longURL, http.StatusTemporaryRedirect)
	}
}

func answerWithError(w http.ResponseWriter, statusCode int, errorMessage string) {
	w.WriteHeader(statusCode)
	w.Header().Set("Content-Type", "application/json")
	response := genericErrorResponse{
		StatusCode: statusCode,
		Error:      errorMessage,
	}
	json.NewEncoder(w).Encode(response)
}

type genericErrorResponse struct {
	StatusCode int    `json:"status_code"`
	Error      string `json:"error"`
}

type accorciaSuccessResponse struct {
	StatusCode int    `json:"status_code"`
	ShortURL   string `json:"short_url"`
}
