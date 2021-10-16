#include <stdio.h>
#include <stdint.h>
#include <stddef.h> /* offsetof */

/*
C 语言没有 Rust 那种编译器字段重排的优化
*/
struct __attribute__((packed)) A {
    uint8_t a;
    uint32_t b;
    uint16_t c;
};

struct B {
    uint8_t a;
    // pading [u8; 7]
    uint32_t b;
    uint16_t c;
};

// objdump --source a.out
int main() {
    printf("sizeof(A) = %ld\n", sizeof(struct A));
    printf("offsetof(struct A, a) = %zu\n", offsetof(struct A, a));
    printf("offsetof(struct A, b) = %zu\n", offsetof(struct A, b));
    printf("offsetof(struct A, c) = %zu\n", offsetof(struct A, c));
    printf("\n\n");
    printf("offsetof(struct B, a) = %zu\n", offsetof(struct B, a));
    printf("offsetof(struct B, b) = %zu\n", offsetof(struct B, b));
    printf("offsetof(struct B, c) = %zu\n", offsetof(struct B, c));
    return 0;
}
