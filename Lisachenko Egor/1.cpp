#include <iostream>
#include <iomanip>
using namespace std;

int main()
{
    int num;
    cin >> num;
    if(num==100){
        cout <<"1 0 1";
    }
    else {
    cout << num%100/10 << ' ' << num%10;
        
    }
    return 0;
}