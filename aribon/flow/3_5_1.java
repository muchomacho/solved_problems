import java.util.*;

class Main {
	static Scanner sc;
	static int n, k;
	static ArrayList<Edge>[] adj;
	public static void main(String[] args) {
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

		System.out.println(max_flow(adj, 0, 1));
	}

	public static int max_flow(ArrayList<Edge>[] adj, int src, int snk) {
		ArrayList<Edge>[] resAdj = adj.clone();
		for (int i = 0; i < resAdj.length; i++) {
			if (resAdj[i].size() == 0) {
				continue;
			}
			for (int j = 0; j < resAdj[i].size(); j++) {
				resAdj[resAdj[i].get(j).to].add(new Edge(j, 0));
			}
		}
		int flow = 0;
		ArrayList<Edge> path;
		while (true) {
			path = bfs(resAdj, src, snk);
			if (path.size() == 0) {
				break;
			}
			int min_cap = Integer.MAX_VALUE;
			for (int i = 0; i < path.size() ; i++) {
				min_cap = Integer.min(min_cap, path.get(i).cap);
			}
			int from = 0;
			for (int i = 0; i < path.size(); i++) {
				for (int j = 0; j < resAdj[from].size(); j++) {
					if (resAdj[from].get(j).to == path.get(i).to) {
						resAdj[from].get(j).cap -= min_cap;
						break;
					}
				}
				for (int j = 0; j < resAdj[path.get(i).to].size(); j++) {
					if (resAdj[path.get(i).to].get(j).to == from) {
						resAdj[path.get(i).to].get(j).cap += min_cap;
						break;
					}
				}
				from = path.get(i).to;
			}
			flow += min_cap;
		}

		return flow;
	}

	public static ArrayList<Edge> bfs(ArrayList<Edge>[] adj, int src, int snk) {
		ArrayDeque<Pair> queue = new ArrayDeque<Pair>();
		queue.add(new Pair(src, new Edge(src, 0)));
		ArrayDeque<Pair> usedQueue = new ArrayDeque<Pair>();
		HashSet<Integer> used = new HashSet<Integer>();
		used.add(src);
		while (queue.size() > 0) {
			Pair p = queue.poll();
			int crt = p.edge.to;
			usedQueue.add(p);
			if (crt == snk) {
				break;
			}
			for (int i = 0; i < adj[crt].size(); i++) {
				if (used.contains(adj[crt].get(i).to) || adj[crt].get(i).cap == 0) {
					continue;
				}
				queue.add(new Pair(crt, adj[crt].get(i)));
				used.add(adj[crt].get(i).to);
			}
		}

		ArrayList<Edge> path = new ArrayList();
		if (usedQueue.size() == 0) {
			return path;
		}
		Pair tb = usedQueue.pollLast();
		if (tb.edge.to != snk) {
			return path;
		}
		path.add(tb.edge);
		while (true) {
			Pair newTb = usedQueue.pollLast();
			if (newTb.edge.to == tb.from) {
				path.add(newTb.edge);
				if (newTb.from == src) {
					break;
				}
				tb = newTb;
			}
		}
		Collections.reverse(path);
		return path;
	}
}


class Edge {
	public int to;
	public int cap;
	public Edge(int to, int cap) {
		this.to = to;
		this.cap = cap;
	}
}

class Pair {
	public int from;
	public Edge edge;
	public Pair(int from, Edge edge) {
		this.from = from;
		this.edge = edge;
	}
}
