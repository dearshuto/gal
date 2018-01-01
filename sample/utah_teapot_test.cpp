//
//  utah_teapot_test.cpp
//  geometric_algorithm_library
//
//  Created by Shuto Shikama on 2018/01/01.
//  Copyright © 2018年 Shuto Shikama. All rights reserved.
//

#include <fstream>
#include <gal/prim/utah_teapot.hpp>

//! @brief ユタ・ティーポット を書き出すサンプル兼テストです。
//! @detail
//! ユタ・ティーポット を作成して Wavefront OBJ 形式で書き出します。
//! 書き出したモデルを一般的なモデルビューワで読み込むとティーポットが表示されます。
int main(int argc, const char * argv[])
{
    // ユタ・ティーポットの各ベジエ曲面を 10x10 に分割して生成します。
    gal::UtahTeapot utahTeapot;
    utahTeapot.update(10, 10);
    
    // テキストで書き出します。
    std::ofstream obj{"utah_teapot_10x10.obj"};
    
    // 頂点情報の書き出し
    auto& vertexData = utahTeapot.getVertices();
    for (std::uint32_t i = 0; i < vertexData.size(); i+=3)
    {
        obj << "v " << vertexData[i] << " " << vertexData[i+1] << " " << vertexData[i+2]*1.25 << std::endl;
    }
    
    // 面情報の書き出し
    auto& indicesData = utahTeapot.getIndices();
    for (std::uint32_t i = 0; i < indicesData.size(); i+=4)
    {
        obj << "f " << indicesData[i]+1 << " " << indicesData[i+1]+1 << " " << indicesData[i+2]+1 << " " << indicesData[i+3]+1 << std::endl;
    }
    
    return 0;
}
