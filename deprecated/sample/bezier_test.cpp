//
//  bezier_test.cpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/07.
//  Copyright © 2018年 Shuto Shikama. All rights reserved.
//

#include <fstream>
#include <gal/prim/bezier_surface.hpp>
#include <gal/util/obj_serializer.hpp>

//! @brief ベジエ曲面を書き出すサンプル兼テストです。
//! @detail
//! ベジエ曲面を作成して Wavefront OBJ 形式で書き出します。
//! 書き出したモデルを一般的なモデルビューワで読み込むとティーポットが表示されます。
int main(int argc, const char * argv[])
{
    gal::prim::BezierSurface bezier;
    bezier.getControllPoint(0) = {0.0f, 0.0f, 0.0f};
    bezier.getControllPoint(1) = {0.33f, 0.0f, 0.2f};
    bezier.getControllPoint(2) = {0.66f, 0.0f, 0.4f};
    bezier.getControllPoint(3) = {1.0f, 0.0f, 0.0f};
    
    bezier.getControllPoint(4) = {0.0f, 0.33f, 0.0f};
    bezier.getControllPoint(5) = {0.33f, 0.33f, 0.3f};
    bezier.getControllPoint(6) = {0.66f, 0.33f, 0.3f};
    bezier.getControllPoint(7) = {1.0f, 0.33f, 0.0f};
    
    bezier.getControllPoint(8) = {0.0f, 0.66f, 0.0f};
    bezier.getControllPoint(9) = {0.33f, 0.66f, -0.3f};
    bezier.getControllPoint(10) = {0.66f, 0.66f, -0.3f};
    bezier.getControllPoint(11) = {1.0f, 0.66f, 0.0f};
    
    bezier.getControllPoint(12) = {0.0f, 1.0f, 0.0f};
    bezier.getControllPoint(13) = {0.33f, 1.0f, 0.0f};
    bezier.getControllPoint(14) = {0.66f, 1.0f, 0.0f};
    bezier.getControllPoint(15) = {1.0f, 1.0f, 0.0f};

    bezier.update(10, 10);
    gal::util::SaveObjToFile(bezier, "bezier_10x10.obj");

    return 0;
}
