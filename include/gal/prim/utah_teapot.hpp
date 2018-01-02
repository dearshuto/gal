//
//  utah_teapot.hpp
//  gal
//
//  Created by Shuto Shikama on 2016/12/02.
//  Copyright © 2016年 Shuto Shikama. All rights reserved.
//

#ifndef utah_teapot_hpp
#define utah_teapot_hpp

#include <string>
#include <vector>
#include <gal/surface.hpp>

namespace gal
{
    //! @brief 3角ポリゴンで構成されるユタティーポッドのメッシュです。
    //! @detail
    //! 注ぎ口が x 軸, ふたが z 軸方向のモデルが生成されます。
    //! @note
    //! update() 関数を少なくとも 1 回読んでください。
    //! 呼ばないと有効なメッシュが構成されません。
    class UtahTeapot : public gal::Surface
    {
    public:
        UtahTeapot() = default;
        ~UtahTeapot() = default;
        
        //! @brief 指定した分割数でメッシュを構築します。
        //! @detail Martin Newell がモデリングしたベジエ曲面の制御点からメッシュを抽出しています。
        void update(const std::uint64_t div, const std::uint64_t subDiv);        
    };

}

#endif /* utah_teapot_hpp */
