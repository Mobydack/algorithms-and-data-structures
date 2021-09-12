#include <stdio.h>
#include <stdlib.h>

void insertion_sort(long long int *pointer, short n);
void swap(long long int *a, long long int *b);

int main(void)
{
	FILE *fp_read, *fp_write;
	short n;

	fp_read = fopen("input.txt", "r");

	if (fp_read == NULL)
	{
		return 1;
	}

	fscanf(fp_read, "%hd", &n);

	long long int arr[n];
	short i = 0;

	while (!feof(fp_read) && i < n)
	{
		fscanf(fp_read, "%lld", &arr[i]);

		i++;
	}


	fclose(fp_read);

	/* insertion_sort(); */

	/* fp_write = fopen("output.txt", "w+"); */

	/* const char *result_buffer = malloc(sizeof result + 1); */

	/* fprintf(fp_write, "%lld", result); */
	/* fclose(fp_write); */

	return 0;
}


void insertion_sort(long long int *pointer, short n)
{
	printf("Sorting array");
}

void swap(long long int *a, long long int *b)
{
	long long int temp = *b;

	*b = *a;
	*a = temp;
}
