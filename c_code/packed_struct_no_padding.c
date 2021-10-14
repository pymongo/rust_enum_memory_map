#include <stdio.h>
#include <stdint.h>

enum UserIDKind {
    UserIDKind_Number,
    UserIDKind_Text,
};

struct A {
    enum UserIDKind kind;
    // padding 4 byte
    uint64_t number;
    char *text;
};

/* __attribute__((packed)) is same as #[repr(packed)] */
// `__attribute__((packed))` means no memory padding in strut fields
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
    // 4 + 8 + 8 = 20
    printf("%ld + %ld + %ld = %ld\n",
        sizeof(enum UserIDKind), sizeof(uint64_t), sizeof(char *),
        sizeof(enum UserIDKind) + sizeof(uint64_t) + sizeof(char *)
    );
}
