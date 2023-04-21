#include <iostream>
using namespace std;

int main() {
    int n, k, m, rez, nn;
    double s;
    cin >> nn;
    n = nn;
    s = 0;
    m = 0;
    while (s <= 0.5) {
        s += 1.0 / n;
        m += 1;
        n -= 1;
    }
    m -= 1;
    rez = nn - m;
    cout << rez << endl;
    return 0;
}