//
//  util_Functional.h
//  Geometry Algorithm Library
//
//  Created by Shuto Shikama on 2018/01/01.
//

#ifndef util_Functional_h
#define util_Functional_h

#include <cstdint>

namespace gal
{
    //! @brief 階乗を計算します。
    template<std::uint32_t N>
    struct Factorial
    {
        enum { value = N * Factorial<N-1>::value };
    };

    template<>
    struct Factorial<0>
    {
        enum { value = 1 };
    };

    template<>
    struct Factorial<1>
    {
        enum { value = 1 };
    };
    
    //! @brief 階乗を計算します。
    std::uint64_t ComputeFactorial(const std::uint32_t n)
    {
        return (n == 0 || n == 1) ? 1 : n * ComputeFactorial(n - 1);
    }
}

#endif /* util_Functional_h */
