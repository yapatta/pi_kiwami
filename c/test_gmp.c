#include <stdio.h>
#include <gmp.h>

int main() {
	mpz_t a, b;

	mpz_init(a);
	mpz_init(b);

	mpz_set_ui(a, 12345);
	mpz_set_str(b, "12345678910987654321", 10);

	mpz_out_str(stdout, 10, a);
	printf("\n");
	mpz_out_str(stdout, 10, b);
	printf("\n");

	mpz_clear(a);
	mpz_clear(b);

	return 0;
}
