// Binary Search


int array[] = {2, 4, 5, 7, 8, 9};
int found = 0;
int mid;
int low = 0;
int high = 5;
int key = 4;  //search for this


void main()
{
    while (low <= high) {
       mid = (low + high) / 2;
            if (array[mid] == key){
                 found = 1;
                 break;
            }
            else if (array[mid] < key)
                 low = mid + 1;
            else
                 high = mid - 1;
       }
        
       // printf("%d", found);
       // printf("%d", mid);
}
