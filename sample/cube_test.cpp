//
//  cube_test.cpp
//  geometric_algorithm_library
//
//  Created by Shuto Shikama on 2018/01/01.
//  Copyright © 2018年 Shuto Shikama. All rights reserved.
//

#include <fstream>
#include <gal/prim/cube.hpp>

//! @brief 球を生成して書き出すサンプル兼テストです。
//! @detail
//! 球を作成して Wavefront OBJ 形式で書き出します。
//! 書き出したモデルを一般的なモデルビューワで読み込むと球が表示されます。
int main(int argc, const char * argv[])
{
    // 球を 50x50 に分割して生成します。
    gal::Cube cube;
    
    std::ofstream obj{"cube.obj"};
    
    // 頂点情報の書き出し
    auto& vertexData = cube.getVertices();
    for (std::uint32_t i = 0; i < vertexData.size(); i+=3)
    {
        obj << "v " << vertexData[i] << " " << vertexData[i+1] << " " << vertexData[i+2] << std::endl;
    }
    
    // 面情報の書き出し
    auto& indicesData = cube.getIndices();
    for (std::uint32_t i = 0; i < indicesData.size(); i+=3)
    {
        obj << "f " << indicesData[i]+1 << " " << indicesData[i+1]+1 << " " << indicesData[i+2]+1 << std::endl;
    }
    
    return 0;
}
