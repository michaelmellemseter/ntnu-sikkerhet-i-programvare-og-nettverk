cmake_minimum_required(VERSION 2.8)

project(opg2C++)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++1y -Wall -Wextra")

add_executable(opg2C++ main.cpp)
