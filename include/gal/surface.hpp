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
    struct Face;
    struct Halfedge;
    struct Vertex;
    
    struct Face
    {
        Halfedge* pHalfedge{nullptr};
    };
    
    struct Halfedge
    {
        Vertex* pBegin{nullptr};
        Face* pFace{nullptr};
        Halfedge* pPair{nullptr};
        Halfedge* pNext{nullptr};
        Halfedge* pPrevious{nullptr};
    };
    
    struct Vertex
    {
        gal::Vector Position;
        float* pLocation{nullptr};
        Halfedge* pHalfedge{nullptr};
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
        std::vector<Halfedge> m_halfedges;
        std::vector<Face> m_faces;
        std::vector<Vertex> m_vertice;
        
        std::vector<float> m_vertices;
        std::vector<std::uint32_t> m_indices;
    };
}

#endif /* surface_hpp */
