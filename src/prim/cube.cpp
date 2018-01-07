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
    std::vector<float> vertices;
    // 上半分
    vertices.push_back(-0.5f);
    vertices.push_back(-0.5f);
    vertices.push_back(0.5f);
    
    vertices.push_back(0.5f);
    vertices.push_back(-0.5f);
    vertices.push_back(0.5f);

    vertices.push_back(0.5f);
    vertices.push_back(0.5f);
    vertices.push_back(0.5f);

    vertices.push_back(-0.5f);
    vertices.push_back(0.5f);
    vertices.push_back(0.5f);
    
    // 下半分
    vertices.push_back(-0.5f);
    vertices.push_back(-0.5f);
    vertices.push_back(-0.5f);
    
    vertices.push_back(0.5f);
    vertices.push_back(-0.5f);
    vertices.push_back(-0.5f);

    vertices.push_back(0.5f);
    vertices.push_back(0.5f);
    vertices.push_back(-0.5f);

    vertices.push_back(-0.5f);
    vertices.push_back(0.5f);
    vertices.push_back(-0.5f);
    
    
    std::vector<std::uint32_t> indices(0);
    indices.push_back(0);
    indices.push_back(4);
    indices.push_back(5);
    indices.push_back(0);
    indices.push_back(5);
    indices.push_back(1);
    
    indices.push_back(1);
    indices.push_back(5);
    indices.push_back(6);
    indices.push_back(1);
    indices.push_back(6);
    indices.push_back(2);

    indices.push_back(2);
    indices.push_back(6);
    indices.push_back(7);
    indices.push_back(2);
    indices.push_back(7);
    indices.push_back(3);

    indices.push_back(3);
    indices.push_back(7);
    indices.push_back(4);
    indices.push_back(3);
    indices.push_back(4);
    indices.push_back(0);

    indices.push_back(3);
    indices.push_back(0);
    indices.push_back(1);
    indices.push_back(3);
    indices.push_back(1);
    indices.push_back(2);

    indices.push_back(4);
    indices.push_back(7);
    indices.push_back(5);
    indices.push_back(5);
    indices.push_back(7);
    indices.push_back(6);
    
    makeHalfedge(vertices, indices);
}
