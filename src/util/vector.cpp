//
//  util_vector.cpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/01.
//

#include <gal/util/vector.hpp>

gal::Vector::Vector(const float x, const float y, const float z)
: X(x), Y(y), Z(z)
{
    
}

gal::Vector gal::Vector::operator+(const gal::Vector &other)const
{
    return gal::Vector{
        this->X + other.X,
        this->Y + other.Y,
        this->Z + other.Z
    };
}

gal::Vector gal::Vector::operator-(const gal::Vector &other)const
{
    return gal::Vector{
        this->X - other.X,
        this->Y - other.Y,
        this->Z - other.Z
    };
}

gal::Vector& gal::Vector::operator-=(const gal::Vector &other)
{
    this->X -= other.X;
    this->Y -= other.Y;
    this->Z -= other.Z;
    return *this;
}

gal::Vector gal::Vector::operator/(const float n)const
{
    return gal::Vector{
        this->X / n,
        this->Y / n,
        this->Z / n
    };
}

gal::Vector gal::Vector::operator*(const float n)const
{
    return gal::Vector{
        this->X * n,
        this->Y * n,
        this->Z * n
    };
}

gal::Vector operator*(const float n, const gal::Vector& other)
{
    return other * n;
}
