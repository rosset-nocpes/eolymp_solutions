#include <iostream>
#include <iomanip>
using namespace std;

int main()
{
    //cout<<"Hello World";
    double l, v, n;
    cin >> l >> v >> n;
    int const N = n;
    int array[N][2];
    for(int i = 0; i<N; i++){
        for(int j = 0; j < 2; j++){
            cin >> array[i][j];
        }
    }
    double extra_v = 0;
    double extra_t = 0;
    for(int i = 0; i < N; i++){
        extra_v+=array[i][0];
    }
    for(int i = 0; i < N; i++){
        extra_t+=array[i][1];
    }
    double answer = l/(v+extra_v)+extra_t;
    if(l/v>answer){
        cout <<fixed<<setprecision(6)<< answer;
    }
    else{
        cout <<fixed<<setprecision(6)<< l/v;
    }
    return 0;
}