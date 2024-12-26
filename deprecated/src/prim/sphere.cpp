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

gal::prim::Sphere::Sphere()
{
    update(20, 20);
}

void gal::prim::Sphere::update(const std::uint32_t div, const std::uint32_t subDiv)
{
    assert((3 <= div) && (3 <= subDiv));
    
    const float kRadius = 0.5f;
    const float kRadian = 2.0f * M_PI / static_cast<float>(div);
    const float kRradianZ = M_PI / static_cast<float>(subDiv);
    std::vector<float> vertices;
    vertices.reserve(div * (subDiv-1) + 2/*北極と南極の分*/);
    
    for (std::uint32_t m = 1; m < subDiv; ++m)
    {
        const float kZ = kRadius * std::cos(m * kRradianZ);
        const float kLocalRadius = kRadius * std::sin(m * kRradianZ);
        
        for (std::uint32_t n = 0; n < div; ++n)
        {
            const float kX = kLocalRadius * std::cos(static_cast<float>(n) * kRadian);
            const float kY = kLocalRadius * std::sin(static_cast<float>(n) * kRadian);
            
            vertices.push_back(kX);
            vertices.push_back(kY);
            vertices.push_back(kZ);
        }
    }

    std::vector<std::uint32_t> indices(0);
    for (std::uint32_t n = 0; n < div-1; ++n){
        for (std::uint32_t m = 0; m < subDiv-2; ++m)
        {
            const int kLeftUp = n + m*div;
            const int kNextRow = div;
            const int kNextColumn = 1;
            indices.push_back(kLeftUp);
            indices.push_back(kLeftUp + kNextRow);
            indices.push_back(kLeftUp + kNextRow + kNextColumn);
            
            indices.push_back(kLeftUp);
            indices.push_back(kLeftUp + kNextRow + kNextColumn);
            indices.push_back(kLeftUp + kNextColumn);
        }
    }
    
    // 周回
    for (std::uint32_t m = 0; m < subDiv-2; ++m)
    {
        indices.push_back(m * div);
        indices.push_back((m+1)*div - 1);
        indices.push_back((m+1)*div);
        
        indices.push_back((m+1) * div -1);
        indices.push_back((m+2) * div -1);
        indices.push_back((m+1) * div);
    }
    
    // 北極
    vertices.push_back(0.0f);
    vertices.push_back(0.0f);
    vertices.push_back(kRadius);

    for (std::uint32_t n = 0; n < div-1; ++n)
    {
        indices.push_back(vertices.size()/3-1);
        indices.push_back(n);
        indices.push_back(n+1);
    }
    indices.push_back(vertices.size()/3-1);
    indices.push_back(div-2+1);
    indices.push_back(0);

    // 南極
    vertices.push_back(0.0f);
    vertices.push_back(0.0f);
    vertices.push_back(-kRadius);
    for (std::uint32_t n = 0; n < div-1; ++n)
    {
        indices.push_back(vertices.size()/3 - 1);
        indices.push_back(vertices.size()/3 - 3 - n);
        indices.push_back(vertices.size()/3 - 4 - n);
    }
    
    indices.push_back(vertices.size()/3 - 1);
    indices.push_back(vertices.size()/3 - 4 - (div-2));
    indices.push_back(vertices.size()/3 - 3 - 0);
    
    makeHalfedge(vertices, indices);
}
