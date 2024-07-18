
#include <stdint.h>
#include <stdio.h>
#include <iostream>

#include <QtWidgets/QApplication>
#include <QtWidgets/QPushButton>

extern "C" void hello() {
  printf("hello from cpp!!!\n");

  int argc = 0;
  char hi = 'c';
  char * two = &hi;

  QApplication app (argc, &two);

  QPushButton button;
  button.setText("My text");
  button.setToolTip("A tooltip");
  button.show();

  int result = app.exec();

  printf("Qapp exited with %d\n", result);

}
