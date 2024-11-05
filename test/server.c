#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define PORT 8080

const char RESPONSE[] = "HTTP/1.1 200 OK\r\n"
                        "Content-Type: text/html\r\n"
                        "Content-Length: 203\r\n\r\n"
                        "<!DOCTYPE html>"
                        "<html>"
                        "<head>"
                        "<title>Elves and CSS</title>"
                        "<style>body { background-color: #f0f0f0; }</style>"
                        "</head>"
                        "<body>"
                        "<h1>Elves and CSS</h1>"
                        "</body>"
                        "</html>";

const int RESPONSE_LEN = sizeof(RESPONSE);

int main()
{
    struct sockaddr_in server_addr;
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    int server_sock = socket(AF_INET, SOCK_STREAM, 0);

    bind(server_sock, (struct sockaddr *)&server_addr, sizeof(server_addr));
    listen(server_sock, 10);

    printf("Server started on port %d\n", PORT);

    char _request[1024]; // 1KB buffer for response
    while (1)
    {
        int client_sock = accept(server_sock, NULL, NULL);
        read(client_sock, _request, sizeof(_request)); // we ignore the request, but we still need to read it

        write(client_sock, RESPONSE, RESPONSE_LEN);

        close(client_sock);
    }

    close(server_sock);

    return 0;
}
