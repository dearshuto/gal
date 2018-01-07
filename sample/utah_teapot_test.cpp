//
//  utah_teapot_test.cpp
//  geometric_algorithm_library
//
//  Created by Shuto Shikama on 2018/01/01.
//  Copyright © 2018年 Shuto Shikama. All rights reserved.
//

#include <fstream>
#include <gal/prim/utah_teapot.hpp>
#include <gal/util/obj_serializer.hpp>

//! @brief ユタ・ティーポット を書き出すサンプル兼テストです。
//! @detail
//! ユタ・ティーポット を作成して Wavefront OBJ 形式で書き出します。
//! 書き出したモデルを一般的なモデルビューワで読み込むとティーポットが表示されます。
int main(int argc, const char * argv[])
{
    // ユタ・ティーポットの各ベジエ曲面を 10x10 に分割して生成します。
    gal::UtahTeapot utahTeapot;
    utahTeapot.update(10, 10);
    
    gal::util::SaveObjToFile(utahTeapot, "utah_teapot_10x10.obj");
    
    return 0;
}
