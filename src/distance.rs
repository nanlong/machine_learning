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
  let mut distance = 0.0;

  // 长度不同时进行处理
  if x_len != y_len {
    x = &x[..end];
    y = &y[..end];
  }

  // 计算结果
  for i in 0..end {
    distance = distance + (x[i] - y[i]).powf(2.0);
  }

  distance.sqrt()
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


// 皮尔逊相关系数
pub fn pearson(mut x: &[f32], mut y: &[f32]) -> f32 {
  let mut sum_xy = 0.0;
  let mut sum_x = 0.0;
  let mut sum_y = 0.0;
  let mut sum_x2 = 0.0;
  let mut sum_y2 = 0.0;
  let mut n = 0.0;

  let x_len = x.len();
  let y_len = y.len();
  let end = x_len.min(y_len);

  if x_len != y_len {
    x = &x[..end];
    y = &y[..end];
  }

  for i in 0..end {
    n = n + 1.0;
    sum_xy = sum_xy + x[i] * y[i];
    sum_x = sum_x + x[i];
    sum_y = sum_y + y[i];
    sum_x2 = sum_x2 + x[i].powf(2.0);
    sum_y2 = sum_y2 + y[i].powf(2.0);
  }

  let denominator = (sum_x2 - sum_x.powf(2.0) / n).sqrt() * (sum_y2 - sum_y.powf(2.0) / n).sqrt();

  if denominator > 0.0 {
    (sum_xy - sum_x * sum_y / n) / denominator
  }
  else {
    denominator
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

  let x = [3.5, 2.0, 5.0, 1.5, 2.0];
  let y = [2.0, 3.5, 2.0, 3.5, 3.0];
  let distance = pearson(&x, &y);
  assert_eq!(distance, -0.9040538);

  let x = [2.0, 4.5, 2.5, 2.0];
  let y = [4.0, 4.0, 4.0, 1.0];
  let distance = pearson(&x, &y);
  assert_eq!(distance, 0.42008406);
}