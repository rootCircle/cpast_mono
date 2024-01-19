#include <bits/stdc++.h>
using namespace std;

int product(vector<int> v)
{
    int product = 1;
    for (auto &value : v)
    {
        product *= value;
    }
    return product;
}

int main()
{
    int t;
    cin >> t;
    while (t--)
    {
        int n;
        cin >> n;
        vector<int> a;
        for (int i = 0; i < n; i++)
        {
            int x;
            cin >> x;
            a.push_back(x);
        }

        int ct = 0;
        while (true)
        {
            int y = product(a);
            if (y < 0)
            {
                auto m = min_element(a.begin(), a.end());
                a.erase(m);
                ct++;
            }
            else
                break;
        }
        cout << ct << endl;
    }
    return 0;
}