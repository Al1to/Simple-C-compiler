float test(int a, int b) {}

int main() {
    int a = 9;
    if (a > 4) {
        ++a;
    } 
    else if (a < 1) {
        --a;
    } 
    else if (a <= 3) {
        a = 2;
    } 
    else {
        a += 27;
    }

    float f = test(a, 67 + 9) + 5.3;
}
