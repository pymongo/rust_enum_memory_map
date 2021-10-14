#include <stdio.h>
#include <stdint.h>

enum UserIDKind {
    UserIDKind_Number,
    UserIDKind_Text,
};

struct A {
    enum UserIDKind kind;
    uint64_t number;
    char *text;
};

// `__attribute__((packed))` means no memory padding in strut fields
// C 语言好像没有 Rust 编译器将字段打乱重排的编译器优化，所以只有 padding 没有 align
struct __attribute__((packed)) B {
    enum UserIDKind kind;
    uint64_t number;
    char *text;
};

int main() {
    printf("sizeof(struct A) = %ld\n", sizeof(struct A));
    printf("%ld + %ld + %ld = %ld\n",
        sizeof(enum UserIDKind), sizeof(uint64_t), sizeof(char *),
        sizeof(enum UserIDKind) + sizeof(uint64_t) + sizeof(char *)
    );

    printf("\nafter add __attribute__((packed))\n\n");
    printf("sizeof(struct B) = %ld\n", sizeof(struct B));
    printf("%ld + %ld + %ld = %ld\n",
        sizeof(enum UserIDKind), sizeof(uint64_t), sizeof(char *),
        sizeof(enum UserIDKind) + sizeof(uint64_t) + sizeof(char *)
    );
}
