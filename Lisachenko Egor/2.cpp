#include <iostream>
#include <iomanip>
using namespace std;

int main()
{
    int num;
    int count = 1;
    cin >> num;
    int i = 10;
    while(i<=num){
        if(num%i>=0){
            count++;
            i*=10;
        }
    }
    cout << count;
    return 0;
}