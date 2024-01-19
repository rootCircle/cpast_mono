#include<bits/stdc++.h>
using namespace std;

int main()
{

    int t;

    cin>>t;

    while(t>0)

    {

        int n;

        cin>>n;

        int arr[n];

        int count=0;

        int flag=0;

        for(int i=0;i<n;i++)

        {

            cin>>arr[i];

            

            if(arr[i]==0)

            {

                flag=1;
            }

            

            if(arr[i]<0)

            {

                count++;

            }

        }

            

                   

        if(count%2==0 || flag!=0)

        cout<<"0"<<endl;

        

        else

        cout<<(count%2)<<endl;        

        t--;

    }



    return 0;

}
