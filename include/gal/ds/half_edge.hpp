//  gal
//! @file half_edge.h
//! @brief ハーフエッジ構造のための構造体を提供します。
//! @author Shuto Shikama
//! @date 2018/01/07.
//

#ifndef half_edge_h
#define half_edge_h

#include <gal/util/vector.hpp>

namespace gal{
namespace ds{

struct Face;
struct Halfedge;
struct Vertex;

    //! @brief 3角ポリゴン
    //! @detail
    //! pHalfedge を3つたどると自分自身に帰ってきます。
    struct Face
    {
        //! この面を構成するいずれかのハーフエッジへのポインタ。
        //! @note pHalfedge->pNext->pNext->pNext == pHalfedge が成立します。
        Halfedge* pHalfedge{nullptr};
    };
    
    //! @brief 始点情報をもつハーフエッジ。
    struct Halfedge
    {
        //! @brief 始点へのポインタ。
        Vertex* pBegin{nullptr};
        
        //! @brief 所属する面へのポインタ。
        Face* pFace{nullptr};
        
        //! @brief 対になるエッジ。存在しない場合は nullptr.
        Halfedge* pPair{nullptr};
        
        //! @brief 終点に接続しているハーフエッジへのポインタ。
        Halfedge* pNext{nullptr};
        
        //! @brief 自身の始点を終点とするハーフエッジへのポインタ。
        Halfedge* pPrevious{nullptr};
    };
    
    //! @brief ハーフエッジ構造の頂点
    struct Vertex
    {
        //! @brief 3次元位置情報へのポインタ。
        //! @detail float[3] として使えます。
        float* pLocation{nullptr};
        
        //! @brief 自身を始点にもつハーフエッジのいずれかへのポインタ。
        Halfedge* pHalfedge{nullptr};
    };

}
}


#endif /* half_edge_h */
