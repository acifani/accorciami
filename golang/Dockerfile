FROM golang:1-alpine

RUN apk add --no-cache git

WORKDIR /go/src/accorciami
COPY . .

RUN go get -d -v ./...
RUN go build -v -o accorciami ./...

CMD ["./accorciami"]
