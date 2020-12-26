#include <boost/multiprecision/cpp_int.hpp>
#include <boost/multiprecision/cpp_dec_float.hpp>
#include <iostream>
#include <boost/multiprecision/gmp.hpp>

using namespace std;
namespace mp = boost::multiprecision;

const int N = 10000;

using BigFloat = mp::number<mp::gmp_float<N>>;

using BigInt = mp::mpz_int;

const BigInt A = 13591409, B = 545140134, C = 640320;

// X[k] = X(0, k+1)
vector<BigInt> X(N + 1, 0), Y(N + 1, 0), Z(N + 1, 0);

// x_k = X(k, k+1)
inline BigInt calc_x(int k)
{
    if (k == 0)
        return BigInt(1);
    return mp::pow(BigInt(k), 3) * mp::pow(C, 3) / 24;
}

// y_k = Y(k, k+1)
inline BigInt calc_y(int k)
{
    return A + B * k;
}

// z_k = Z(k, k+1)
inline BigInt calc_z(int k)
{
    return (-1) * (6 * BigInt(k) + 1) * (2 * BigInt(k) + 1) * (6 * BigInt(k) + 5);
}

// X(0, K) = X(0, 1) * X(1, 2) ... X(k-1, k)を求める
// X(0, K) = X(0, K-1) * x_{k-1}を求める
// X(0, 1) = x_0 = 1
inline BigInt calc_X(int k)
{
    if (X[k] != 0)
        return X[k];
    if (k == 1)
        return (X[1] = calc_x(0));
    // X(k-1, k) = x_{k-1}
    return (X[k] = (calc_X(k - 1) * calc_x(k - 1)));
}

// Z(0, K) = Z(0, 1) * Z(1, 2) ... Z(k-1, k)を求める
// Z(0, K) = Z(0, K-1) * z_{k-1}を求める
// Z(0, 1) = z_0 = (-1) * 1 * 1 * 5 = -5
inline BigInt calc_Z(int k)
{
    if (Z[k] != 0)
        return Z[k];
    if (k == 1)
        return (Z[1] = calc_z(0));
    return (Z[k] = (calc_Z(k - 1) * calc_z(k - 1)));
}

// Y(0, K) = Z(0, k-1)*Y(n-1,n)-Y(0,n-1)*X(n-1,n)を求める
// X(n-1,n) = x_{n-1}
// Y(0, 1) = y_0 = A
// FIXME: マイナスになる
inline BigInt calc_Y(int k)
{
    if (Y[k] != 0)
        return Y[k];
    if (k == 1)
        return (Y[1] = calc_y(0));
    return (Y[k] = calc_Z(k - 1) * calc_y(k - 1) + calc_Y(k - 1) * calc_x(k - 1));
}

int main()
{

    BigFloat tmp = mp::sqrt(BigFloat(C) * BigFloat(C) * BigFloat(C)) / 12;

    cout << setprecision(N) << tmp * calc_X(N) / calc_Y(N) << endl;
    return 0;
}
