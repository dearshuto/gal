//
//  sphere.hpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/04.
//

#ifndef sphere_hpp
#define sphere_hpp

#include <cstdint>
#include <gal/surface.hpp>

namespace gal
{
    class Sphere : public gal::Surface
    {
    public:
        Sphere() = default;
        ~Sphere() = default;
        
        //! @brief 指定された分割数で球体のメッシュを生成します。
        //! @param[in] div z 軸周りの分割数
        //! @param[in] subDiv z 軸方向の分割数
        //! @pre 3 <= div, 3 <= subDiv, これより分割数を減らすと体積が 0 になってしまうためです。
        void update(const std::uint32_t div, const std::uint32_t subDiv);
    };
}

#endif /* sphere_hpp */
