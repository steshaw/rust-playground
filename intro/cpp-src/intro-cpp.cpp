#include <stdio.h>

int by_ref(int& i) {
  return i + 1;
}

void use_by_ref() {
  int i = 10;
  int r1 = by_ref(i);
  int i41 = 41;
  int r2 = by_ref(i41);
  printf("by_refs %d %d\n", r1, r2);
}

int main() {
  use_by_ref();
}
