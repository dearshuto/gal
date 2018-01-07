//
//  half_edge.h
//  GeometryAlgorithmLibrary
//
//  Created by Shuto Shikama on 2018/01/07.
//

#ifndef half_edge_h
#define half_edge_h

#include <gal/util/vector.hpp>

namespace gal{
namespace ds{

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

}
}


#endif /* half_edge_h */
