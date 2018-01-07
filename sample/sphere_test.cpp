//
//  utah_teapot_test.cpp
//  geometric_algorithm_library
//
//  Created by Shuto Shikama on 2018/01/01.
//  Copyright © 2018年 Shuto Shikama. All rights reserved.
//

#include <fstream>
#include <gal/prim/sphere.hpp>
#include <gal/util/obj_serializer.hpp>

//! @brief 球を生成して書き出すサンプル兼テストです。
//! @detail
//! 球を作成して Wavefront OBJ 形式で書き出します。
//! 書き出したモデルを一般的なモデルビューワで読み込むと球が表示されます。
int main(int argc, const char * argv[])
{
    // 球を 50x50 に分割して生成します。
    gal::Sphere sphere;
    sphere.update(50 , 50);
    
    gal::util::SaveObjToFile(sphere, "sphere_50x50.obj");
    
    return 0;
}
