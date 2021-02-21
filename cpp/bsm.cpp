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
vector<BigInt> x(N, 0), y(N, 0), z(N, 0);

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
    // x, y, z
    for (int i = 0; i < N; i++)
    {
        if (i == 0)
        {
            x[0] = BigInt(1);
        }
        else
        {
            x[i] = mp::pow(BigInt(i), 3) * mp::pow(C, 3) / 24;
        }

        y[i] = A + B * i;

        z[i] = (-1) * (6 * BigInt(i) + 1) * (2 * BigInt(i) + 1) * (6 * BigInt(i) + 5);
    }

    // X, Z
    for (int i = 1; i <= N; i++)
    {
        if (i == 1)
        {
            X[1] = x[0];
            Z[1] = z[0];
        }
        else
        {
            X[i] = X[i - 1] * x[i - 1];
            Z[i] = Z[i - 1] * z[i - 1];
        }
    }
    // Y
    for (int i = 1; i <= N; i++)
    {
        if (i == 1)
        {
            Y[1] = y[0];
        }
        else
        {
            Y[i] = Z[i - 1] * y[i - 1] + Y[i - 1] * x[i - 1];
        }
    }

    BigFloat tmp = mp::sqrt(BigFloat(C) * BigFloat(C) * BigFloat(C)) / 12;

    cout << setprecision(N) << tmp * X[N] / Y[N] << endl;
    return 0;
}
