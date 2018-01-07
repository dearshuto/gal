//
//  surface.hpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/01.
//

#ifndef surface_hpp
#define surface_hpp

#include <vector>
#include <gal/util/vector.hpp>
#include "half_edge.hpp"

namespace gal
{
namespace ds
{
 
    class Surface
    {
    protected:
        Surface() = default;
    public:
        virtual~Surface() = default;
        
        std::vector<float>& getVertices(){ return m_vertexData; }
        
        const std::vector<Halfedge>& getHalfedges()const{ return m_halfedges; };
        
        const std::vector<Face>& getFaces()const{ return m_faces; }
        
        const std::vector<Vertex>& getVertices()const{ return m_vertices; }
        
    protected:
        
        void makeHalfedge(const std::vector<float>& vertexData, std::vector<std::uint32_t>& indexData);
        
    private:
        std::vector<Halfedge> m_halfedges;
        std::vector<Face> m_faces;
        std::vector<Vertex> m_vertices;
        std::vector<float> m_vertexData;
    };

}
}

#endif /* surface_hpp */
