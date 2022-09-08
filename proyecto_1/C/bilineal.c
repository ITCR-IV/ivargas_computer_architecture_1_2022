#include <stdlib.h>
#include <stdio.h>

// c2i = coordinates to index
unsigned int c2i(unsigned int row, unsigned int col, unsigned int w) {
	return row*w + col;
}

int bil_interpol(const char* img, unsigned int h, unsigned int w) {
	unsigned int new_h = 3*h - 2;
	unsigned int new_w = 3*w - 2;
	char* const new_img = malloc(new_h*new_w);

	// Primero copiar todos los valores originales
	for(int r = 0; r < h; r++) {
		for(int c = 0; c < w; c++) {
			new_img[c2i(r*3, c*3, new_w)] = img[c2i(r, c, w)];
		}
	}

	// Primero verticales entre pixeles OG
	for(int new_r = 0; new_r < new_h; new_r++){
		for(int c = 0; c < w; c++) {
			int new_i = c2i(new_r, c*3, new_w);
			int mod = new_r % 3;

			if(mod == 0) {
				new_img[new_i] = img[c2i(new_r/3, c, w)];
			}
			else {
				int up_ref = img[c2i(new_r/3, c, w)];
				int down_ref = img[c2i((new_r + 3)/3, c, w)];
				new_img[new_i] = ((3-mod) * up_ref + mod * down_ref)/3;
			}
		}
	}

	// Segundo horizontales todos
	for(int new_r = 0; new_r < new_h; new_r++){
		for(int new_c = 0; new_c < new_w; new_c++) {
			int new_i = c2i(new_r, new_c, new_w);
			int mod = new_c % 3;
			int inv_mod = 3-mod;

			if(mod == 0) {
				continue;
			}

			else {
				int left_ref = new_img[c2i(new_r, new_c - mod, new_w)];
				int right_ref = new_img[c2i(new_r, new_c + inv_mod, new_w)];
				unsigned int result = ((inv_mod) * left_ref + mod * right_ref)/3;

				new_img[new_i] = result;
			}
		}
	}


	// Print OG
	printf("Original matrix:");
	for(int r = 0; r < h; r++){
		printf("\n");
		for(int c = 0; c < w; c++) {
			printf("%d, ", img[c2i(r, c, w)]);
		}
	}
	printf("\n\n");

	printf("Interpolated matrix:");
	for(int new_r = 0; new_r < new_h; new_r++){
		printf("\n");
		for(int new_c = 0; new_c < new_w; new_c++) {
			printf("%d,	", new_img[c2i(new_r, new_c, new_w)]);
		}
	}

	free(new_img);

}

int main() {
	char* M = malloc(16);

	for(int r = 0; r < 4; r++){
		for(int c = 0; c < 4; c++) {
			M[c2i(r, c, 4)] = (c+1)*10 + r*20;
		}
	}

	M[15] = 0;

	bil_interpol(M, 4, 4);

	free(M);

	return 0;
}
