//
//  obj_serializer.cpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/07.
//

#include <cmath>
#include <cstdint>
#include <fstream>
#include <gal/ds/surface.hpp>
#include <gal/util/obj_serializer.hpp>

bool gal::util::SaveObjToFile(const gal::ds::Surface &surface, const std::string &filename)
{
    std::ofstream obj{filename};
    
    if (obj.fail())
    {
        return false;
    }
    
    // 頂点情報の書き出し
    for (const auto& kVertex : surface.getVertices())
    {
        obj << "v " << kVertex.pLocation[0] << " " << kVertex.pLocation[1] << " " << kVertex.pLocation[2] << std::endl;
    }
    
    // 面情報の書き出し
    for (const auto& kFace : surface.getFaces())
    {
        gal::ds::Vertex* pV1 = kFace.pHalfedge->pBegin;
        gal::ds::Vertex* pV2 = kFace.pHalfedge->pNext->pBegin;
        gal::ds::Vertex* pV3 = kFace.pHalfedge->pNext->pNext->pBegin;
        
        // 1次元配列なので、先頭要素との引き算でインデクスを算出。
        // エンディアンを考慮して絶対値をとってます。
        std::uint32_t kIndex1 = std::abs(&surface.getVertices().front() - pV1);
        std::uint32_t kIndex2 = std::abs(&surface.getVertices().front() - pV2);
        std::uint32_t kIndex3 = std::abs(&surface.getVertices().front() - pV3);
        
        // Wavefront obj のファイルフォーマットは、1 始まりなので、+1してから出力
        obj << "f " << kIndex1+1 << " " << kIndex2+1 << " " << kIndex3+1 << std::endl;
    }
    
    return true;
}
