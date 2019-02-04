import java.util.*;

public class Main {
	static long k;
	static int n;
	static int M;
	static int[][] matVal;
	static Matrix mat;
	static int[][] initVal;
	static Matrix init; 
	static public void main(String[] args) {
		Scanner sc = new Scanner(System.in);
		n = sc.nextInt();
		k = sc.nextLong();
		M = sc.nextInt();

		matVal = new int[2 * n][2 * n];
		initVal = new int[2 * n][n];
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++) {
				matVal[i][j] = 0;
				matVal[i + n][j] = 0;
				matVal[i][j + n] = 0;
				matVal[i + n][j + n] = sc.nextInt();
				initVal[i][j] = matVal[i + n][ j + n];
				initVal[i + n][j] = matVal[i + n][j + n];
			}
			matVal[i][i] = 1;
			matVal[i + n][i] = 1;
		}
		mat = new Matrix(matVal);
		init = new Matrix(initVal);

		Matrix ans = mat.power(k - 1, M).matmul(init, M);
		
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++) {
				System.out.print(ans.values[i + n][j]);
				System.out.print(" ");
			}
			System.out.println("");
		}
	}
}

class Matrix {
	public static int bitNum = 64;
	public int rowNum, colNum;
	public int[][] values;
	public Matrix(int[][] values) {
		this.rowNum = values.length;
		this.colNum = values[0].length;
		this.values = values;
	}

	public Matrix power(long x, int res) {
		boolean[] bin = new boolean[bitNum];
		for (int i = 0; i < bitNum; i++) {
			if ((x >> i & 1) == 1) {
				bin[i] = true;
			}
		}

		Matrix[] powers = new Matrix[bitNum];
		powers[0] = this;
		for (int i = 1; i < bitNum; i++) {
		       powers[i] = powers[i - 1].matmul(powers[i - 1], res);
		}

		Matrix ans = Matrix.eye(this.rowNum);
		for (int i = 0; i < bitNum; i++) {
			if (bin[i]) {
				ans = ans.matmul(powers[i], res);
			}
		}

		return ans;
	}

	public Matrix matmul(Matrix m, int res) {
		int[][] ans = new int[this.rowNum][this.colNum];
		for (int i = 0; i < this.rowNum; i++) {
			for (int j = 0; j < m.colNum; j++) {
				ans[i][j] = 0;
				for (int k = 0; k < this.colNum; k++) {
					ans[i][j] = (ans[i][j] + this.values[i][k] * m.values[k][j]) % res;
				}
			}
		}

		return new Matrix(ans);
	}

	public Vector vecmul(Vector v, int res) {
		int[] ans = new int[this.rowNum];
		for (int i = 0; i < this.rowNum; i++) {
			ans[i] = 0;
			for (int j = 0; j < this.colNum; j++) {
				ans[i] = (ans[i] + this.values[i][j] * v.values[j]) % res;
			}
		}

		return new Vector(ans);
	}

	public static Matrix eye(int n) {
		int[][] ans  = new int[n][n];

		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++) {
				ans[i][j] = 0;
			}
			ans[i][i] = 1;
		}

		return new Matrix(ans);
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
