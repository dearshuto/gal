//  gal
//! @file surface.hpp
//! @brief ハーフエッジ構造であらわされるメッシュを提供します。
//! @author Shuto Shikama
//! @date 2018/01/01.
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
 
    //! @brief ハーフエッジ構造であらわされるメッシュです。
    class Surface
    {
    protected:
        // 継承して使用してもらうために protected にしてます。
        Surface() = default;
    public:
        virtual~Surface() = default;
        
        inline std::vector<float>& getVertices(){ return m_vertexData; }
        
        inline const std::vector<Halfedge>& getHalfedges()const{ return m_halfedges; };
        
        inline const std::vector<Face>& getFaces()const{ return m_faces; }
        
        inline const std::vector<Vertex>& getVertices()const{ return m_vertices; }
        
    protected:
        
        //! @brief ハーフエッジ構造を構築します。
        //! @param[in] vertexData x1, y1, z1, x2, y2, z2, x3...
        //! @param[in] indexData fx1, fy1, fz1, fx2, fy2, fz2, ...
        //! @pre vertexData.size() %3 == 0 （3次元ベクトルである）
        //! @pre indexData.size() %3 == 0 (3角ポリゴンで構成されている)
        //! @note このクラスの継承先から呼ばれることを想定しています。
        void makeHalfedge(const std::vector<float>& vertexData, std::vector<std::uint32_t>& indexData);
        
    private:
        
        //! @brief ハーフエッジ。
        std::vector<Halfedge> m_halfedges;
        
        //! @brife ハーフエッジ構造で表現される面。
        std::vector<Face> m_faces;
        
        //! @brief ハーフエッジ構造で使用する頂点。
        std::vector<Vertex> m_vertices;
        
        //! @brief 全頂点を1次元で確保するコンテナ
        //! @note GPU に転送することを想定したコンテナです。
        // x1, y1, z1, x2, y2, z2, ...
        std::vector<float> m_vertexData;
    };

}
}

#endif /* surface_hpp */
