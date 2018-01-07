//
//  factorial_sample.cpp
//  geometric_algorithm_library
//
//  Created by Shuto Shikama on 2018/01/01.
//  Copyright © 2018年 Shuto Shikama. All rights reserved.
//

#include <iostream>
#include <gal/util/factorial.h>

//! @brief 階乗計算のサンプル兼テストです。
int main(int argc, const char * argv[])
{
    // コンパイル時計算のサンプル
    std::cout << "template" << std::endl;
    std::cout << gal::Factorial<0>::value<< std::endl;
    std::cout << gal::Factorial<1>::value<< std::endl;
    std::cout << gal::Factorial<2>::value<< std::endl;
    std::cout << gal::Factorial<3>::value<< std::endl;
    std::cout << gal::Factorial<4>::value<< std::endl;
    std::cout << gal::Factorial<5>::value<< std::endl;
    
    // 実行時計算のサンプル
    std::cout << "standard" << std::endl;
    std::cout << gal::ComputeFactorial(0) << std::endl;
    std::cout << gal::ComputeFactorial(1) << std::endl;
    std::cout << gal::ComputeFactorial(2) << std::endl;
    std::cout << gal::ComputeFactorial(3) << std::endl;
    std::cout << gal::ComputeFactorial(4) << std::endl;
    std::cout << gal::ComputeFactorial(5) << std::endl;
    
    return 0;
}
