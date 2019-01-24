import java.util.*;
public class Main {
	public static void main(String[] args){
		Scanner sc = new Scanner(System.in);
		int n = sc.nextInt();
		int m = sc.nextInt();
		int a = sc.nextInt();
		int b = sc.nextInt();
		int t[] = new int[n];
		for (int i = 0; i < n; i++) {
			t[i] = sc.nextInt();
		}
		// read adjacency matrix
		int adj[][] = new int[m][m];
		for (int i = 0; i < m; i++) {
			for (int j = 0; j < m; j++) {
				String s = sc.next();
				if (s.equals("INF")) {
					adj[i][j] = Integer.MAX_VALUE;
				} else {
					adj[i][j] = Integer.parseInt(s);
				}
			}
		}

		double dp[][] = new double[1 << n][m];
		for (int i = 0; i < 1 << n; i++) {
			for (int j = 0; j < m; j++) {
				dp[i][j] = -1.0;
			}
		}
		
		double ans = rec(t, dp, adj, 0, a, b);
		if (ans < Double.MAX_VALUE) {
			System.out.println(ans);
		} else {
			System.out.println("Impossible");
		}
		sc.close();
	}

	static double rec(int[] t, double[][] dp, int[][] adj, int S, int start, int end) {
		if (dp[S][start] >= 0.0) {
			return dp[S][start];
		}
		if (start == end) {
			dp[S][start] = 0.0;
			return 0.0;
		}
		double min_cost = Double.MAX_VALUE;
		for (int i = 0; i < adj.length; i++) {
			if (adj[start][i] == Integer.MAX_VALUE) {
				continue;
			}
			for (int j = 0; j < t.length; j++) {
				if ((S & (1 << j)) >> j == 1) {
					continue;
				}
				double cost = rec(t, dp, adj, S | (1 << j), i, end);
				if (cost < Double.MAX_VALUE) {
					min_cost = Math.min(min_cost, cost + ((double) adj[start][i]) / ((double) t[j]));
				}
			}
		}
		dp[S][start] = min_cost;
		return min_cost;
	}
}
