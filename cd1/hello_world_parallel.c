// OpenMP header 
#include <omp.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char* argv[])
{
        int nthreads, tid;

        // Begin of parallel region 
        #pragma omp parallel                   
        {
        printf("Hello World... from thread = %d\n",
                omp_get_thread_num());
        }

        printf("soy develop");
        
}
// testeando OpenMP