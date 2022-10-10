TARGET=.
OBJ=./obj
FLAGS=-Wall -Wextra -pedantic -std=c++2a -g -Iinc -O3 #-D_DEBUG

__start__: $(TARGET)/cpm
	$(TARGET)/cpm

$(TARGET)/cpm: $(OBJ)/main.o 
	g++ -o $(TARGET)/cpm $(OBJ)/main.o

$(OBJ):
	mkdir -p $(OBJ)

$(OBJ)/main.o: main.cpp
	g++ -c $(FLAGS) -o $(OBJ)/main.o main.cpp

clean:
	rm -f $(OBJ)/*.o
	rm -f $(TARGET)/cpm