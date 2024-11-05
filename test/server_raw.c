int syscall_1(int number, int arg1)
{
    int result;

    __asm__ volatile(
        "movl %1, %%eax\n"
        "movl %2, %%ebx\n"
        "int $0x80\n"
        "movl %%eax, %0\n"
        : "=r"(result)
        : "g"(number), "g"(arg1)
        : "%eax", "%ebx");

    return result;
}

int syscall_2(int number, int arg1, int arg2)
{
    int result;

    __asm__ volatile(
        "movl %1, %%eax\n"
        "movl %2, %%ebx\n"
        "movl %3, %%ecx\n"
        "int $0x80\n"
        "movl %%eax, %0\n"
        : "=r"(result)
        : "g"(number), "g"(arg1), "g"(arg2)
        : "%eax", "%ebx", "%ecx");

    return result;
}

int syscall_3(int number, int arg1, int arg2, int arg3)
{
    int result;

    __asm__ volatile(
        "movl %1, %%eax\n"
        "movl %2, %%ebx\n"
        "movl %3, %%ecx\n"
        "movl %4, %%edx\n"
        "int $0x80\n"
        "movl %%eax, %0\n"
        : "=r"(result)
        : "g"(number), "g"(arg1), "g"(arg2), "g"(arg3)
        : "%eax", "%ebx", "%ecx", "%edx");

    return result;
}

int syscall_4(int number, int arg1, int arg2, int arg3, int arg4)
{
    int result;

    __asm__ volatile(
        "movl %1, %%eax\n"
        "movl %2, %%ebx\n"
        "movl %3, %%ecx\n"
        "movl %4, %%edx\n"
        "movl %5, %%esi\n"
        "int $0x80\n"
        "movl %%eax, %0\n"
        : "=r"(result)
        : "g"(number), "g"(arg1), "g"(arg2), "g"(arg3), "g"(arg4)
        : "%eax", "%ebx", "%ecx", "%edx", "%esi");

    return result;
}

#define SYS_READ 0x03
#define SYS_WRITE 0x04
#define SYS_CLOSE 0x06
#define SYS_SOCKET 0x167
#define SYS_BIND 0x169
#define SYS_LISTEN 0x16b
#define SYS_ACCEPT4 0x16c

void _start()
{

    const char RESPONSE[] = "HTTP/1.1 200 OK\r\n"
                            "Content-Type: text/html\r\n"
                            "Content-Length: 154\r\n\r\n"
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

    int server_sock = syscall_3(SYS_SOCKET, 2, 1, 0); // socket(AF_INET, SOCK_STREAM, 0)

    char server_addr[] = {
        0x02, 0x00,                                    // AF_INET
        0x1f, 0x90,                                    // htons(8080)
        0x00, 0x00, 0x00, 0x00,                        // INADDR_ANY
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 // padding
    };

    syscall_3(SYS_BIND, server_sock, &server_addr, 16); // bind(server_sock, &server_addr, sizeof(server_addr))
    syscall_2(SYS_LISTEN, server_sock, 10);             // listen(server_sock, 10)

    syscall_3(SYS_WRITE, 1, "Server started on port 8080\n", 29); // write(1, "Server started on port 8080\n", 27)

    char _request[1024];
    while (1)
    {
        int client_sock = syscall_4(SYS_ACCEPT4, server_sock, 0, 0, 0);    // accept4(server_sock, NULL, NULL, 0)
        syscall_3(SYS_READ, client_sock, _request, sizeof(_request));      // read(client_sock, _request, sizeof(_request))
        syscall_3(SYS_WRITE, client_sock, RESPONSE, sizeof(RESPONSE) - 1); // write(client_sock, RESPONSE,  sizeof(RESPONSE) - 1)
        syscall_1(SYS_CLOSE, client_sock);                                 // close(client_sock)
    }
}