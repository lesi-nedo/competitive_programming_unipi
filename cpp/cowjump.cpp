#include <bits/stdc++.h>


using namespace std;

using ll = long long;

struct Point {
    ll x, y;
};

struct Segment {
    Point a,b;
};

enum EventType {
    Begin,
    End
};

struct Event {
    Point* p;
    EventType e;
    int id;

    Event(Point* p_, EventType e_, int id_)
            : p(p_), e(e_), id(id_) {}
};
long double y_at(Segment& s, ll x) {
    if (s.a.x == s.b.x) return static_cast<long double>(s.a.y);
    return static_cast<long double>(s.a.y) + static_cast<long double>(s.b.y-s.a.y) * static_cast<long double>(x - s.a.x) / static_cast<long double>(s.b.x - s.a.x);
}

struct SegmentComparator {
    std::vector<Segment>* segments;
    long long* sweep_x;

    bool operator()(int id1, int id2) const {
        if (id1 == id2) return false;
        long double y1 = y_at((*segments)[id1], *sweep_x);
        long double y2 = y_at((*segments)[id2], *sweep_x);
        if (y1 != y2) return y1 < y2;
        return id1 < id2;
    }
};

ll orient(Point& a, Point& b, Point& c) {
    return (b.x - a.x) * (c.y - a.y) -
           (b.y - a.y) * (c.x - a.x);

}

bool on_segment(Point& a, Point& b, Point& p) {
    return min(a.x, b.x) <= p.x && p.x <= max(a.x, b.x) &&
           min(a.y, b.y) <= p.y && p.y <= max(a.y, b.y);
}

bool intersect (Segment& s1, Segment& s2) {
    auto& [a,b] = s1;
    auto& [c, d] = s2;
    auto o1 = orient(a, b, c);
    auto o2 = orient(a, b, d);
    auto o3 = orient(c, d, a);
    auto o4 = orient(c, d, b);

    if (((o1 > 0 && o2 < 0) || (o1 < 0 && o2 > 0)) && 
        ((o3 > 0 && o4 < 0) || (o3 < 0 && o4 > 0))) return true;

    if (o1 == 0 && on_segment(a, b, c)) return true;
    if (o2 == 0 && on_segment(a, b, d)) return true;
    if (o3 == 0 && on_segment(c, d, a)) return true;
    if (o4 == 0 && on_segment(c, d, b)) return true;
    return false;
}

int main() {
  freopen("cowjump.in", "r", stdin);
  freopen("cowjump.out", "w", stdout);
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  int n;

  while (cin >> n) {
    vector<Event> es;
    vector<Segment> segments;
    segments.resize(n);
    es.reserve(2*n);
    for (int i = 0; i < n; i++) {
        cin >> segments[i].a.x >> segments[i].a.y >> segments[i].b.x >> segments[i].b.y;
        if (segments[i].a.x > segments[i].b.x)
            swap(segments[i].a, segments[i].b);

        es.emplace_back(&segments[i].a, EventType::Begin, i);
        es.emplace_back(&segments[i].b, EventType::End, i);
    }
    sort(es.begin(), es.end(), [](const auto& a, const auto& b) {
        const auto& [pa, ea, ida] = a;
        const auto& [pb, eb, idb] = b;
        if (pa->x != pb->x)
            return pa->x < pb->x;
        if (ea != eb)
            return ea < eb;
        if (pa->y != pb->y)
            return pa->y < pb->y;
        return ida < idb;
    });
    ll curr_sweep_x;
    SegmentComparator cmp {&segments, &curr_sweep_x};
    set<int, SegmentComparator> active (cmp);

    int ans_id = -1;
    auto check_and_set = [&](int id1, int id2) {
        int count1 = 0, count2 = 0;
        for (int i = 0; i < n; i++) {
            if (i != id1 && intersect(segments[id1], segments[i])) count1++;
            if (i != id2 && intersect(segments[id2], segments[i])) count2++;
        }
        if (count1 > 1) ans_id = id1 + 1;
        else if (count2 > 1) ans_id = id2 + 1;
        else ans_id = min(id1, id2) + 1;
    };

    for(auto& [p, e, id] : es){
        curr_sweep_x = p->x;
        if (e == EventType::Begin){
            auto itr = active.insert(id).first;
            auto& s = segments[*itr];
            auto above = next(itr);
            auto below = (itr == active.begin()) ? active.end() : prev(itr);
            if (above != active.end() && intersect(s, segments[*above])){
                check_and_set(id, *above);
                break;
            }
            if (below != active.end() && intersect(s, segments[*below])){
                check_and_set(id, *below);
                break;
            }
        }
        if (e == EventType::End) {
            auto itr = active.find(id);
            if (itr != active.end()) {
                auto above = next(itr);
                auto below = (itr == active.begin()) ? active.end() : prev(itr);
                if (above != active.end() && below != active.end() && intersect(segments[*above], segments[*below])){
                    check_and_set(*above, *below);
                    break;
                }
                active.erase(itr);
            }
        }
    }
    if (ans_id != -1) cout << ans_id << "\n";
  }

}