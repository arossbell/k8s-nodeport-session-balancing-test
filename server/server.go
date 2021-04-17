// Single-Threaded Server for Kubernetes Service test.
//  Receives an integer over TCP; it sleeps for that long.
// From https://github.com/arossbell/k8s-nodeport-session-balancing-test

package main

import (
    "net"
    "fmt"
    "time"
    "os"
    "strconv"
)

const (
    HOST        = "0.0.0.0"
    PORT        = "5501"
    TYPE        = "tcp"
)

func main() {
    rcv, err := net.Listen(TYPE, fmt.Sprintf("%s:%s", HOST, PORT))
    defer rcv.Close()
    if err != nil {
        fmt.Println("Can't listen.", err.Error())
        os.Exit(1)
    }

    for {
        cnx, err := rcv.Accept()
        if err != nil {
            fmt.Println("Can't accept.", err.Error())
            os.Exit(1)
        }
        // Sane "production-ready" code would say `go handler(cnx)` here.
        //  However, we're interested in making this single-threaded to observe how k8s acts.
        handler(cnx)
    }
}

func handler(cnx net.Conn) {
    hostname, _ := os.Hostname()

    buffer := make([]byte, 1024)
    readlen, err := cnx.Read(buffer)

    defer cnx.Close()

    if err != nil {
        fmt.Println("Can't read.", err.Error())
        return
    }

    rcvData, _ := strconv.ParseInt(string(buffer[:(readlen)]), 10, 64)

    time.Sleep(time.Duration(rcvData) * time.Second)

    cnx.Write([]byte(hostname))
}
