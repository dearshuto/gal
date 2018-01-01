//
//  surface.hpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/01.
//

#ifndef surface_hpp
#define surface_hpp

#include <vector>
#include "util/vector.hpp"

namespace gal
{
    struct Vertex
    {
        gal::Vector Position;
        gal::Vector Normal;
    };
    
    class Surface
    {
    protected:
        Surface() = default;
    public:
        virtual~Surface() = default;
        
        std::vector<float>& getVertices(){ return m_vertices; }
        
        std::vector<std::uint32_t>& getIndices(){ return m_indices; }
        
    protected:
        std::vector<float> m_vertices;
        std::vector<std::uint32_t> m_indices;
    };
}

#endif /* surface_hpp */
