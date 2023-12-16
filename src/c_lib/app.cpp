#include<iostream>
typedef struct Human
{
  int a;
  char name[10];
};

class Car {
public: 
  int a;
};

extern "C" void print_human(Human h) {
  printf("%d\n", h.a);  
  printf("%s\n", h.name);
}

extern "C" void print_car(Car c) {
  std::cout << "car:" << c.a << std::endl;
}