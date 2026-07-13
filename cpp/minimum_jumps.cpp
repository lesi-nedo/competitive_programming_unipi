#include <bits/stdc++.h>
#include <charconv>
#include <limits>
using namespace std;

int solve(const vector<int>& arr){
    int n = arr.size();
    if (n < 2)
        return 0;
    if (arr[0] == 0 && n > 1)
        return -1;
    int jumps = 0;
    int r = 0;
    int sr = 0;
    while (r < n - 1){
        int fi = r;
        for (int c = sr; c <= r; c++){
            if (fi < c+arr[c]){
                sr = r+1;
                fi = c+arr[c];
            }
        }
        if (fi == r) {
            return -1;
        }
        r = fi;
        jumps += 1;
    }
    return jumps;
}

int main() {
    string line;

    while (getline(cin, line)) {
        string_view v(line.data()+1, line.size()-2);
        vector<int> arr;
        while (true) {
            size_t pos = v.find(",");
            auto sub = v.substr(0, pos);
            size_t start = sub.find_first_not_of(" ");
            sub = sub.substr(start);
            int val = 0;
            auto [ptr, ec] = from_chars(sub.data(), sub.data()+sub.size(), val);
            if (ec == errc()) {
                arr.push_back(val);
            }
            if (pos == string_view::npos) {
                break;
            }
            v.remove_prefix(pos + 1);

        }
        cout << solve(arr) << endl;
    }

}