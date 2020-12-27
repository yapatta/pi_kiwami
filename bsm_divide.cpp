#include <boost/multiprecision/cpp_int.hpp>
#include <boost/multiprecision/cpp_dec_float.hpp>
#include <iostream>
#include <tuple>
#include <boost/multiprecision/gmp.hpp>

using namespace std;
namespace mp = boost::multiprecision;

const int N = 100000000;

using BigFloat = mp::number<mp::gmp_float<N>>;

using BigInt = mp::mpz_int;

using XYZ = tuple<BigInt, BigInt, BigInt>;

const BigInt A = 13591409, B = 545140134, C = 640320;

// x_k = X(k, k+1)
BigInt calc_x(int k)
{
	if (k == 0)
		return BigInt(1);
	return mp::pow(BigInt(k), 3) * mp::pow(C, 3) / 24;
}

// y_k = Y(k, k+1)
BigInt calc_y(int k)
{
	return A + B * k;
}

// z_k = Z(k, k+1)
BigInt calc_z(int k)
{
	return (-1) * (6 * BigInt(k) + 1) * (2 * BigInt(k) + 1) * (6 * BigInt(k) + 5);
}

XYZ calc(int l, int r)
{
	if (r - l == 1)
	{
		return make_tuple(calc_x(l), calc_y(l), calc_z(l));
	}

	int m = (l + r) >> 1;

	XYZ lc = calc(l, m);
	XYZ rc = calc(m, r);

	BigInt lx = get<0>(lc);
	BigInt rx = get<0>(rc);
	BigInt X = lx * rx;

	BigInt lz = get<2>(lc);
	BigInt rz = get<2>(rc);
	BigInt Z = lz * rz;

	BigInt ly = get<1>(lc);
	BigInt ry = get<1>(rc);
	BigInt Y = rx * ly + ry * lz;

	return make_tuple(X, Y, Z);
}

int main()
{

	BigFloat tmp = mp::sqrt(BigFloat(C) * BigFloat(C) * BigFloat(C)) / 12;

	XYZ xyz = calc(0, N);
	BigInt X = get<0>(xyz);
	BigInt Y = get<1>(xyz);

	cout << setprecision(N) << tmp * X / Y << endl;
	return 0;
}
