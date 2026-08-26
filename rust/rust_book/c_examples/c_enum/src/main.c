#include <stdio.h>

enum ret {
    OK,
    ERR,
};

// enum asfasdfads {
//     OK,
//     ERR,
// };

enum ret my_operation(int v) {
    if (v > 5) {
        return OK;
    } else {
        return ERR;
    }
}

int main() {
    int a = 8;
    
    enum ret result = my_operation(a);
    
    if (result == OK) {
        printf("alles goed!!\n");
    } else {
        printf("niet goed!!\n");
    }
    
    printf("is_true(%d): %d\n", a, my_operation(a));
    printf("hello you\n");
    return 0;
}
