#include<iostream>
using namespace std;
int bin(int a){
if(a==1) return 2;
if(a==2) return 4;
return bin(a-1)+bin(a-2); 
}
int main(){
int n; cin>>n;
cout<<bin(n)<<endl; 
}