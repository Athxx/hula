

class Time {
  secondToTime (int sec) {
    return DateTime.fromMicrosecondsSinceEpoch(sec * 1000);
  }
}
