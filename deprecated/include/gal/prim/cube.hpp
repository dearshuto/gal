//  gal
//! @file cube.hpp
//! @brief キューブを提供します。
//! @author Shuto Shikama
//! @date 2018/01/06.
//

#ifndef cube_hpp
#define cube_hpp

#include <gal/ds//surface.hpp>

namespace gal
{
namespace prim
{
    //! @brief 中心が原点で1辺の長さが1のキューブ
    class Cube : public gal::ds::Surface
    {
    public:
        Cube();
        ~Cube() = default;
        
    private:
        
        //! @brief コンストラクタのなかで使用されます。
        void initialize();
    };
}
}

#endif /* cube_hpp */
