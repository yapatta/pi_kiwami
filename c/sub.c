/* Substruct two floating point */
/*  $ gcc sub.c -lgmp -o sub */
#include <stdio.h>
#include <stdlib.h>
#include <gmp.h>
#include <math.h>

int main (int argc, char* argv[]) {

	int digits = 1000;
	int prec = digits * log2(10);

	mpf_t x, y;
	FILE* fpx;
	FILE* fpy;

	if (argc != 3) {
		printf("Usage: sub file1 file2");
		exit(EXIT_FAILURE);
	}

	/* Initialize Floating point numbers */
	mpf_set_default_prec(prec);
	mpf_init(x);
	mpf_init(y);

	/* Read from files */
	fpx = fopen(argv[1], "r");
	if (fpx == NULL) {
		printf("Couldn't load %s", argv[1]);
		exit(EXIT_FAILURE);
	}
	fpy = fopen(argv[2], "r");
	if (fpy == NULL) {
		printf("Couldn't load %s", argv[2]);
		exit(EXIT_FAILURE);
	}
	mpf_inp_str(x, fpx, 10);
	mpf_inp_str(y, fpy, 10);

	/* Substruct x = x - y, and output x */
	mpf_sub(x, x, y);
	mpf_out_str(stdout, 10, 20, x);
	printf("\n");

	mpf_clear(x);
	mpf_clear(y);
	return 0;
}
