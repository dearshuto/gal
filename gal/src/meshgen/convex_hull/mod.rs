use std::ops::{Add, Div, Mul, Sub};

use kgal::{distance_2, ILine, ISqrt, IVectorAccessorX, IVectorAccessorY};

struct Line<T, TPoint>
where
    T: Clone,
    TPoint: IVectorAccessorX<T> + IVectorAccessorY<T>,
{
    a: T,
    b: T,
    c: T,
    begin: TPoint,
    end: TPoint,
    _marker: std::marker::PhantomData<T>,
}

impl<T, TPoint> Line<T, TPoint>
where
    T: Clone + Sub<T, Output = T> + Mul<T, Output = T> + PartialOrd + num::Zero,
    TPoint: IVectorAccessorX<T> + IVectorAccessorY<T>,
{
    pub fn new(begin: TPoint, end: TPoint) -> Self
    where
        TPoint: IVectorAccessorX<T> + IVectorAccessorY<T>,
    {
        let gradient_top = end.y() - begin.y();
        let gradient_bottom = end.x() - begin.x();

        // ax + by + c = 0
        let a = T::zero() - gradient_top;
        let b = gradient_bottom;
        let c = T::zero() - (a.clone() * begin.x() + b.clone() * begin.y());
        Self {
            a,
            b,
            c,
            begin,
            end,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn is_above(&self, p: &TPoint) -> bool {
        let vl = (self.end.x() - self.begin.x(), self.end.y() - self.begin.y());
        let vp = (p.x() - self.begin.x(), p.y() - self.begin.y());

        // yl x yp
        let cross = vl.0 * vp.1 - vl.1 * vp.0;

        T::zero() < cross
    }
}

impl<T, TPoint> ILine<T> for Line<T, TPoint>
where
    T: Clone + Sub<T, Output = T> + Mul<T, Output = T>,
    TPoint: IVectorAccessorX<T> + IVectorAccessorY<T>,
{
    fn a(&self) -> T {
        self.a.clone()
    }

    fn b(&self) -> T {
        self.b.clone()
    }

    fn c(&self) -> T {
        self.c.clone()
    }
}

struct Chunk<T, TPoint>
where
    T: Clone + Sub<T, Output = T> + Mul<T, Output = T>,
    TPoint: IVectorAccessorX<T> + IVectorAccessorY<T>,
{
    pub line: Line<T, TPoint>,
    pub points: Vec<TPoint>,
}

pub fn convex_hull_2<T, TIterator, TPoint>(mut points: TIterator) -> Result<Vec<TPoint>, ()>
where
    T: Clone
        + ISqrt<T>
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Div<T, Output = T>
        + Mul<T, Output = T>
        + num::Zero
        + PartialOrd,
    TIterator: Iterator<Item = TPoint> + Clone,
    TPoint: IVectorAccessorX<T> + IVectorAccessorY<T> + Copy,
{
    let mut separeted_points = Vec::default();
    let mut edge_points = [points.next().unwrap(), points.next().unwrap()];
    for point in points {
        if point.x() < edge_points[0].x() {
            let mut point = point;
            std::mem::swap(&mut point, &mut edge_points[0]);
            separeted_points.push(point);
        } else if edge_points[1].x() < point.x() {
            let mut point = point;
            std::mem::swap(&mut point, &mut edge_points[1]);
            separeted_points.push(point);
        } else {
            separeted_points.push(point);
        }
    }

    let line = Line::new(edge_points[0], edge_points[1]);

    // 最初に選んだ両端点は必ず凸包に含まれる
    let mut results = Vec::from(edge_points);
    let mut chunks = vec![Chunk {
        line,
        points: separeted_points,
    }];
    while let Some(chunk) = chunks.pop() {
        // 頂点がなかった
        if chunk.points.is_empty() {
            continue;
        }

        // 頂点がひとつしかないので凸包更新が確実
        if chunk.points.len() == 1 {
            results.push(chunk.points[0]);
            continue;
        }

        // 最も遠い点を探す
        let line = &chunk.line;
        let mut furthest_index = 0;
        let mut furthest_distance = T::zero();
        for (index, point) in chunk.points.iter().enumerate() {
            let distance = distance_2(point, line);

            if furthest_distance < distance {
                // より遠い点が見つかったので更新
                furthest_distance = distance;
                furthest_index = index;
            } else {
                continue;
            }
        }

        // 最も遠い点を追加
        let furthest_point = chunk.points[furthest_index];
        results.push(furthest_point);

        // 最も遠い点がなす新たな領域を区切る線
        let line0 = Line::new(chunk.line.begin, furthest_point);
        let line1 = Line::new(furthest_point, chunk.line.end);

        // 点群を新たな領域に割り当てる
        let mut points0 = Vec::default();
        let mut points1 = Vec::default();
        for point in chunk.points {
            if line0.is_above(&point) {
                points0.push(point);
            } else if line1.is_above(&point) {
                points1.push(point);
            } else {
                // 凸包内の点は何もしない
                continue;
            }
        }

        // 新たに構築した部分を追加
        chunks.push(Chunk {
            line: line0,
            points: points0,
        });

        chunks.push(Chunk {
            line: line1,
            points: points1,
        });
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector2;

    #[test]
    fn simple() {
        let points = [
            Vector2::new(0.0, 0.0),
            Vector2::new(10.0, 0.0),
            Vector2::new(10.0, 10.0),
            Vector2::new(6.0, 5.0),
            Vector2::new(4.0, 1.0),
        ];

        let result = super::convex_hull_2(points.into_iter()).unwrap();
        assert_eq!(result.len(), 3);
        assert!(result.contains(&Vector2::new(0.0, 0.0)));
        assert!(result.contains(&Vector2::new(10.0, 0.0)));
        assert!(result.contains(&Vector2::new(10.0, 10.0)));
    }
}
