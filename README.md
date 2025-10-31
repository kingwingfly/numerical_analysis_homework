# 说明

采用Rust语言。画图使用`plotters`，数值分析采用`peroxide`。

# 实验2.1 多项式插值振荡现象

## 分析

目标函数有三个：
$$
f(x) = \frac{1}{1+25 x^2} \\
h(x) = \frac{x}{1+x^4} \\
g(x) = \arctan{x}
$$
即三个 `Fn(f64) -> f64`，

连续，并且在不同区间上 n 等分，自然，需要的函数签名大概为

```rust
fn lagrange<F, I>(f: F, start: f64, end: f64, ns: I) -> Result<()>
where
    F: Fn(f64) -> f64,
    I: IntoIterator<Item = usize>,
{
    // 求插值多项式，求点
    // 求最值，初始化网格
    // 画出原函数图像、拉格朗日多项式图像
}
```

规定了起始点、结束点、以及一个迭代器，yield等分次数。

具体实现见源码.

## 结果

1.
$$
f(x) = \frac{1}{1+25 x^2}
$$

![result_frac{1}{1+x^2}](assets/frac{1}{1+25x^2}.svg)

2.
$$
h(x) = \frac{x}{1+x^4}
$$

![result_frac{1}{1+x^2}](assets/frac{x}{1+x^4}.svg)

3.
$$
g(x) = \arctan{x}
$$

![result_frac{1}{1+x^2}](assets/arctan{x}.svg)



**分析**： TODO



## 采用切比雪夫点

`peroxide`有切比雪夫点的实现：peroxide::numerical::interp::chebyshev_nodes

1.
$$
f(x) = \frac{1}{1+25 x^2}
$$

<div style="display: flex; justify-content: space-between; width: 100%;">
    <img src="assets/frac{1}{1+25x^2}.svg" style="flex: 1; max-width: 50%; height: auto;"/>
    <img src="assets/frac{1}{1+25x^2}(chebyshev_nodes).svg" style="flex: 1; max-width: 50%; height: auto;"/>
<div/>

2.
$$
h(x) = \frac{x}{1+x^4}
$$

<div style="display: flex; justify-content: space-between; width: 100%;">
    <img src="assets/frac{x}{1+x^4}.svg" style="flex: 1; max-width: 50%; height: auto;"/>
    <img src="assets/frac{x}{1+x^4}(chebyshev_nodes).svg" style="flex: 1; max-width: 50%; height: auto;"/>
<div/>

3.
$$
g(x) = \arctan{x}
$$

<div style="display: flex; justify-content: space-between; width: 100%;">
    <img src="assets/arctan{x}.svg" style="flex: 1; max-width: 50%; height: auto;"/>
    <img src="assets/arctan{x}(chebyshev_nodes).svg" style="flex: 1; max-width: 50%; height: auto;"/>
<div/>



**分析**： TODO

# 实验2.2 样条插值的收敛性


1.
$$
f(x) = \frac{1}{1+25 x^2}
$$

<div style="display: flex; justify-content: space-between; width: 100%;">
    <img src="assets/frac{1}{1+25x^2}.svg" style="flex: 1; max-width: 33%; height: auto;"/>
    <img src="assets/frac{1}{1+25x^2}(chebyshev_nodes).svg" style="flex: 1; max-width: 33%; height: auto;"/>
    <img src="assets/frac{1}{1+25x^2}(spline).svg" style="flex: 1; max-width: 33%; height: auto;"/>
<div/>

2.
$$
h(x) = \frac{x}{1+x^4}
$$

<div style="display: flex; justify-content: space-between; width: 100%;">
    <img src="assets/frac{x}{1+x^4}.svg" style="flex: 1; max-width: 33%; height: auto;"/>
    <img src="assets/frac{x}{1+x^4}(chebyshev_nodes).svg" style="flex: 1; max-width: 33%; height: auto;"/>
    <img src="assets/frac{x}{1+x^4}(spline).svg" style="flex: 1; max-width: 33%; height: auto;"/>
<div/>

3.
$$
g(x) = \arctan{x}
$$

<div style="display: flex; justify-content: space-between; width: 100%;">
    <img src="assets/arctan{x}.svg" style="flex: 1; max-width: 33%; height: auto;"/>
    <img src="assets/arctan{x}(chebyshev_nodes).svg" style="flex: 1; max-width: 33%; height: auto;"/>
    <img src="assets/arctan{x}(spline).svg" style="flex: 1; max-width: 33%; height: auto;"/>
<div/>

**分析**：TODO
