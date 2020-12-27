#include <boost/multiprecision/cpp_int.hpp>
#include <boost/multiprecision/cpp_dec_float.hpp>
#include <iostream>
#include <boost/multiprecision/gmp.hpp>

using namespace std;
namespace mp = boost::multiprecision;

const int N = 1000000;

using BigFloat = mp::number<mp::gmp_float<N>>;

using BigInt = mp::mpz_int;

const BigInt A = 13591409, B = 545140134, C = 640320;

// x_k = X(k, k+1)

// y_k = Y(k, k+1)

// z_k = Z(k, k+1)

// X(0, K) = X(0, 1) * X(1, 2) ... X(k-1, k)を求める
// X(0, K) = X(0, K-1) * x_{k-1}を求める
// X(0, 1) = x_0 = 1

// Z(0, K) = Z(0, 1) * Z(1, 2) ... Z(k-1, k)を求める
// Z(0, K) = Z(0, K-1) * z_{k-1}を求める
// Z(0, 1) = z_0 = (-1) * 1 * 1 * 5 = -5

// Y(0, K) = Z(0, k-1)*Y(n-1,n)-Y(0,n-1)*X(n-1,n)を求める
// X(n-1,n) = x_{n-1}
// Y(0, 1) = y_0 = A

int main()
{
	BigInt bX = 1, bY = A, bZ = -5, aX, aY, aZ;
	// X, Y, Z
	for (int i = 2; i <= N; i++)
	{
		aX = bX * (mp::pow(BigInt(i-1), 3) * mp::pow(C, 3) / 24);
		aY = bZ * (A + (i-1) * B) + bY * (mp::pow(BigInt(i-1), 3) * mp::pow(C, 3) / 24);
		aZ = bZ * ((-1) * (6 * BigInt(i-1) + 1) * (2 * BigInt(i-1) + 1) * (6 * BigInt(i-1) + 5));

		bX = aX;
		bY = aY;
		bZ = aZ;
	}

	BigFloat tmp = mp::sqrt(BigFloat(C) * BigFloat(C) * BigFloat(C)) / 12;

	cout << setprecision(N) << tmp * bX / bY << endl;
	return 0;
}
