import java.util.*;

public class Main {
	static int n, m, a, b;
	static int t[];
	static int adj[][];
	static double dp[][];

	public static void main(String[] args){
		Scanner sc = new Scanner(System.in);
		n = sc.nextInt();
		m = sc.nextInt();
		a = sc.nextInt();
		b = sc.nextInt();
		t = new int[n];
		for (int i = 0; i < n; i++) {
			t[i] = sc.nextInt();
		}
		// read adjacency matrix
		adj = new int[m][m];
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

		dp = new double[1 << n][m];
		for (int i = 0; i < 1 << n; i++) {
			for (int j = 0; j < m; j++) {
				dp[i][j] = -1.0;
			}
		}
		
		double ans = rec(0, a);
		if (ans < Double.MAX_VALUE) {
			final String ans_string = String.format("%1$.3f", ans);
			System.out.println(ans_string);
		} else {
			System.out.println("Impossible");
		}
		sc.close();
	}

	static double rec(int S, int v) {
		if (dp[S][v] >= 0.0) {
			return dp[S][v];
		}
		if (v == b) {
			dp[S][v] = 0.0;
			return 0.0;
		}
		double min_cost = Double.MAX_VALUE;
		for (int i = 0; i < adj.length; i++) {
			if (adj[v][i] == Integer.MAX_VALUE) {
				continue;
			}
			for (int j = 0; j < t.length; j++) {
				if ((S & (1 << j)) >> j == 1) {
					continue;
				}
				double cost = rec(S | (1 << j), i);
				if (cost < Double.MAX_VALUE) {
					min_cost = Math.min(min_cost, cost + ((double) adj[v][i]) / ((double) t[j]));
				}
			}
		}
		dp[S][v] = min_cost;
		return min_cost;
	}
}
