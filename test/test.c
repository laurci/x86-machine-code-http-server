#include <stdio.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/ip.h>

#define print_constant(constant) printf(#constant " = %d (%ld bytes)\n", constant, sizeof(constant))
#define print_sizeof(type) printf("sizeof(" #type ") = %ld bytes\n", sizeof(type))

int main()
{
    print_constant(AF_INET);
    print_constant(SOCK_STREAM);
    print_constant(SOL_SOCKET);
    print_constant(SO_REUSEADDR);

    print_sizeof(sa_family_t);
    print_sizeof(in_port_t);
    print_sizeof(socklen_t);

    return 0;
}