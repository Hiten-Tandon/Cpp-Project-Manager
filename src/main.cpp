#include <iostream>
#include <windows.h>

#include "../headers/misc.hpp"
#include "../headers/projectManager.hpp"
#include "../headers/conv.hpp"
#include "../headers/argCheck.hpp"

#include "create.cpp"
#include "projects.cpp"
#include "add.cpp"
using namespace arguments;

/*
 * CP
 *
 * This is CP's main file.
 * Although its not much, its responsible for the recursion between options 2, 3, and 4;
 * If any new files are created, you want to add a new option and also add it to the switch.
 */
int main(int argc, char *argv[]){
    if(argCheck("cp", argc, argv, 1, 0)){
        std::cout << cmdInformation::help << std::endl;
    }
    if(argCheck("create", argc, argv)){
        std::cout << "CP Create Called" << std::endl;
    }
    if(argCheck("projects", argc, argv)){
        std::cout << "CP Projects Called" << std::endl;
    }
    if(argCheck("add", argc, argv)){
        std::cout << "CP Add Called" << std::endl;
    }
    if(argCheck("help", argc, argv)){
        std::cout << cmdInformation::help << std::endl;
    }

    // int option; // switch option
    // std::cout << "Welcome to Creation Project. What would you like to do?: " << std::endl;
    // while(true){
    //     std::cout << cmdInformation::menu << std::endl;
    //     std::cin >> option; // input check. can be changed to only accept numbers and recursively run if a valid number is not given. 
    //     // todo: migrate all files here to main file
    //     // todo: create something similar to cargo's `cargo new` function
    //     switch(option){
    //         // case 1: createFolder(); break;
    //         // case 2: system("tree /f"); system("cp"); break;
    //         // case 3: removeFolder(); break; 
    //         // case 4: mainFunction(); break;
    //         // case 5: cpproject(); break;
    //         // case 6: std::cout << "Thank you for using CP File." << std::endl; return 0; break; // no default case
    //     }
    // }
    
}