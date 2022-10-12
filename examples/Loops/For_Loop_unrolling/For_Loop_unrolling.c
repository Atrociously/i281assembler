// For Loop Unrolling
//
// C version


int main()
{
        int N=3;   // this is a small number
        int i, sum;
        
        sum=0;
        for(i=1; i<=N; i++)
           sum+=i;
        

        // which is equivalent to
        // // (N is not needed in this case)
        // int i, sum;
        // 
        // sum=0;
        // i=1;                // i<-1
        // sum+=i;
        // i++;                // i<-2
        // sum+=i;
        // i++;                // i<-3
        // sum+=i;

        // printf("%d\n", sum);
}
