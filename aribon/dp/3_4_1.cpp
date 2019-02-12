#include <iostream>
using namespace std;

#define INF 1000000000

int n;
int** d;
int** dp;

int recursive(int S, int v) {
    if (dp[S][v] >= 0) {
        return dp[S][v];
    }
    if (S == (1 << n) - 1 && v == 0) {
        dp[S][v] = 0;
        return 0;
    } else if (S & 1) {
        dp[S][v] = INF;
        return INF;
    }
    int minimum = INF;
    for (int i = 0; i < n; i++) {
        if (d[v][i] == -1 || S & (1 << i)) {
            continue;
        }
        minimum = min(minimum, recursive(S | (1 << i), i) + d[v][i]);
    }
    dp[S][v] = minimum;
    return minimum;
}

int main() {
    cin >> n;

    d = new int*[n];
    dp = new int*[1 << n];
    for (int i = 0; i < n; i++) {
        d[i] = new int[n];
    }
    for (int i = 0; i < 1 << n; i++) {
        dp[i] = new int[n];
    }
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            cin >> d[i][j];
        }
    }

    for (int i = 0; i < 1 << n; i++) {
        memset(dp[i], -1, sizeof(int) * n);
    }
    cout << recursive(0, 0) << endl;

    for (int i = 0; i < n; i++) {
        delete [] d[i];
    }
    for (int i = 0; i < 1 << n; i++) {
        delete [] dp[i];
    }
    delete [] d;
    delete [] dp;
}