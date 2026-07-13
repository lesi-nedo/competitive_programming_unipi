#include <bits/stdc++.h>
#include <string>

using namespace std;

int lsc(const string &s1, const string &s2) {
    const auto n1 = s1.size();
    const auto n2 = s2.size();
    const auto n1p1 = n1 + 1;
    const auto n2p1 = n2 + 1;
    if (n1 == 0 || n2 == 0)
        return 0;
    vector<unsigned int> mtr (n1p1*n2p1, 0);

    for (size_t ind_s1 = 1; ind_s1 <= n1; ind_s1++) {

        for (size_t ind_s2 = 1; ind_s2 <= n2; ind_s2++) {
            auto value = max(mtr[(ind_s1 - 1)*n2p1 + ind_s2], mtr[ind_s1*n2p1 + ind_s2 - 1]);
            if (s1[ind_s1-1] == s2[ind_s2-1])
               value = mtr[(ind_s1-1)*n2p1 + ind_s2 - 1] + 1;

            mtr[ind_s1*n2p1 + ind_s2] = value;

        }

    }
    return mtr[n1p1*n2p1-1];
}

int main() {

   string s1, s2;
   while (getline(cin, s1)) {
       if (!(getline(cin, s2))) break;
       cout << "Result: " << lsc(s1, s2) << endl;
   }

    return 1;
}