import java.util.*;

public class Main {
	static int n, m, M;
	static boolean[][] colors;
	static int[][] dp;
       	
	static public void main(String[] args) {
		Scanner sc = new Scanner(System.in);
		n = sc.nextInt();
		m = sc.nextInt();
		M = sc.nextInt();
		
		colors = new boolean[n][m];
		for (int i = 0; i < n; i++) {
			String line = sc.next();
			for (int j = 0; j < m; j++) {
				if (line.charAt(j) == 'x') {
					colors[i][j] = true;
				} else {
					colors[i][j] = false;
				}
			}
		}
		
		dp = new int[2][1 << m];
		for (int i = 0; i < 2; i++) {
			for (int j = 0; j < 1 << m; j++) {
				dp[i][j] = 0;
			}
		}
		
		for (int i = n - 1; i >= 0; i--) {
			for (int j = m - 1; j >= 0; j--) {
				int crt = (i * m + j) % 2;
			        int nxt = (crt + 1) % 2;
				for (int k = 0; k < 1 << m; k++) {
					if (i == n - 1 && j == m - 1 && k == (1 << j)) {
						dp[crt][k] = 1;
					} else if ((k >> j & 1) == 1 || colors[i][j]) {
						dp[crt][k] = dp[nxt][k & ~(1 << j)];
					} else {
						int res = 0;
						if (j + 1 < m && !colors[i][j + 1] && (k >> (j + 1) & 1) == 0) {
							res += dp[nxt][k | 1 << (j + 1)];
						}
						if (i + 1 < n && !colors[i + 1][j]) {
							res = (res + dp[nxt][k | 1 << j] % M);
						}
						dp[crt][k] = res;
					}
				}
			}
		}

		System.out.println(dp[0][0]);
	}
}
							
					
				


