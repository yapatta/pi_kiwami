#include <boost/multiprecision/cpp_int.hpp>
#include <boost/multiprecision/cpp_dec_float.hpp>
#include <iostream>
#include <boost/multiprecision/gmp.hpp>
#include <thread>
#include <vector>
#include <future>

using namespace std;
namespace mp = boost::multiprecision;

const int N = 10000000;
const int n = N / 14;

using BigFloat = mp::number<mp::gmp_float<N>>;
using BigInt = mp::mpz_int;

const BigInt A = 13591409, B = 545140134, C = 640320;
const BigInt CT = C * C * C;

struct M
{
    BigInt X, Y, Z;
};

const BigInt CT24 = C * C * C / 24;
inline BigInt calcX(long long k)
{
    if (k == 0)
        return 1;
    return k * k * CT24 * k;
}

inline BigInt calcY(long long k)
{
    return A + B * k;
}

inline BigInt calcZ(long long k)
{
    if (k == n - 1)
        return 0;
    return -1 * (6 * k + 1) * BigInt(2 * k + 1) * (6 * k + 5);
    //(6k+1)(6k+2)(6k+3)(6k+4)(6k+5)(6k+6)
}

inline M mul(const M &lm, const M &rm)
{
    return {lm.X * rm.X, lm.Z * rm.Y + lm.Y * rm.X, lm.Z * rm.Z};
}

M calcM(int l, int r, int tn = -1)
{
    if (r == l)
    {
        return {1, 0, 1};
    }
    if (r - l == 1)
    {
        return {calcX(l), calcY(l), calcZ(l)};
    }

    int mid = (l + r) / 2;
    M lm = calcM(l, mid), rm = calcM(mid, r);
    if (tn != -1)
        cerr << tn << "end" << endl;
    return mul(lm, rm);
}

M threadedM(int l, int r, int tnum)
{
    vector<future<M>> fus;
    int len = (r - l + tnum - 1) / tnum, c = l;
    for (int i = 0; tnum - 1 > i; i++)
    {
        fus.push_back(async(&calcM, c, c + len, i));
        c += len;
    }
    M ret = calcM(c, r, tnum - 1);
    for (auto &fu : fus)
    {
        ret = mul(ret, fu.get());
    }
    return ret;
}

int main(int argc, char *argv[])
{
    BigFloat p;
    // M m = calcM(0, n);
    // int tnum = thread::hardware_concurrency();
    int tnum = 1;
    if (argc > 1)
        tnum = atoi(argv[1]);
    cerr << N << endl
         << tnum << endl;
    M m = threadedM(0, n, tnum);
    cerr << "M" << endl;
    p = sqrt(BigFloat(CT)) * m.X / 12 / m.Y;
    cerr << "T" << endl;
    cout << setprecision(N) << p << endl;
    return 0;
}
