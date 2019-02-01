import java.util.*;

public class Main {
	static int n;
	static int[][] mat_val = {{2, 1, 0}, {2, 2, 2}, {0, 1, 2}};
	static Matrix update = new Matrix(mat_val);
	static int[] vec_val = {2, 2, 0};
	static Vector init = new Vector(vec_val);
	static public void main(String[] args) {
		Scanner sc = new Scanner(System.in);
		n = sc.nextInt();

		Vector ans = update.power(n - 1).vecmul(init);
		System.out.println(ans.values[0]);
	}
}

class Matrix {
	public int res = 10007;
	public int dim;
	public int[][] values;
	public Matrix(int[][] values) {
		this.dim = values.length;
		this.values = values;
	}

	public Matrix power(int x) {
		boolean[] bin = new boolean[32];
		for (int i = 0; i < 32; i++) {
			if ((x >> i & 1) == 1) {
				bin[i] = true;
			}
		}

		Matrix[] powers = new Matrix[32];
		powers[0] = this;
		for (int i = 1; i < 32; i++) {
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
		
		for (int i = 0; i < 32; i++) {
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
