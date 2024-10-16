#include <stdio.h>

float InvSqrt(float x)
{
float xhalf = 0.5f*x;
int i = *(int*)&x;
i = 0x5f3759df - (i>>1);
x = *(float*)&i;
x = x*(1.5f-xhalf*x*x);
return x;
}

int main()
{
    float x = 2.0;
    printf("InvSqrt(%f) = %f\n", x, InvSqrt(x));
}