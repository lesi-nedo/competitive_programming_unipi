#include <bits/stdc++.h>
using namespace std;

struct TimeSpan {
    int s;
    int f;
    int ind;

    TimeSpan(int s, int f, int ind): s(s), f(f), ind(ind) {}
};

vector<int> maximum_meetings_one_room(vector<int> s, vector<int> f) {
    vector<int> res;
    vector<TimeSpan> tr;
    for(size_t ind = 0; ind < s.size(); ind++)
        tr.emplace_back(s[ind], f[ind], ind+1);
    sort(tr.begin(), tr.end(), [](const TimeSpan& a, const TimeSpan& b) {
        if (a.f == b.f)
            return a.ind < b.ind;
        return a.f < b.f;
    });
    res.emplace_back(tr[0].ind);
    auto last_f = tr[0].f;
    for(size_t ind = 1; ind < s.size(); ind++){
        if(tr[ind].s > last_f){
            res.emplace_back(tr[ind].ind);
            last_f = tr[ind].f;
        }
    }
    sort(res.begin(), res.end());
    return res;
}

int main() {
    string s, f;
    while(getline(cin, s)) {
        if (!getline(cin, f)) break;
        stringstream sss(s);
        stringstream ssf(f);
        vector<int> sv (
            (istream_iterator<int>(sss)),
            istream_iterator<int>()
        );
        vector<int> fv (
            (istream_iterator<int>(ssf)),
            istream_iterator<int>()
        );
        auto res = maximum_meetings_one_room(sv, fv);
        for (auto el: res)
            cout << el << " ";
        cout << endl;
    }
}