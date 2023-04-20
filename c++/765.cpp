#include <iostream>
#include <cstring>
using namespace std;

int main() {
    int dp[100010];
    int l, v, n, vi, t, mx;
    cin >> l >> v >> n;
    memset(dp, 0x3F, sizeof(dp));
    dp[0] = mx = 0;
    for(int i = 0; i < n; i++) {
        cin >> vi >> t;
        for(int j = mx; j >= 0; j--) {
            if(dp[j] + t < dp[j + vi]) {
                dp[j + vi] = dp[j] + t;
            }
        }
        mx += vi;
    }

    double res = 1e10;
    for(int i = 0; i <= mx; i++) {
        double temp = 1.0 * l / (v + i) + dp[i];
        if(temp < res) {
            res = temp;
        }
    }
    printf("%.6lf\n", res);
    return 0;
}