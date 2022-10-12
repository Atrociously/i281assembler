// Bubble Sort
//
// C version

int array[] = {7, 3, 2, 1, 6, 4, 5, 8};
int last = 7; // last valid index in the array
int temp;
int i,j;


int main()
{
   for (i = 0; i < last; i++)
        for (j = 0; j < last-i; j++)
               if (array[j] > array[j+1]){
                      temp = array[j];
                      array[j] = array[j+1];
                      array[j+1] = temp;
               }

   //for(i = 0; i < N; i++){
   //    printf("%d, ", array[i]);
   //}
}
