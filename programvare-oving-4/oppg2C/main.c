#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *replace(char text[], char newText[]) {
  int i = 0;
  int location = i;
  int offset = 0;
  //Initialize to not get garbage
  while (text[i] != '\0') {
    location = i + offset;

    char letter = text[i];
    if (letter == '&') {
      newText[location] = '&';
      newText[location + 1] = 'a';
      newText[location + 2] = 'm';
      newText[location + 3] = 'p';
      newText[location + 4] = ';';
      offset += 4;
    } else if (letter == '<') {
      newText[location] = '&';
      newText[location + 1] = 'l';
      newText[location + 2] = 't';
      newText[location + 3] = ';';
      offset += 3;
    } else if (letter == '>') {
      newText[location] = '&';
      newText[location + 1] = 'g';
      newText[location + 2] = 't';
      newText[location + 3] = ';';
      offset += 3;
    } else {
      newText[location] = text[i];
    }
    i++;
  }

  return newText;
}

int main() {
  char text[1000] = "Hello World! & < >"; //Initialize to not get garbage
  printf("Text to be changed: %s\n", text);

  char newText[1000] = {0};
  printf("%s\n", replace(text, newText));

  return 0;
}
