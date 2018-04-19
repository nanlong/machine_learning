// 曼哈顿距离
pub fn manhattan(mut x: &[f32], mut y: &[f32]) -> f32 {
    let x_len = x.len();
    let y_len = y.len();
    let end = x_len.min(y_len);
    let mut distance = 0.0;

    // 长度不同时进行处理
    if x_len != y_len {
        x = &x[..end];
        y = &y[..end];
    }

    // 计算结果
    for i in 0..end {
        distance = distance + (x[i] - y[i]).abs();
    }

    distance
}

// 欧氏距离
pub fn euclidean(mut x: &[f32], mut y: &[f32]) -> f32 {
    let x_len = x.len();
    let y_len = y.len();
    let end = x_len.min(y_len);
    let mut euclidean = 0.0;

    // 长度不同时进行处理
    if x_len != y_len {
        x = &x[..end];
        y = &y[..end];
    }

    // 计算结果
    for i in 0..end {
        euclidean = euclidean + (x[i] - y[i]).powf(2.0);
    }

    euclidean.sqrt()
}

// 明氏距离
// r 为 1 = 曼哈顿距离
// r 为 2 = 欧氏距离
// r 为 ∞ = 上确界距离
pub fn minkowski(mut x: &[f32], mut y: &[f32], r: f32) -> f32 {
    let x_len = x.len();
    let y_len = y.len();
    let end = x_len.min(y_len);
    let mut distance = 0.0;

    // 长度不同时进行处理
    if x_len != y_len {
        x = &x[..end];
        y = &y[..end];
    }

    // 计算结果
    for i in 0..end {
        distance = distance + (x[i] - y[i]).abs().powf(r);
    }

    if distance > 0.0 {
        distance.powf(1.0 / r)
    }
    else {
        distance
    }
}


#[test]
fn test() {
  let x = [2.0, 4.0];
  let y = [5.0, 5.0];

  let distance = manhattan(&x, &y);
  assert_eq!(distance, 4.0);

  let distance = euclidean(&x, &y);
  assert_eq!(distance, 3.1622777);

  let distance = minkowski(&x, &y, 1.0);
  assert_eq!(distance, 4.0);

  let distance = minkowski(&x, &y, 2.0);
  assert_eq!(distance, 3.1622777);

  let distance = minkowski(&x, &y, 3.0);
  assert_eq!(distance, 3.0365891);
}