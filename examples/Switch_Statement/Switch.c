// Switch Statement
//
// C version


int main()
{
        int x=2;
        int y;

        switch(x)
        {
          case 0:       y=2*x;
                        break;
        
          case 1:       y=x+1;
                        break;

          case 2:       y=x-3;
                        break;

          default:      y=x/2;
                        break;
        }
}
