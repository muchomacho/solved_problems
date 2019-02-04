import java.util.*;

public class Main {
	static int n;
	static int m;
	static int[] values;
	static Pair[] sorters;
	static public void main(String[] args) {
		Scanner sc = new Scanner(System.in);
		n = sc.nextInt();
		m = sc.nextInt();
		sorters = new Pair[m];
		for (int i = 0; i < m; i++) {
			int l = sc.nextInt() - 1;
			int r = sc.nextInt() - 1;
			sorters[i] = new Pair(l, r);
		}

		values = new int[n];
		values[0] = 0;
		for (int i = 1; i < n; i++) {
			values[i] = Integer.MAX_VALUE;
		}
		SegTree dp = new SegTree(values);

		for (int i = 0; i < m; i++) {
			int minVal = dp.query(sorters[i].l, sorters[i].r - 1);
			if (minVal == Integer.MAX_VALUE) {
				continue;
			}
			dp.update(sorters[i].r, minVal + 1);
		}

		System.out.println(dp.query(n - 1, n - 1));
	}
		
}

class SegTree {
	public int length;
	public int size;
	public int[] values;

	public SegTree(int[] values) {
		this.length = values.length;
		int highestBit = Integer.highestOneBit(this.length);
		if (this.length == highestBit) { 
			this.size = highestBit;
		} else {
			this.size = highestBit << 1;
		}
		this.values = new int[2 * this.size - 1];
		for (int i = 0; i < this.length; i++) {
			this.values[this.size - 1 + i] = values[i];
		}
		for (int i = this.size - 2; i >= 0; i--) {
			this.values[i] = Integer.min(this.values[2 * i + 1], this.values[2 * i + 2]);
		}
	}

	public void update(int x, int val) {
		int l = 0;
		int r = this.size;
		int index = 0;
		while (l < r) {
			int mid = (l + r) / 2;
			this.values[index] = Integer.min(this.values[index], val);
			if (x <= mid) {
				r = mid;
				index = 2 * index + 1;
			} else {
				l = mid + 1;
				index = 2 * index + 2;
			}
		}
		this.values[index] = Integer.min(this.values[index], val);
	}

	public int query(int start, int end) {
		return this.queryRec(start, end, 0, 0, this.size);
	}

	int queryRec(int start, int end, int index, int l, int r) {
		if (start == l && end == r) {
			return this.values[index];
		}
		int mid = (l + r) / 2;
		if (end <= mid) {
			return this.queryRec(start, end, 2 * index + 1, l, mid);
		}
		if (start > mid) {
			return this.queryRec(start, end, 2 * index + 2, mid + 1, r);
		}
		int l_val = this.queryRec(start, mid, 2 * index + 1, l, mid);
		int r_val = this.queryRec(mid + 1, end, 2 * index + 2, mid + 1, r);
		return Integer.min(l_val, r_val);
	}
}

class Pair {
	public int l, r;
	public Pair(int l, int r) {
		this.l = l;
		this.r = r;
	}
}