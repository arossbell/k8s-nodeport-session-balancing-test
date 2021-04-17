FROM golang:alpine

WORKDIR /src/

COPY server/* /src/

RUN go build -o /bin/server .

EXPOSE 5501

CMD /bin/server
