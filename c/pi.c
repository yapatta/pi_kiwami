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

	mpf_t pi, p, q, a, P, Q, T, A, B, C;

	/* Initialize Floating point numbers */
	mpf_set_default_prec(prec);
	mpf_init(pi);
	mpf_init(p);
	mpf_init(q);
	mpf_init(a);
	mpf_init(P);
	mpf_init(Q);
	mpf_init(T);
	mpf_init(A);
	mpf_init(B);
	mpf_init(C);

	/* Assignment */
	mpf_set_str(A, "13591409", 10);
	mpf_set_str(B, "545140134", 10);
	mpf_set_str(C, "640320", 10);
	mpf_set_ui(P, 1);
	mpf_set_ui(Q, 1);
	mpf_set_ui(T, 0);

	/* Main loop. about 14 * n digits precision */
	for (i = 1; i <= n; i++) {
		/* p = (2 * i - 1) * (6 * i - 5) * (6 * i - 1) */
		mpf_set_ui(p, (2 * i - 1));
		mpf_mul_ui(p, p, (6 * i - 5));
		mpf_mul_ui(p, p, (6 * i - 1));
		/* q = C^3 * i^3 / 24 */
		mpf_set_ui(q, i * i * i);
		mpf_mul(q, q, C);
		mpf_mul(q, q, C);
		mpf_mul(q, q, C);
		mpf_div_ui(q, q, 24);
		/* a = (-1)^i * (A + B * i) */
		mpf_mul_ui(a, B, i);
		mpf_add(a, a, A);
		if (i & 1) {
			mpf_neg(a, a);
		}
		/* P(0, N) = P(0, N - 1) p */
		mpf_mul(P, P, p);
		/* Q(0, N) = Q(0, N - 1) q */
		mpf_mul(Q, Q, q);
		/* T(0, N) = T(0, N - 1) * q + a * P */
		mpf_mul(T, T, q);
		mpf_mul(a, a, P);
		mpf_add(T, T, a);
	}

	/* pi = C ^ (1 / 2)     */
	mpf_sqrt(pi, C);
	/*      * C             */
	mpf_mul(pi, pi, C);
	/*      * Q             */
	mpf_mul(pi, pi, Q);
	/*      / (T + A * Q)   */
	mpf_mul(Q, Q, A);
	mpf_add(Q, Q, T);
	mpf_div(pi, pi, Q);
	/*      / 12            */
	mpf_div_ui(pi, pi, 12);

	fp = fopen("output.txt", "w");
	if (fp == NULL) {
		printf("couldn't open output.txt\n");
		exit(EXIT_FAILURE);
	}

	mpf_out_str(fp, 10, digits, pi);

	mpf_clear(pi);
	mpf_clear(p);
	mpf_clear(q);
	mpf_clear(a);
	mpf_clear(P);
	mpf_clear(Q);
	mpf_clear(T);
	mpf_clear(A);
	mpf_clear(B);
	mpf_clear(C);

	end = clock();
	printf("%.3f s\n",(double)(end - start) / CLOCKS_PER_SEC);
	
	return 0;
}
