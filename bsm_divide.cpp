#include <boost/multiprecision/cpp_int.hpp>
#include <boost/multiprecision/cpp_dec_float.hpp>
#include <iostream>
#include <boost/multiprecision/gmp.hpp>

using namespace std;
namespace mp = boost::multiprecision;

const int64_t N = 100000000;
const int64_t n = N / 14;

using BigFloat = mp::number<mp::gmp_float<N>>;
using BigInt = mp::mpz_int;

const BigInt A = 13591409, B = 545140134, C = 640320;
const BigInt CT = C * C * C;
const BigInt CTd24 = CT / 24;

struct XYZ
{
	BigInt x, y, z;
};

// x_k = X(k, k+1)
inline BigInt calc_x(int64_t k)
{
	if (k == 0)
		return 1;
	return (k * k) * CTd24 * k;
}

// y_k = Y(k, k+1)
inline BigInt calc_y(int64_t k)
{
	return A + B * k;
}

// z_k = Z(k, k+1)
inline BigInt calc_z(int64_t k)
{
	if (k == n - 1)
		return 0;
	return (-1) * BigInt((6 * k + 1) * (2 * k + 1)) * (6 * k + 5);
}

XYZ calc(int64_t l, int64_t r)
{
	if (r - l == 1)
	{
		return {calc_x(l), calc_y(l), calc_z(l)};
	}

	int64_t m = (l + r) >> 1;

	XYZ lc = calc(l, m);
	XYZ rc = calc(m, r);

	return {lc.x * rc.x, rc.x * lc.y + rc.y * lc.z, lc.z * rc.z};
}

int main()
{
	XYZ xyz = calc(0, n);
	BigFloat p = mp::sqrt(BigFloat(CT)) * xyz.x / 12 / xyz.y;

	cout << setprecision(N) << p << endl;
	return 0;
}
