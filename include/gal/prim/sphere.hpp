//  gal
//! @file sphere.hpp
//! @brief 球を提供します。
//! @author Shuto Shikama
//! @date 2018/01/04.
//

#ifndef sphere_hpp
#define sphere_hpp

#include <cstdint>
#include <gal/ds//surface.hpp>

namespace gal
{
namespace prim
{
    //! @brief ハーフエッジ構造であらわされる球。
    //! @note 初期状態では 20x20 で生成されます。
    class Sphere : public gal::ds::Surface
    {
    public:
        Sphere();
        ~Sphere() = default;
        
        //! @brief 指定された分割数で球体のメッシュを生成します。
        //! @param[in] div z 軸周りの分割数
        //! @param[in] subDiv z 軸方向の分割数
        //! @pre 3 <= div, 3 <= subDiv, これより分割数を減らすと体積が 0 になってしまうためです。
        void update(const std::uint32_t div, const std::uint32_t subDiv);
    };

}
}

#endif /* sphere_hpp */
