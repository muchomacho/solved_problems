import java.util.*;
import java.lang.reflect.Array;

@SuppressWarnings("unchecked")
class Main {
	static Scanner sc;
	static int n, k;
	static HashMap<Integer, Integer>[] adj;
	public static void main(String[] args) {
		sc = new Scanner(System.in);
		n = sc.nextInt();
		k = sc.nextInt();
		adj = (HashMap<Integer, Integer>[]) Array.newInstance(HashMap.class, n + k + 2);
		int n_offset = 2;
		int k_offset = n_offset + n;
		for (int i = 0; i < adj.length; i++) {
			adj[i] = new HashMap<Integer, Integer>();
		}
		for (int i = 0; i < n; i++) {
			adj[0].put(n_offset + i, 1);
			for (int j = 0; j < k; j++) {
				int conn = sc.nextInt();
				if (conn == 1) {
					adj[n_offset + i].put(k_offset + j, 1);
				}
			}
		}
		for (int i = 0; i < k; i++) {
			adj[k_offset + i].put(1, 1);
		}

		System.out.println(max_flow(0, 1));
	}

	public static int max_flow(int src, int snk) {
		HashMap<Integer, Integer>[] resAdj = (HashMap<Integer, Integer>[])Array.newInstance(HashMap.class, adj.length);
		for (int i = 0; i < adj.length; i++) {
			resAdj[i] = (HashMap<Integer, Integer>) adj[i].clone();
		}
		for (int i = 0; i < adj.length; i++) {
			Iterator<Map.Entry<Integer, Integer>> it = adj[i].entrySet().iterator();
			while (it.hasNext()) {
				resAdj[it.next().getKey()].put(i, 0);
			}
		}
		int flow = 0;
		Integer[] path;
		while (true) {
			path = bfs(resAdj, src, snk);
			if (path.length == 0) {
				break;
			}
			int minCap = Integer.MAX_VALUE;
			for (int i = 0; i < path.length - 1; i++) {
				minCap = Integer.min(minCap, resAdj[path[i]].get(path[i + 1]));
			}
			for (int i = 0; i < path.length - 1; i++) {
				int oldCap = resAdj[path[i]].get(path[i + 1]);
				resAdj[path[i]].replace(path[i + 1], oldCap - minCap);
				oldCap = resAdj[path[i + 1]].get(path[i]);
				resAdj[path[i + 1]].replace(path[i], oldCap + minCap);
			}
			flow += minCap;
		}
		return flow;
	}

	public static Integer[] bfs(HashMap<Integer, Integer>[] adj, int src, int snk) {
		ArrayDeque<Edge> queue = new ArrayDeque<Edge>();
		queue.add(new Edge(src, src));
		ArrayDeque<Edge> usedQueue = new ArrayDeque<Edge>();
		boolean[] used = new boolean[adj.length];
		for (int i = 0; i < adj.length; i++) {
			used[i] = false;
		}
		used[src] = true;
		while (queue.size() > 0) {
			Edge e = queue.poll();
			int crt = e.to;
			usedQueue.add(e);
			if (crt == snk) {
				break;
			}
			Iterator<Map.Entry<Integer, Integer>> it = adj[crt].entrySet().iterator();
			while (it.hasNext()) {
				Map.Entry<Integer, Integer> pair = it.next();
				int nxt = pair.getKey();
				int cap = pair.getValue();
				if (used[nxt] || cap == 0) {
					continue;
				}
				queue.add(new Edge(crt, nxt));
				used[nxt] = true;
			}
		}

		ArrayList<Integer> path = new ArrayList<Integer>();
		if (usedQueue.size() == 0) {
			Integer[] arr = new Integer[0];
			return path.toArray(arr);
		}
		Edge tb = usedQueue.pollLast();
		if (tb.to != snk) {
			Integer[] arr = new Integer[0];
			return path.toArray(arr);
		}
		path.add(tb.to);
		path.add(tb.from);
		while (true) {
			Edge newTb = usedQueue.pollLast();
			if (newTb.to == tb.from) {
				path.add(newTb.from);
				if (newTb.from == src) {
					break;
				}
				tb = newTb;
			}
		}
		Collections.reverse(path);
		Integer[] arr = new Integer[path.size()];
		return path.toArray(arr);
	}
}

class Edge {
	public int from;
	public int to;
	public Edge(int from, int to) {
		this.from = from;
		this.to = to;
	}
}
