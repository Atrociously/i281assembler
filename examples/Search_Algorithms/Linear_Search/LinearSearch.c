// Linear Search


int array[] = {5, 2, 7, 3, 6, 1};
int found=0;
int index;
int N = 6;
int key = 5; // search for this
int i;

void main()
{
    for(i=0; i < N; i++)
    {
         if(array[i] == key) {
              found = 1;
              index=i;
              break;
         }
    }
    // printf("%d", found);
    // printf("%d", index);
}
