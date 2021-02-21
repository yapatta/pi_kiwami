#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <complex.h>
#include <assert.h>

#define PI2 6.28318530717958647692

#define R 18
#define N (1<<R)

typedef double complex cmplx;

void printAr(cmplx *A, int n){
    for(int i = 0; n > i; i++)
        printf("%f+%fi ", creal(A[i]), cimag(A[i]));
    printf("\n");
}

// 動的に
void fftr(cmplx *A, int k){
    if(k == 0) return;
    else{
        int n = 1<<k;
        cmplx *Ae = malloc(sizeof(cmplx) * n/2);
        cmplx *Ao = malloc(sizeof(cmplx) * n/2);
        for(int i = 0; n/2 > i; i++) 
            Ae[i] = A[i*2], Ao[i] = A[i * 2 + 1];

        fftr(Ae, k - 1);
        fftr(Ao, k - 1);

        for(int i = 0; n/2 > i; i++) 
            A[i] = Ae[i] + Ao[i] * cexp(-i*I*PI2/n);
        for(int i = 0; n/2 > i; i++) 
            A[i+n/2] = Ae[i] - Ao[i] * cexp(-i*I*PI2/n);

        free(Ao);
        free(Ae);
    }
}

// インラインで頑張っている
void fftrpost(cmplx *A, int k){
    if(k == 0) return;
    int n = 1 << k;
    fftrpost(A,k-1);
    fftrpost(A+n/2,k-1);
    for(int i = 0; n/2 > i; i++) {
        cmplx ev = A[i];
        A[i] = ev + A[i+n/2] * cexp(-i*I*PI2/n);
        A[i+n/2] = ev - A[i+n/2] * cexp(-i*I*PI2/n);
    }
}

void fftrp(cmplx *A, int k){
    int n = 1 << k;
    for(int i = 0; n > i; i++){
        int to = __builtin_bitreverse32(i) >> (32-k);
        if(to < i) {
            cmplx buf = A[i];
            A[i] = A[to];
            A[to] = buf;
        }
    }
    fftrpost(A,k);
}

void fft(cmplx *A, int k){
    int i,j,l,n = 1 << k;
    cmplx tbl[N] = {},buf;
    for(int i = 0; n > i; i++){
        int to = __builtin_bitreverse32(i) >> (32-k);
        if(to < i) {
            cmplx buf = A[i];
            A[i] = A[to];
            A[to] = buf;
        }
    }
    
    tbl[0] = 1;
    for(j = 1; k >= j; j++){
        int sn = 1 << j;
        for(i = 0; i < j; i++) 
            tbl[1<<i] = cexp(-(1<<i)*I*PI2/sn);
        for(i = 0; sn/2 > i; i++) {
            if(i!=(i&-i))
                tbl[i] = tbl[i&-i]*tbl[i^(i&-i)];
            
            for(l = 0; n > l; l += sn){
                buf = A[l+i+sn/2] * tbl[i];
                A[l+i] += buf;
                A[l+i+sn/2] = A[l+i] - 2*buf;
            }
        }
    }
}

cmplx CC[N];
cmplx CH[N];

#define FFT fft

void convolve(const int *A, const int *B, int *C, int n){
    int k = 1, l = 0;
    n++;
    while(k < n) k<<=1,l++;
    l++,k*=2;

    for(int i = 0; n > i; i++){
        CH[i] = A[i] + I*B[i];
    }
    FFT(CH, l);

    for(int i = 0; k/2 > i; i++){
        cmplx buf = i==0?conj(CH[0]):conj(CH[k-i]);
        cmplx buf2 = conj(CH[k/2-i]);
        cmplx fr = conj((CH[i] + buf) * (CH[i] - buf) / (I*4));
        cmplx bk = conj((CH[k/2+i] + buf2) * (CH[k/2+i] - buf2) / (I*4));
        CC[i] = ((fr+bk) +  I*(fr-bk)*cexp(-i*I*PI2/k))/2;
    }
    FFT(CC, l-1);
    
    for(int i = 0; k/2 > i; i++){
        C[i*2] = 2*round(creal(CC[i]))/k;
        C[i*2 + 1] = 2*round(cimag(CC[i]))/k;
    }
}
