FROM golang:1-alpine

WORKDIR /go/src/accorciami
COPY . .

RUN go get -d -v ./...
RUN go build -v ./...

CMD ["./accorciami"]
