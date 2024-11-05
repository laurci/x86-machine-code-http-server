section .data

RESPONSE:
    db 'HTTP/1.1 200 OK', 0xd, 0xa
    db 'Content-Type: text/html', 0xd, 0xa
    db 'Content-Length: 154', 0xd, 0xa, 0xd, 0xa
    db '<!DOCTYPE html>'
    db '<html>'
    db '<head>'
    db '<title>Elves and CSS</title>'
    db '<style>body { background-color: #f0f0f0; }</style>'
    db '</head>'
    db '<body>'
    db '<h1>Elves and CSS</h1>'
    db '</body>'
    db '</html>'
RESPONSE_LEN equ $ - RESPONSE

MSG_SERVER_STARTED:
    db 'Server started on port 8080', 0xa
MSG_SERVER_STARTED_LEN equ $ - MSG_SERVER_STARTED

SERVER_ADDR:
    dw 2                ; sin_family = AF_INET
    dw 0x961f           ; sin_port = htons(8080)
    dd 0x00000000       ; sin_addr = INADDR_ANY (0.0.0.0)
    times 8 db 0x00     ; padding to make it 16 bytes total

; variables
server_sock dd 0x00
client_sock dd 0x00
request_buffer times 1024 db 0x00

section .text
global _start

_start:
    ; socket(AF_INET, SOCK_STREAM, 0)
    mov eax, 0x167
    mov ebx, 2 ; AF_INET
    mov ecx, 1 ; SOCK_STREAM
    mov edx, 0
    int 0x80

    ; save the server socket
    mov [server_sock], eax

    ; bind(server_sock, SERVER_ADDR, 16)
    mov eax, 0x169
    mov ebx, [server_sock]
    mov ecx, SERVER_ADDR
    mov edx, 16
    int 0x80

    ; listen(server_sock, 10)
    mov eax, 0x16b
    mov ebx, [server_sock]
    mov ecx, 10
    int 0x80

    ; write(1, MSG_SERVER_STARTED, MSG_SERVER_STARTED_LEN)
    mov eax, 0x4
    mov ebx, 1
    mov ecx, MSG_SERVER_STARTED
    mov edx, MSG_SERVER_STARTED_LEN
    int 0x80

handle_conn:
    ; accept(server_sock, NULL, NULL)
    mov eax, 0x16c
    mov ebx, [server_sock]
    mov ecx, 0
    mov edx, 0
    mov esi, 0
    int 0x80

    ; save the client socket
    mov [client_sock], eax

    ; read(client_sock, request_buffer, 1024)
    mov eax, 0x3
    mov ebx, [client_sock]
    mov ecx, request_buffer
    mov edx, 1024
    int 0x80

    ; write(client_sock, RESPONSE, RESPONSE_LEN)
    mov eax, 0x4
    mov ebx, [client_sock]
    mov ecx, RESPONSE
    mov edx, RESPONSE_LEN
    int 0x80

    ; close(client_sock)
    mov eax, 0x6
    mov ebx, [client_sock]
    int 0x80

    ; loop back to accept another connection
    jmp handle_conn