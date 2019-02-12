import java.util.*;

class Main {
	static int n, k;
	static ArrayList<Edge>[] adj;
	public static main(String[] args) {
		sc = new Scanner(System.in);
		n = sc.nextInt();
		k = sc.nextInt();
		adj = new ArrayList[n + k + 2];
		int n_offset = 2;
		int k_offset = n_offset + n;
		for (int i = 0; i < adj.length; i++) {
			adj[i] = new ArrayList();
		}
		for (int i = 0; i < n; i++) {
			adj[0].add(new Edge(n_offset + i, 1));
			for (int j = 0; j < k; j++) {
				int conn = sc.nextInt();
				if (conn == 1) {
					adj[n_offset + i].add(new Edge(k_offset + j, 1));
				}
			}
		}
		for (int i = 0; i < k; i++) {
			adj[k_offset + i].add(new Edge(1, 1));
		}

		int ans = 0;
	}

	public static int max_flow(ArrayList<Edge>[] adj, int src, int snk) {
		ArrayList<Edge>[] res_adj = adj.copy();
		for (int i = 0; i < res_adj.length; i++) {
			if (res_adj[i].length == 0) {
				continue;
			}
			for (int j = 0; j < res_adj[i].length; j++) {
				res_adj[res_adj[i][j].to].add(new Edge(j, 0));
			}
		}
		int flow = 0;
		ArrayList<int> path;
		while (true) {
			path = bfs(res_adj, src, snk);
			if (path.length == 0) {
				break;
			}
			int min_cap = Integer.MAX_VALUE;
			for (int i = 0; i < path.length - 1; i++) {
				for (int j = 0; j < res_adj[path[i]]; j++) {
				}
				min_cap = Integer.min(min_cap, path[i].cap);
			}
			int from = 0;
			for (int i = 0; i < path.length; i++) {
				for (int j = 0; j < res_adj[from].length; j++) {
					if (res_adj[from][j].to == path[i].to) {
						res_adj[from][j].cap -= min_cap;
						break;
					}
				}
				for (int j = 0; j < res_adj[path[i].to]; j++) {
					if (res_adj[path[i].to][j].to == from) {
						res_adj[path[i][j].cap += min_cap;
					}
				}
			}
		}

		return flow;
	}

	public static ArrayList<int> bfs(ArrayList<Edge>[] adj, int src, int snk) {
		ArrayDeque<Pair> queue = new ArrayDeque();
		queue.add(new ArrayList({ new Pair(0, 0) }));
		ArrayDeque<Pair> used_queue = new ArrayDeque();
		Set<int> used = new Set();
		used.add(0);
		while (queue.length > 0) {
			Pair p = queue.poll();
			int prv = p.prv;
			int crt = p.crt;
			used_deque.add(p);
			if (crt == snk) {
				break;
			}
			for (int i = 0; i < adj[crt].length; i++) {
				if (used.contains(adj[crt][i].to)) {
					continue;
				}
				queue.add((crt, adj[crt][i].to));
				used.add(adj[crt][i].to);
			}
		}

		ArrayList<int> path = new ArrayList();
		if (used_deque.length == 0) {
			return path;
		}
		Pair trace_back = used_deque.poll_last();
		if (trace_back.crt != snk) {
			return path;
		}
		path.add(trace_back.crt);
		path.add(trace_back.prv);
		while (true) {
			Pair new_trace_back = used_deque.poll_last();
			if (new_trace_back.crt == trace_back.prv) {
				path.add(new_trace_back.prv);
				if (new_trace_back.prv == src) {
					break;
				}
				trace_back = new_trace_back;
			}
		}
		Collections.reverse(path);
		return path;
	}


				

			

}


class Edge {
	public int to;
	public int cap;
	public Edge(int to; int cap) {
		this.to = to;
		this.cap = cap;
	}
}

class Pair {
	public int prv;
	public int crt;
	public Pair(int prv, int crt) {
		this.prv = prv;
		this.crt = crt;
	}
}
