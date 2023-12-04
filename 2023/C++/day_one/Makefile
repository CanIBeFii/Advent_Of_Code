NAME = day_one

HEADERS_LIST = day_one.hpp
HEADERS_DIR = ./includes/
HEADERS = $(addprefix $(HEADERS_DIR), $(HEADERS_LIST))

SOURCES_LIST = day_one.cpp
SOURCES_DIR = ./src/
SOURCES = $(addprefix $(SOURCES_DIR), $(SOURCES_LIST))

CC = g++
FLAGS = -Wall -Wextra -Werror -g3


all: clean $(NAME)

$(NAME): 
	$(CC) $(FLAGS) $(SOURCES) -o $(NAME)

clean:
	rm -rf $(NAME)

run: $(NAME)
	./$(NAME) ./input.txt

test: $(NAME)
	./$(NAME) ./test.txt