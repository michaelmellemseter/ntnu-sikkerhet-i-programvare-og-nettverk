cmake_minimum_required(VERSION 2.8)

project(opg2C)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -std=c11 -Wall -Wextra")

add_executable(opg2C main.c)
