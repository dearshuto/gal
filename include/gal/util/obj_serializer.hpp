//
//  obj_serializer.hpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/07.
//

#ifndef obj_serializer_hpp
#define obj_serializer_hpp

#include <string>

namespace gal {
namespace ds
{
    class Surface;
}
    
namespace util
{
 
    //! @brief サーフェイスを Wavefront Obj 形式で書き出します。
    //! @param[in] surface 書き出すサーフェイス
    //! @param[in] filename 出力するファイル名
    //! @return ファイルの書き出しに成功したら true, それ以外は false
    bool SaveObjToFile(const gal::ds::Surface& surface, const std::string& filename);
    
}
}

#endif /* obj_serializer_hpp */
