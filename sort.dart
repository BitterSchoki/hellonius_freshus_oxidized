class Ingredient {
  int id;
  Ingredient(this.id);
}

List<Ingredient> crossAllergiesSort(List<Ingredient> ings, List<int> selectedIds) {
  List<List<int>> crossAllergies = [
    [18, 52], // milk + beef
    [19, 51], // egg + chicken
  ];

  List<(int, int)> scoreList = ings.map((i) {
    var matchingCross = crossAllergies.where((l) => l.contains(i.id)).toList();
    int score = 0;
    for (var c in matchingCross) {
      int factor = 20; // could also populate this from crossAllergies
      int cnt = c.map((id) => selectedIds.contains(id) ? 1 : 0).reduce((acc, v) => acc + v);
      score += factor * cnt;
    }
    return (i.id, score);
  }).toList();

  Map<int, int> scores = Map.fromIterable(scoreList, key: (v) => v[0], value: (v) => v[1]);

  ings.sort((a, b) => (scores[b.id] ?? 0).compareTo(scores[a.id] ?? 0));
  return ings;
}