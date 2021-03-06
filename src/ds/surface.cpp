//
//  surface.cpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/06.
//

#include <list>
#include <gal/ds//surface.hpp>

void gal::ds::Surface::makeHalfedge(const std::vector<float> &vertexData, std::vector<std::uint32_t> &indices)
{
    m_vertexData = vertexData;
    m_faces.resize(indices.size()/3);
    m_halfedges.resize(indices.size());
    m_vertices.resize(vertexData.size()/3);
    
    // 頂点情報のセット
    for (std::uint32_t i = 0; i < m_vertexData.size()/3; ++i)
    {
        // xi へのポインタをもたせます。
        
        // 3つ飛びにすると xi へのポインタになる。
        // ↓0          ↓1
        // x1, y1, z1, x2, y2, z2, x3, y3, z3, ...
        
        m_vertices[i].pLocation = &m_vertexData[i*3];
    }
    
    // 3角ポリゴンを前提にしているので、3つとびにアクセスしています。
    for (std::uint32_t i = 0; i < indices.size()/3; ++i)
    {
        // ハーフエッジの始点の設定。
        m_halfedges[i*3 + 0].pBegin = &m_vertices[indices[i*3 + 0]];
        m_vertices[indices[i*3 + 0]].pHalfedge = &m_halfedges[i*3 + 0];
        
        m_halfedges[i*3 + 1].pBegin = &m_vertices[indices[i*3 + 1]];
        m_vertices[indices[i*3 + 1]].pHalfedge = &m_halfedges[i*3 + 1];
        
        m_halfedges[i*3 + 2].pBegin = &m_vertices[indices[i*3 + 2]];
        m_vertices[indices[i*3 + 2]].pHalfedge = &m_halfedges[i*3 + 2];
        
        // ハーフエッジ同士の接続
        m_halfedges[i*3 + 0].pNext = &m_halfedges[i*3 + 1];
        m_halfedges[i*3 + 1].pNext = &m_halfedges[i*3 + 2];
        m_halfedges[i*3 + 2].pNext = &m_halfedges[i*3 + 0];
        
        // ハーフエッジ同士の接続
        m_halfedges[i*3 + 0].pPrevious = &m_halfedges[i*3 + 2];
        m_halfedges[i*3 + 1].pPrevious = &m_halfedges[i*3 + 0];
        m_halfedges[i*3 + 2].pPrevious = &m_halfedges[i*3 + 1];
        
        // ハーフエッジが所属する面の設定
        m_faces[i].pHalfedge = &m_halfedges[i*3+0];
    }
    
    // ペアを探す
    // 総当たりでもいいですが、
    // ここではいちどリストにコピーして、みつかるたびにリストから外して、
    // 重複がないようにしています。
    // いちどリストにコピーするコストと、全頂点で毎回総当たりするコスト
    // どっちが大きいのかはわからないです。
    std::list<gal::ds::Halfedge*> target;
    std::list<gal::ds::Halfedge*> pair;

    target.resize(m_halfedges.size());
    pair.resize(m_halfedges.size());

    // リストにコピー。アドレスに変換します。
    std::transform(std::begin(m_halfedges), std::end(m_halfedges)
                   , std::begin(target)
                   , [](gal::ds::Halfedge& halfedge){ return &halfedge; });
    
    // 初期状態では、ターゲットとペア候補はすべてのエッジが入っています
    std::copy(std::begin(target), std::end(target), std::begin(pair));

    for (const auto& pTarget : target)
    {
        auto result = std::find_if(std::begin(pair), std::end(pair)
                                   , [&](gal::ds::Halfedge* pPair)
        {
            const float kMatchBegenEnd = (pTarget->pBegin == pPair->pNext->pBegin);
            const float kMatchEndBegin = (pTarget->pNext->pBegin == pPair->pBegin);

            return kMatchBegenEnd && kMatchEndBegin;
        });

        
        if (result == std::end(pair))
        {
            // ペアが見つからないときはなにもしない
            continue;
        }
        else
        {
            // ペアがみつかったらそれぞれ設定して、pair リストから除外
            gal::ds::Halfedge* pResult = *result;
            pTarget->pPair = pResult;
            pResult->pPair = pTarget;
            pair.erase(result);
        }
    }
}
