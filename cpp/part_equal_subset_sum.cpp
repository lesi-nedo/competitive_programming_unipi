#include <bits/stdc++.h>

using namespace std;

bool part_equal_subset_sum(vector<int> &arr) {
    const auto n = arr.size();
    if (n < 2) {
        return false;
    }
    auto sum = accumulate(arr.begin(), arr.end(), 0);
    if (sum % 2 == 1)
        return false;
    auto target = sum / 2;
    vector<bool> dp (target+1, false);
    dp[0] = true;
    for (auto x: arr) {
        if (x > target)
            return false;
        for (size_t t = target; t > x; t--){
            dp[t] = dp[t] || dp [t - x];
        }
    }
    return dp[target];
}

int main() {
    string nums;

    while(getline(cin, nums)) {
        stringstream ss(nums);
        vector<int> arr (
            (istream_iterator<int>(ss)),
            istream_iterator<int>()
        );

        cout  << part_equal_subset_sum(arr) << endl;
    }

}