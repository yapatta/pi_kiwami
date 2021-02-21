#include <stdio.h>
#include <gmp.h>

int main (int argc, char* argv[]) {

	unsigned long int i;
	mpz_t a;

	mpz_init(a);
	mpz_set_ui(a, 1);

	for (i = 1; i <= 100; i++) {
		mpz_mul_ui(a, a, i);
	}

	i--;

	printf("%ld! = ", i);
	mpz_out_str(stdout, 10, a);
	printf("\n");

	mpz_clear(a);
	return 0;
}
