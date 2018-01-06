//
//  sphere.cpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/04.
//

#include <cassert>
#include <cmath>
#include <math.h>
#include <gal/prim/sphere.hpp>

void gal::Sphere::update(const std::uint32_t div, const std::uint32_t subDiv)
{
    assert((3 <= div) && (3 <= subDiv));
    
    const float kRadius = 0.5f;
    const float kRadian = 2.0f * M_PI / static_cast<float>(div);
    const float kRradianZ = M_PI / static_cast<float>(subDiv);
    
    m_vertice.reserve(div * (subDiv-1) + 2/*北極と南極の分*/);
    m_vertices.reserve(div * (subDiv-1) + 2/*北極と南極の分*/);
    
    for (std::uint32_t m = 1; m < subDiv; ++m)
    {
        const float kZ = kRadius * std::cos(m * kRradianZ);
        const float kLocalRadius = kRadius * std::sin(m * kRradianZ);
        
        for (std::uint32_t n = 0; n < div; ++n)
        {
            const float kX = kLocalRadius * std::cos(static_cast<float>(n) * kRadian);
            const float kY = kLocalRadius * std::sin(static_cast<float>(n) * kRadian);
            
            m_vertices.push_back(kX);
            m_vertices.push_back(kY);
            m_vertices.push_back(kZ);
        }
    }

    for (std::uint32_t n = 0; n < div-1; ++n){
        for (std::uint32_t m = 0; m < subDiv-2; ++m)
        {
            const int kLeftUp = n + m*div;
            const int kNextRow = div;
            const int kNextColumn = 1;
            m_indices.push_back(kLeftUp);
            m_indices.push_back(kLeftUp + kNextRow);
            m_indices.push_back(kLeftUp + kNextRow + kNextColumn);
            
            m_indices.push_back(kLeftUp);
            m_indices.push_back(kLeftUp + kNextRow + kNextColumn);
            m_indices.push_back(kLeftUp + kNextColumn);
        }
    }
    
    // 周回
    for (std::uint32_t m = 0; m < subDiv-2; ++m)
    {
        m_indices.push_back(m * div);
        m_indices.push_back((m+1)*div - 1);
        m_indices.push_back((m+1)*div);
        
        m_indices.push_back((m+1) * div -1);
        m_indices.push_back((m+2) * div -1);
        m_indices.push_back((m+1) * div);
    }
    
    // 北極
    m_vertices.push_back(0.0f);
    m_vertices.push_back(0.0f);
    m_vertices.push_back(kRadius);

    for (std::uint32_t n = 0; n < div-1; ++n)
    {
        m_indices.push_back(m_vertices.size()/3-1);
        m_indices.push_back(n);
        m_indices.push_back(n+1);
    }
    m_indices.push_back(m_vertices.size()/3-1);
    m_indices.push_back(div-2+1);
    m_indices.push_back(0);

    // 南極
    m_vertices.push_back(0.0f);
    m_vertices.push_back(0.0f);
    m_vertices.push_back(-kRadius);
    for (std::uint32_t n = 0; n < div-1; ++n)
    {
        m_indices.push_back(m_vertices.size()/3 - 1);
        m_indices.push_back(m_vertices.size()/3 - 3 - n);
        m_indices.push_back(m_vertices.size()/3 - 4 - n);
    }
    
    m_indices.push_back(m_vertices.size()/3 - 1);
    m_indices.push_back(m_vertices.size()/3 - 4 - (div-2));
    m_indices.push_back(m_vertices.size()/3 - 3 - 0);
    
    makeHalfedge(m_indices);
}
