//
//  util_vector.cpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/01.
//

#include <gal/util/vector.hpp>

gal::util::Vector::Vector(const float x, const float y, const float z)
: X(x), Y(y), Z(z)
{
    
}

gal::util::Vector gal::util::Vector::operator+(const gal::util::Vector &other)const
{
    return gal::util::Vector{
        this->X + other.X,
        this->Y + other.Y,
        this->Z + other.Z
    };
}

gal::util::Vector gal::util::Vector::operator-(const gal::util::Vector &other)const
{
    return gal::util::Vector{
        this->X - other.X,
        this->Y - other.Y,
        this->Z - other.Z
    };
}

gal::util::Vector& gal::util::Vector::operator-=(const gal::util::Vector &other)
{
    this->X -= other.X;
    this->Y -= other.Y;
    this->Z -= other.Z;
    return *this;
}

gal::util::Vector gal::util::Vector::operator/(const float n)const
{
    return gal::util::Vector{
        this->X / n,
        this->Y / n,
        this->Z / n
    };
}

gal::util::Vector gal::util::Vector::operator*(const float n)const
{
    return gal::util::Vector{
        this->X * n,
        this->Y * n,
        this->Z * n
    };
}

gal::util::Vector operator*(const float n, const gal::util::Vector& other)
{
    return other * n;
}
