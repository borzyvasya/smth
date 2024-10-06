#include <cstdlib>
#include <iostream>
#include <vector>

using namespace std;

int main() {
    
    int row = 10;
    int column = 10;
    
    vector<vector<int>> matrix(row, vector<int>(column));
   
    for (int i = 0; i < row; i++) {
        for (int j = 0; j < column; j++) {
            matrix[i][j] = rand() % 10+1;
        }
    } 
   
    for (int i = 0; i < row; i++) {
        for (int j = 0; j < column; j++) {
            cout<<matrix[i][j]<< " ";
        }
        cout<<"\n";
    } 
   
    cout<<"\n";
   

    int multiplication = 1;
    for (int i = 0; i < row; i++) {
        multiplication *= matrix[i][i];
    }
    cout<<multiplication<<endl;
    
    for (int i = 0; i < row; i++) {
        multiplication *= matrix[i][i - row - 1];
    }
    cout<<multiplication<<endl;
   
    
    for (int i = 0; i < row; i++) {
        for (int j = 0; j < column; j++) {
           if (i > j) {
               matrix[i][j] = 0;
           }
        }
    }
    
    for (int i = 0; i < row; i++) {
        for (int j = 0; j < column; j++) {
           cout<<matrix[i][j]<< " ";
        }
        cout<<"\n";
    } 
    

    return 0;
}