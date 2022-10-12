// Insertion Sort
//
// C version

int array[]={2, 3, 4, 1};
int N = 4;

int main()
{
        int i;
        int j;
        int INS;
        
        for( i=1; i< N; i++)
        {
                j = i;
                INS = array [i];
                
                while( (j>0) && (array[j-1] < INS) )
                {
                        array[j] = array[j-1];
                        j--;
                }
                
                array[j] = INS;         // do the insertion
        }
        
        // print the array1
        // for(i=0; i< N; i++) {
        //   printf("%d, ", array[i]);
        // }
}
