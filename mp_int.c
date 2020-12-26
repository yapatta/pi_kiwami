/* Calculate pi based on Chudnovsky algorithm, using GMP */
/* [1] Computation of 2700 billion decimal digits of Pi using a Desktop Computer,
   Fabrice Bellard, Feb 11 2010 (4th revision),
http://bellard.org/pi/pi2700e9/pipcrecord.pdf */
#include <stdio.h>
#include <stdlib.h>
#include <gmp.h>
#include <math.h>
#include <time.h>

int main (int argc, char* argv[]) {

	clock_t start, end;
	FILE* fp;

	int digits = 20000;
	int prec = digits * log2(10);
	int i;
	int n = digits / 14;

	start = clock();

	mpf_t pi, temp;
	mpz_t p, q, a, P, Q, T, A, B, C, C3over24;

	/* Initialize GMP numbers */
	mpf_set_default_prec(prec);
	mpf_init(pi);
	mpf_init(temp);
	mpz_init(p);
	mpz_init(q);
	mpz_init(a);
	mpz_init(P);
	mpz_init(Q);
	mpz_init(T);
	mpz_init(A);
	mpz_init(B);
	mpz_init(C);
	mpz_init(C3over24);

	/* Assignment */
	mpz_set_str(A, "13591409", 10);
	mpz_set_str(B, "545140134", 10);
	mpz_set_str(C, "640320", 10);
	mpz_mul(C3over24, C, C);
	mpz_mul(C3over24, C3over24, C);
	mpz_div_ui(C3over24, C3over24, 24);
	mpz_set_ui(P, 1);
	mpz_set_ui(Q, 1);
	mpz_set_ui(T, 0);

	/* Main loop. about 14 * n digits precision */
	for (i = 1; i <= n; i++) {
		/* p = (2 * i - 1) * (6 * i - 5) * (6 * i - 1) */
		mpz_set_ui(p, (2 * i - 1));
		mpz_mul_ui(p, p, (6 * i - 5));
		mpz_mul_ui(p, p, (6 * i - 1));
		/* q = C^3 * i^3 / 24 */
		mpz_set_ui(q, i * i);
		mpz_mul_ui(q, q, i);
		mpz_mul(q, q, C3over24);
		/* a = (-1)^i * (A + B * i) */
		mpz_mul_ui(a, B, i);
		mpz_add(a, a, A);
		if (i & 1) {
			mpz_neg(a, a);
		}
		/* P(0, N) = P(0, N - 1) p */
		mpz_mul(P, P, p);
		/* Q(0, N) = Q(0, N - 1) q */
		mpz_mul(Q, Q, q);
		/* T(0, N) = T(0, N - 1) * q + a * P */
		mpz_mul(T, T, q);
		mpz_mul(a, a, P);
		mpz_add(T, T, a);
	}

	/* pi = C ^ (1 / 2)     */
	mpf_set_z(temp, C);
	mpf_sqrt(pi, temp);
	/*      * C             */
	mpf_mul(pi, pi, temp);
	/*      * Q             */
	mpf_set_z(temp, Q);
	mpf_mul(pi, pi, temp);
	/*      / (T + A * Q)   */
	mpz_mul(Q, A, Q);
	mpz_add(Q, Q, T);
	mpf_set_z(temp, Q);
	mpf_div(pi, pi, temp);
	/*      / 12            */
	mpf_div_ui(pi, pi, 12);

	/* mpf_out_str(stdout, 10, digits, pi); */
	fp = fopen("output.txt", "w");
	if (fp == NULL) {
		printf("couldn't open output.txt");
		exit(EXIT_FAILURE);
	}
	mpf_out_str(fp, 10, digits, pi);

	mpf_clear(pi);
	mpf_clear(temp);
	mpz_clear(p);
	mpz_clear(q);
	mpz_clear(a);
	mpz_clear(P);
	mpz_clear(Q);
	mpz_clear(T);
	mpz_clear(A);
	mpz_clear(B);
	mpz_clear(C);
	mpz_clear(C3over24);

	end = clock();
	printf("%.3f s\n",(double)(end - start) / CLOCKS_PER_SEC);

	return 0;
}
