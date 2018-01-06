//
//  surface.cpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/06.
//

#include <list>
#include <gal/surface.hpp>

void gal::Surface::makeHalfedge(const std::vector<std::uint32_t> &indices)
{
    m_faces.resize(indices.size()/3);
    m_halfedges.resize(indices.size());
    m_vertice.resize(m_vertices.size());
    
    for (std::uint32_t i = 0; i < m_vertices.size()/3; ++i)
    {
        m_vertice[i].pLocation = &m_vertices[i*3];
    }
    
    for (std::uint32_t i = 0; i < indices.size()/3; ++i)
    {
        m_halfedges[i+0].pBegin = &m_vertice[indices[i+0]];
        m_halfedges[i+1].pBegin = &m_vertice[indices[i+1]];
        m_halfedges[i+2].pBegin = &m_vertice[indices[i+2]];
        
        m_halfedges[i+0].pNext = &m_halfedges[i+1];
        m_halfedges[i+1].pNext = &m_halfedges[i+2];
        m_halfedges[i+2].pNext = &m_halfedges[i+0];
        
        m_halfedges[i+0].pPrevious = &m_halfedges[i+2];
        m_halfedges[i+1].pPrevious = &m_halfedges[i+0];
        m_halfedges[i+2].pPrevious = &m_halfedges[i+1];
        
        m_faces[i].pHalfedge = &m_halfedges[i+0];
    }
    
    // ペアを探す
    std::list<gal::Halfedge*> target;
    std::list<gal::Halfedge*> pair;
    
    target.resize(m_halfedges.size());
    pair.resize(m_halfedges.size());

    std::transform(std::begin(m_halfedges), std::end(m_halfedges)
                   , std::begin(target)
                   , [](gal::Halfedge& halfedge){ return &halfedge; });
    std::copy(std::begin(target), std::end(target), std::begin(pair));
    
    for (const auto& pTarget : target)
    {
        auto result = std::find_if(std::begin(pair), std::end(pair)
                     , [&](gal::Halfedge* p1){ return p1 == pTarget; });
        pTarget->pPair = *result;
        (*result)->pPair = pTarget;
        pair.erase(result);
    }
}
