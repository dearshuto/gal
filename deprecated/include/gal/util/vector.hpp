//
//  util_vector.hpp
//  gal
//
//  Created by Shuto Shikama on 2018/01/01.
//

#ifndef util_vector_hpp
#define util_vector_hpp

namespace gal
{
namespace util
{
    //! @brief 三次元ベクトルです。
    class Vector
    {
    public:
        // デフォルト
        Vector() = default;
        ~Vector() = default;
        
        // コピー
        Vector(const Vector& other) = default;
        Vector& operator=(const Vector& other) = default;
        
        Vector(const float x, const float y, const float z);
        // 足し算
        inline Vector operator+(const Vector& other)const;
        inline Vector& operator+=(const Vector& other)
        {
            this->X += other.X;
            this->Y += other.Y;
            this->Z += other.Z;
            return *this;
        }
        
        // 引き算
        inline Vector operator-(const Vector& other)const;
        inline Vector& operator-=(const Vector& other);
        
        // 割り算
        Vector operator/(const float n)const;
        
        // かけ算
        Vector operator*(const float n)const;
        
        float X{ 0.0f };
        float Y{ 0.0f };
        float Z{ 0.0f };
    };
}
}

gal::util::Vector operator*(const float n, const gal::util::Vector& other);

#endif /* util_vector_hpp */
