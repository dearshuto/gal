//
//  bezier_surface.cpp
//  gal
//
//  Created by Shuto Shikama on 2016/12/06.
//  Copyright © 2016年 Shuto Shikama. All rights reserved.
//

#include <cassert>
#include <cmath>
#include <functional>
#include <gal/util/factorial.h>
#include <gal/prim/bezier_surface.hpp>

void gal::prim::BezierSurface::update(const std::uint32_t div, std::uint32_t subDiv)
{
    // ベジエ曲面上の UV 座標を表す構造体。
    // UV なので 値は [0, 1] の範囲です。
    struct BezierCoordinate
    {
        float U{0.0f};
        float V{0.0f};
    };
    
    std::vector<float> vertices;
    std::vector<std::uint32_t> indices;
    
    // たてよこの頂点数を取得。
    // 両端に頂点があるので +2 してます。
    const std::uint64_t kN = div + 1;
    const std::uint64_t kM = subDiv + 1;
    
    // 各頂点の UV 座標
    // i 番目の要素がが m_vertices[i] の UV を表します
    // UV はこの関数内でしか使わないので、頂点と UV で別変数を管理してます。
    std::vector<BezierCoordinate> uv;
    
    // 頂点数分の領域を確保
    uv.resize(kN * kM);
    vertices.resize(kN * kM * 3);
    
    // 足しこみを行うので 全ての要素を 0 で初期化しておく
    std::fill(std::begin(vertices), std::end(vertices), 0.0f);
    
    // ベジエ曲面上で隣り合う頂点の距離
    // 等分割を前提にしているので、すべて同じ感覚で並んでます。
    const float kStrideU = 1.0f / div;
    const float kStrideV = 1.0f / subDiv;
    
    // 求める頂点の UV 座標を算出。
    for (std::uint32_t n = 0; n < kN; ++n) {
        for (std::uint32_t m = 0; m < kM; ++m)
        {
            uv[n + kN*m].U = kStrideU * static_cast<float>(n);
            uv[n + kN*m].V = kStrideV * static_cast<float>(m);
        }
    }
    
    // 全頂点に対してベジエ曲面上の座標を算出
    for (std::uint32_t n = 0; n < kN; ++n) {
        for (std::uint32_t m = 0; m < kM; ++m)
        {
            const float kU = uv[n + kN*m].U;
            const float kV = uv[n + kN*m].V;
            const std::size_t kIndex = (n+m*kN) * 3;
            float* position = &vertices[kIndex];
            
            // ずべての制御点からの影響を足しこむ
            for (int i = 0; i < 4; ++i) {
                for (int j = 0; j < 4; ++j)
                {
                    const gal::util::Vector& kControlPoint = getControllPoint(i + 4*j);
                    const float kBu = ComputeBernsteinPolynormal(kU, i);
                    const float kBv = ComputeBernsteinPolynormal(kV, j);
                    const gal::util::Vector kResult =kBu * kBv * kControlPoint;
                    position[0] += kResult.X;
                    position[1] += kResult.Y;
                    position[2] += kResult.Z;
                }
            }
        }
    }
    
    // 三角ポリゴンとして登録。
    for (std::uint32_t n = 0; n < kN-1; ++n){
        for (std::uint32_t m = 0; m <kM-1; ++m)
        {
            const int kLeftUp = n + m*kN;
            indices.push_back(kLeftUp);
            indices.push_back(kLeftUp + kN + 1);
            indices.push_back(kLeftUp + kN);
            
            indices.push_back(kLeftUp);
            indices.push_back(kLeftUp+1);
            indices.push_back(kLeftUp + kN + 1);
        }
    }
    
    makeHalfedge(vertices, indices);
}

float gal::prim::BezierSurface::ComputeBernsteinPolynormal(const float t, std::uint32_t i )
{
    assert(0 <= t && t <= 1);
    assert(0 <= i && i <= 3); // 3次なので
    
    // 3 次バーンスタイン多項式
    constexpr std::uint64_t n = 3; // 3次なので
    
    // 二項係数
    const float kBinomialCoefficients = gal::Factorial<n>::value / (gal::ComputeFactorial(i) *gal::ComputeFactorial(n-i));
    
    return kBinomialCoefficients * std::pow(t, i) * std::pow<float>(1.0f-t, n-i);
}
