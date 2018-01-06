//
//  cube.cpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/06.
//

#include <gal/prim/cube.hpp>

gal::Cube::Cube()
{
    initialize();
}

void gal::Cube::initialize()
{
    // 上半分
    m_vertices.push_back(-0.5f);
    m_vertices.push_back(-0.5f);
    m_vertices.push_back(0.5f);
    
    m_vertices.push_back(0.5f);
    m_vertices.push_back(-0.5f);
    m_vertices.push_back(0.5f);

    m_vertices.push_back(0.5f);
    m_vertices.push_back(0.5f);
    m_vertices.push_back(0.5f);

    m_vertices.push_back(-0.5f);
    m_vertices.push_back(0.5f);
    m_vertices.push_back(0.5f);
    
    // 下半分
    m_vertices.push_back(-0.5f);
    m_vertices.push_back(-0.5f);
    m_vertices.push_back(-0.5f);
    
    m_vertices.push_back(0.5f);
    m_vertices.push_back(-0.5f);
    m_vertices.push_back(-0.5f);

    m_vertices.push_back(0.5f);
    m_vertices.push_back(0.5f);
    m_vertices.push_back(-0.5f);

    m_vertices.push_back(-0.5f);
    m_vertices.push_back(0.5f);
    m_vertices.push_back(-0.5f);
    

    m_indices.push_back(0);
    m_indices.push_back(4);
    m_indices.push_back(5);
    m_indices.push_back(0);
    m_indices.push_back(5);
    m_indices.push_back(1);
    
    m_indices.push_back(1);
    m_indices.push_back(5);
    m_indices.push_back(6);
    m_indices.push_back(1);
    m_indices.push_back(6);
    m_indices.push_back(2);

    m_indices.push_back(2);
    m_indices.push_back(6);
    m_indices.push_back(7);
    m_indices.push_back(2);
    m_indices.push_back(7);
    m_indices.push_back(3);

    m_indices.push_back(3);
    m_indices.push_back(7);
    m_indices.push_back(4);
    m_indices.push_back(3);
    m_indices.push_back(4);
    m_indices.push_back(0);

    m_indices.push_back(3);
    m_indices.push_back(0);
    m_indices.push_back(1);
    m_indices.push_back(3);
    m_indices.push_back(1);
    m_indices.push_back(2);

    m_indices.push_back(4);
    m_indices.push_back(7);
    m_indices.push_back(5);
    m_indices.push_back(5);
    m_indices.push_back(7);
    m_indices.push_back(6);
}
