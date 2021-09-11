#include <stdio.h>
#include <stdlib.h>

long long int my_sum(long long int a, long long int b);

int main(void)
{
	FILE *fp_read, *fp_write;
	long long int num_1, num_2;

	fp_read = fopen("input.txt", "r");

	if (fp_read == NULL)
		{
			return 1;
		}

	fscanf(fp_read, "%lld", &num_1);
	fscanf(fp_read, "%lld", &num_2);

	fclose(fp_read);

	const long long int result = my_sum(num_1, num_2);

	fp_write = fopen("output.txt", "w+");

	const char *result_buffer = malloc(sizeof result + 1);

	fprintf(fp_write, "%lld", result);
	fclose(fp_write);

	return 0;
}


long long int my_sum(long long int a, long long int b)
{
	return a + b * b;
}
