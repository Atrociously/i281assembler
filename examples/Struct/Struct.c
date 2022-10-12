// Struct
//
// C version

struct point{
        int x;
        int y;
        int z;
}; 

int main()
{
        struct point p;
        p.x = 2;
        p.y = 3;
        p.z = p.x + p.y;
        
        // print the struct
        //   printf("%d, ", p.x);
        //   printf("%d, ", p.y);
        //   printf("%d, ", p.z);
}
