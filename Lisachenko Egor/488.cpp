#include <iostream>

using namespace std;

int main()
{
    //cout <<
    //cin >> 
    int n;
    cin >> n;
    int number = 1;
    int array[n][n];
    for(int i = 0; i < n; i++){
        if(i%2==0){
            for(int j = 0; j < n; j++){
                array[i][j] = number;
                number++;
            }
        }
        else {
            for(int j = n-1; j > -1; j--){
                array[i][j] = number;
                number++;
            }
        }
    }
        for(int i = 0; i < n; i++){
            for(int j = 0; j < n; j++){
                cout << array[i][j] << " ";
            }
        cout << "\n";
    }
}