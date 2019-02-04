import java.util.*;

public class Main {
	static long k;
	static int n;
	static int[][] matVal;
	static Matrix adj;
	static public void main(String[] args) {
		Scanner sc = new Scanner(System.in);
		n = sc.nextInt();
		k = sc.nextLong();

		matVal = new int[n][n];
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++) {
				matVal[i][j] = sc.nextInt();
			}
		}
		adj = new Matrix(matVal);
		
		Matrix paths = adj.power(k);
		int ans = 0;
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++) {
				ans += paths.values[i][j];
			}
		}
		System.out.println(ans);
	}
}

class Matrix {
	public static int bitNum = 64;
	public int res = 10007;
	public int dim;
	public int[][] values;
	public Matrix(int[][] values) {
		this.dim = values.length;
		this.values = values;
	}

	public Matrix power(long x) {
		boolean[] bin = new boolean[bitNum];
		for (int i = 0; i < bitNum; i++) {
			if ((x >> i & 1) == 1) {
				bin[i] = true;
			}
		}

		Matrix[] powers = new Matrix[bitNum];
		powers[0] = this;
		for (int i = 1; i < bitNum; i++) {
		       powers[i] = powers[i - 1].matmul(powers[i - 1]);
		}

		int[][] eye = new int[this.dim][this.dim];
		for (int i = 0; i < this.dim; i++) {
			for (int j = 0; j < this.dim; j++) {
				if (i == j) {
					eye[i][j] = 1;
				} else {
					eye[i][j] = 0;
				}
			}
		}
		Matrix ans = new Matrix(eye);
		
		for (int i = 0; i < bitNum; i++) {
			if (bin[i]) {
				ans = ans.matmul(powers[i]);
			}
		}

		return ans;
	}

	public Matrix matmul(Matrix m) {
		int[][] ans = new int[this.dim][this.dim];
		for (int i = 0; i < this.dim; i++) {
			for (int j = 0; j < this.dim; j++) {
				ans[i][j] = 0;
				for (int k = 0; k < this.dim; k++) {
					ans[i][j] = (ans[i][j] + this.values[i][k] * m.values[k][j]) % res;
				}
			}
		}

		return new Matrix(ans);
	}

	public Vector vecmul(Vector v) {
		int[] ans = new int[this.dim];
		for (int i = 0; i < this.dim; i++) {
			ans[i] = 0;
			for (int j = 0; j < this.dim; j++) {
				ans[i] = (ans[i] + this.values[i][j] * v.values[j]) % res;
			}
		}

		return new Vector(ans);
	}
}

class Vector {
	public int dim;
	public int[] values;
	public Vector(int[] values) {
		this.dim = values.length;
		this.values = values;
	}

}
