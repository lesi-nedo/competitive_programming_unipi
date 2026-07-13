#include <bits/stdc++.h>

using namespace std;

class SegTree {

    private:
    vector<size_t> tree;
    size_t last;



    size_t query_util(size_t ss, size_t se, size_t sq, size_t eq, size_t si){
        if (ss >= sq && se <= eq) {
            return this->tree[si];
        }
        if (ss > se || ss > eq || se < sq)
            return 0;
        auto md = this->gm(ss, se);
        auto lc = this->glc(si);
        auto rc = this->grc(si, ss, md);
        return max(
            query_util(ss, md, sq, min(eq, md), lc),
            query_util(md+1, se, max(sq, md+1), eq, rc)
        );
    }

    size_t update_util(size_t ss, size_t se, size_t ui, size_t uv, size_t si){
        if (ss == se && ss == ui) {
            this->tree[si] = max(this->tree[si], uv);
            return this->tree[si];
        } else {
            if (ss > se || ss > ui || se < ui)
                return this->tree[si];
        }
        auto md = this->gm(ss, se);
        auto lc = this->glc(si);
        auto rc = this->grc(si, ss, md);
        auto lv = update_util(ss, md, ui, uv, lc);
        auto rv = update_util(md+1, se, ui, uv, rc);

        this->tree[si] = max(lv, rv);
        return this->tree[si];
    }

    size_t gm(size_t s, size_t e) {
        if (e < s){
            string errmsg = "Cannot compute the middle, because the end (" + to_string(e) + ") is smaller than the start (" + to_string(s) + ").";
            throw invalid_argument(errmsg);

        }
        return s + (e - s) / 2;
    }

    size_t glc(size_t i){
        return i + 1;
    }
    size_t grc(size_t i, size_t s, size_t m) {
        return i + 2 * (m - s + 1);
    }

    public:

    SegTree(int n) {
       if (n <= 0){
           throw invalid_argument("n must be greater than 0");
       }
       tree = vector<size_t>(2*n, 0);
       last = static_cast<size_t>(n - 1);
    }

    void update(int ui, size_t uv) {
        if (ui < 0) {
            throw invalid_argument("The compressed value to be updated must be positive.");
        }
        this->update_util(0, this->last, static_cast<size_t>(ui), uv, 0);
    }

    size_t query(int sq, int se){
        if (sq > se)
            return 0;
        if (sq < 0 || se < 0)
            throw invalid_argument("First or last endpoint is less than 0");
        return this->query_util(0, this->last, sq, se, 0);
    }

};

size_t lis (vector<int> &arr) {
    if (arr.empty())
        return 0;
    const auto n = arr.size();
    size_t res = 0;
    vector<int> copy_arr (arr.begin(), arr.end());
    auto sgt = SegTree(copy_arr.size());
    sort(copy_arr.begin(), copy_arr.end());
    copy_arr.erase(unique(copy_arr.begin(), copy_arr.end()), copy_arr.end());
    unordered_map<int, int> comp {};
    for (size_t ind = 0; ind < copy_arr.size(); ind++)
        comp.emplace(copy_arr[ind], ind);

    for (auto x: arr) {
        auto el = comp[x];
        auto nv = sgt.query(0, el-1)+1;
        res = max(res, nv);
        sgt.update(el, nv);
    }
    return res;
}

int main () {
    string nums;
    while(getline(cin, nums)) {
        stringstream ss(nums);
        vector<int> arr (
            (istream_iterator<int> (ss)),
            istream_iterator<int> ()
        );
        cout << lis(arr) << endl;
    }

    return 0;
}