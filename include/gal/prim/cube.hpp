//
//  cube.hpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/06.
//

#ifndef cube_hpp
#define cube_hpp

#include <gal/surface.hpp>

namespace gal
{
    //! @brief 中心が原点で1辺の長さが1のキューブ
    class Cube : public gal::Surface
    {
    public:
        Cube();
        ~Cube() = default;
        
    private:
        
        //! @brief コンストラクタのなかで使用されます。
        void initialize();
    };
}

#endif /* cube_hpp */
