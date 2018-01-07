//
//  cube_test.cpp
//  geometric_algorithm_library
//
//  Created by Shuto Shikama on 2018/01/01.
//  Copyright © 2018年 Shuto Shikama. All rights reserved.
//

#include <iostream>
#include <fstream>
#include <gal/prim/cube.hpp>
#include <gal/util/obj_serializer.hpp>

//! @brief 球を生成して書き出すサンプル兼テストです。
//! @detail
//! 球を作成して Wavefront OBJ 形式で書き出します。
//! 書き出したモデルを一般的なモデルビューワで読み込むと球が表示されます。
int main(int argc, const char * argv[])
{
    // 球を 50x50 に分割して生成します。
    gal::prim::Cube cube;
    
    gal::util::SaveObjToFile(cube, "cube.obj");
    
    return 0;
}
