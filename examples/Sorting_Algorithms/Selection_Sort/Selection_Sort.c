// Selection Sort
//
// C version

int array[]={2, 3, 4, 1};
int last = 3;              // last valid index in the array

int main()
{
        int i;
        int j;
        int maxIndex;
        
        for( i=0; i< last; i++)
        {
                maxIndex = i;
                for( j=i+1; j <= last; j++)
                {
                        if(array[j] > array[maxIndex])
                                maxIndex = j;
                }
                
                // swap
                int temp = array[i];
                array[i] = array[maxIndex];
                array[maxIndex] = temp;
        }
        
        // print the array
        // for(i=0; i<=last; i++) {
        //   printf("%d, ", array[i]);
        // }
}
